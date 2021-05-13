use super::{optional, Element, Parse, Result};
use crate::svd::{Access, RegisterProperties};

impl Parse for RegisterProperties {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        Ok(RegisterProperties::builder()
            .size(optional::<u32>("size", tree)?)
            .access(optional::<Access>("access", tree)?)
            .reset_value(optional::<u64>("resetValue", tree)?)
            .reset_mask(optional::<u64>("resetMask", tree)?)
            .build()?)
    }
}
