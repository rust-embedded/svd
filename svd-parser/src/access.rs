use super::{elementext::ElementExt, Node, Parse, Result, SVDError};

use crate::svd::Access;
impl Parse for Access {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Node) -> Result<Self> {
        let text = tree.get_text()?;

        match &text[..] {
            "read-only" => Ok(Access::ReadOnly),
            "read-write" => Ok(Access::ReadWrite),
            "read-writeOnce" => Ok(Access::ReadWriteOnce),
            "write-only" => Ok(Access::WriteOnly),
            "writeOnce" => Ok(Access::WriteOnce),
            _ => Err(SVDError::UnknownAccessType(tree.id(), text).into()),
        }
    }
}
