use super::Element;

use crate::error::*;
use crate::parse;
use crate::svd::{Access, RegisterProperties};
use crate::types::Parse;

impl Parse for RegisterProperties {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        let p = RegisterProperties {
            size: parse::optional::<u32>("size", tree)?,
            access: parse::optional::<Access>("access", tree)?,
            reset_value: parse::optional::<u64>("resetValue", tree)?,
            reset_mask: parse::optional::<u64>("resetMask", tree)?,
        };
        check_reset_value(p.size, p.reset_value, p.reset_mask)?;
        Ok(p)
    }
}
