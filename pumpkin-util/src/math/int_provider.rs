use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use serde::Deserialize;
use syn::LitInt;

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum NormalIntProvider {
    #[serde(rename = "minecraft:uniform")]
    Uniform(UniformIntProvider),
    #[serde(rename = "minecraft:weighted_list")]
    WeightedList(WeightedListIntProvider),
    #[serde(rename = "minecraft:clamped")]
    Clamped(ClampedIntProvider),
    #[serde(rename = "minecraft:biased_to_bottom")]
    BiasedToBottom(BiasedToBottomIntProvider), // TODO: Add more...
}

impl ToTokens for NormalIntProvider {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            NormalIntProvider::Uniform(uniform) => {
                tokens.extend(quote! {
                    NormalIntProvider::Uniform(#uniform)
                });
            }
            NormalIntProvider::WeightedList(_) => todo!(),
            NormalIntProvider::Clamped(_) => todo!(),
            NormalIntProvider::BiasedToBottom(biased_to_bottom_int_provider) => todo!(),
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum IntProvider {
    Object(NormalIntProvider),
    Constant(i32),
}

impl ToTokens for IntProvider {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            IntProvider::Object(int_provider) => {
                tokens.extend(quote! {
                    IntProvider::Object(#int_provider)
                });
            }
            IntProvider::Constant(i) => tokens.extend(quote! {
                IntProvider::Constant(#i)
            }),
        }
    }
}

impl IntProvider {
    pub fn get_min(&self) -> i32 {
        match self {
            IntProvider::Object(int_provider) => match int_provider {
                NormalIntProvider::Uniform(uniform) => uniform.get_min(),
                NormalIntProvider::WeightedList(provider) => provider.get_min(),
                NormalIntProvider::Clamped(provider) => provider.get_min(),
                NormalIntProvider::BiasedToBottom(provider) => provider.get_min(),
            },
            IntProvider::Constant(i) => *i,
        }
    }

    pub fn get(&self) -> i32 {
        match self {
            IntProvider::Object(int_provider) => match int_provider {
                NormalIntProvider::Uniform(uniform) => uniform.get(),
                NormalIntProvider::WeightedList(provider) => provider.get(),
                NormalIntProvider::Clamped(provider) => provider.get(),
                NormalIntProvider::BiasedToBottom(provider) => provider.get(),
            },
            IntProvider::Constant(i) => *i,
        }
    }

    pub fn get_max(&self) -> i32 {
        match self {
            IntProvider::Object(int_provider) => match int_provider {
                NormalIntProvider::Uniform(uniform) => uniform.get_max(),
                NormalIntProvider::WeightedList(provider) => provider.get_max(),
                NormalIntProvider::Clamped(provider) => provider.get_max(),
                NormalIntProvider::BiasedToBottom(provider) => provider.get_max(),
            },
            IntProvider::Constant(i) => *i,
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct BiasedToBottomIntProvider {
    min_inclusive: i32,
    max_inclusive: i32,
}

impl BiasedToBottomIntProvider {
    pub fn get_min(&self) -> i32 {
        self.min_inclusive
    }
    pub fn get(&self) -> i32 {
        // TODO: use random
        self.min_inclusive
    }
    pub fn get_max(&self) -> i32 {
        self.max_inclusive
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct ClampedIntProvider {
    source: Box<IntProvider>,
    min_inclusive: i32,
    max_inclusive: i32,
}

impl ClampedIntProvider {
    pub fn get_min(&self) -> i32 {
        self.min_inclusive.max(self.source.get_min())
    }
    pub fn get(&self) -> i32 {
        self.source
            .get()
            .clamp(self.min_inclusive, self.max_inclusive)
    }
    pub fn get_max(&self) -> i32 {
        self.max_inclusive.min(self.source.get_max())
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct WeightedListIntProvider {
    // TODO
}

impl WeightedListIntProvider {
    // TODO
    pub fn get_min(&self) -> i32 {
        0
    }
    pub fn get(&self) -> i32 {
        0
    }
    pub fn get_max(&self) -> i32 {
        0
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct UniformIntProvider {
    pub min_inclusive: i32,
    pub max_inclusive: i32,
}

impl ToTokens for UniformIntProvider {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let min_inclusive = LitInt::new(&self.min_inclusive.to_string(), Span::call_site());
        let max_inclusive = LitInt::new(&self.max_inclusive.to_string(), Span::call_site());

        tokens.extend(quote! {
            UniformIntProvider { min_inclusive: #min_inclusive, max_inclusive: #max_inclusive }
        });
    }
}

impl UniformIntProvider {
    pub fn get_min(&self) -> i32 {
        self.min_inclusive
    }
    pub fn get(&self) -> i32 {
        rand::random_range(self.min_inclusive..self.max_inclusive)
    }
    pub fn get_max(&self) -> i32 {
        self.max_inclusive
    }
}
