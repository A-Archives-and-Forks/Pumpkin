use pumpkin_data::BlockDirection;
use pumpkin_util::{
    math::{int_provider::IntProvider, position::BlockPos},
    random::RandomGenerator,
};
use serde::Deserialize;

use crate::{
    ProtoChunk,
    generation::{block_predicate::BlockPredicate, block_state_provider::BlockStateProvider},
};

#[derive(Deserialize)]
pub struct BlockColumnFeature {
    layers: Vec<Layer>,
    direction: BlockDirection,
    allowed_placement: BlockPredicate,
    prioritize_tip: bool,
}

#[derive(Deserialize)]
struct Layer {
    height: IntProvider,
    provider: BlockStateProvider,
}

impl BlockColumnFeature {
    pub fn generate(
        &self,
        chunk: &mut ProtoChunk,
        min_y: i8,
        height: u16,
        feature: &str, // This placed feature
        random: &mut RandomGenerator,
        pos: BlockPos,
    ) -> bool {
        let i = self.layers.len();
        let mut is = vec![0; i];
        let mut j = 0;

        for k in 0..i {
            is[k] = (self.layers[k].height).get(random);
            j += is[k];
        }

        if j == 0 {
            return false;
        }

        let mut mutable = pos;
        let mut mutable2 = BlockPos(mutable.0.add(&self.direction.to_offset()));

        let mut l = 0;
        while l < j {
            if !self.allowed_placement.test(chunk, &mutable2) {
                Self::adjust_layer_heights(&mut is, j, l, self.prioritize_tip);
                break;
            }
            mutable2 = BlockPos(mutable2.0.add(&self.direction.to_offset()));
            l += 1;
        }

        for l in 0..i {
            let m = is[l];
            if m == 0 {
                continue;
            }
            let layer = &self.layers[l];
            for _n in 0..m {
                let state = layer.provider.get(random, mutable);
                chunk.set_block_state(&mutable.0, &state);
                mutable = BlockPos(mutable.0.add(&self.direction.to_offset()));
            }
        }

        true
    }

    fn adjust_layer_heights(
        layer_heights: &mut [i32],
        expected_height: i32,
        actual_height: i32,
        prioritize_tip: bool,
    ) {
        let mut i = expected_height - actual_height;
        let j = if prioritize_tip { 1 } else { -1 };
        let k = if prioritize_tip {
            0
        } else {
            layer_heights.len() as i32 - 1
        };
        let l = if prioritize_tip {
            layer_heights.len() as i32
        } else {
            -1
        };

        let mut m = k;
        while m != l && i > 0 {
            let n = layer_heights[m as usize];
            let o = std::cmp::min(n, i);
            layer_heights[m as usize] -= o;
            i -= o;
            m += j;
        }
    }
}
