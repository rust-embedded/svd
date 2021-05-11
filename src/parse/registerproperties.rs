use super::{optional, Element, Parse};

use crate::error::*;
use crate::svd::{Access, RegisterProperties};

impl Parse for RegisterProperties {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        let p = RegisterProperties {
            size: optional::<u32>("size", tree)?,
            access: optional::<Access>("access", tree)?,
            reset_value: optional::<u64>("resetValue", tree)?,
            reset_mask: optional::<u64>("resetMask", tree)?,
        };
        check_reset_value(p.size, p.reset_value, p.reset_mask)?;
        Ok(p)
    }
}
