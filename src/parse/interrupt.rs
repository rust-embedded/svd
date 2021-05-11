use super::{elementext::ElementExt, Element, Parse};

use crate::error::*;
use crate::svd::Interrupt;

impl Interrupt {
    fn _parse(tree: &Element, name: String) -> Result<Self> {
        Ok(Self {
            name,
            description: tree.get_child_text_opt("description")?,
            value: tree.get_child_u32("value")?,
        })
    }
}

impl Parse for Interrupt {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        if tree.name != "interrupt" {
            return Err(SVDError::NotExpectedTag(tree.clone(), "interrupt".to_string()).into());
        }
        let name = tree.get_child_text("name")?;
        Self::_parse(tree, name.clone()).with_context(|| format!("In interrupt `{}`", name))
    }
}
