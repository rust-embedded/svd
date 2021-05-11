use super::{Element, Parse};

use crate::elementext::ElementExt;
use crate::error::*;

use crate::svd::Usage;
impl Parse for Usage {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        let text = tree.get_text()?;

        match &text[..] {
            "read" => Ok(Usage::Read),
            "write" => Ok(Usage::Write),
            "read-write" => Ok(Usage::ReadWrite),
            _ => Err(SVDError::UnknownUsageVariant(tree.clone()).into()),
        }
    }
}
