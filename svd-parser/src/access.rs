use super::{elementext::ElementExt, Config, Node, Parse, Result, SVDError};

use crate::svd::Access;
impl Parse for Access {
    type Object = Self;
    type Error = anyhow::Error;
    type Config = Config;

    fn parse(tree: &Node, _config: &Self::Config) -> Result<Self> {
        let text = tree.get_text()?;

        match text {
            "read-only" => Ok(Access::ReadOnly),
            "read-write" => Ok(Access::ReadWrite),
            "read-writeOnce" => Ok(Access::ReadWriteOnce),
            "write-only" => Ok(Access::WriteOnly),
            "writeOnce" => Ok(Access::WriteOnce),
            _ => Err(SVDError::UnknownAccessType(text.into())
                .at(tree.id())
                .into()),
        }
    }
}
