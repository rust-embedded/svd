use super::{elementext::ElementExt, Element, Parse, Result};
use crate::svd::AddressBlock;

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
