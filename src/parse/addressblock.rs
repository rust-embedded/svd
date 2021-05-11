use super::Element;

use crate::elementext::ElementExt;
use crate::error::*;
use crate::svd::AddressBlock;
use crate::types::Parse;

impl Parse for AddressBlock {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        Ok(Self {
            offset: tree.get_child_u32("offset")?,
            size: tree.get_child_u32("size")?,
            usage: tree.get_child_text("usage")?,
        })
    }
}
