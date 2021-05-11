use super::Element;

use crate::elementext::ElementExt;
use crate::error::*;
use crate::types::Parse;

use crate::svd::Access;
impl Parse for Access {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        let text = tree.get_text()?;

        match &text[..] {
            "read-only" => Ok(Access::ReadOnly),
            "read-write" => Ok(Access::ReadWrite),
            "read-writeOnce" => Ok(Access::ReadWriteOnce),
            "write-only" => Ok(Access::WriteOnly),
            "writeOnce" => Ok(Access::WriteOnce),
            _ => Err(SVDError::UnknownAccessType(tree.clone(), text).into()),
        }
    }
}
