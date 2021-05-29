use super::*;

use crate::svd::Access;
impl Parse for Access {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, _config: &Self::Config) -> Result<Self, Self::Error> {
        let text = tree.get_text()?;

        match text {
            "read-only" => Ok(Access::ReadOnly),
            "read-write" => Ok(Access::ReadWrite),
            "read-writeOnce" => Ok(Access::ReadWriteOnce),
            "write-only" => Ok(Access::WriteOnly),
            "writeOnce" => Ok(Access::WriteOnce),
            _ => Err(SVDError::UnknownAccessType(text.into()).at(tree.id())),
        }
    }
}
