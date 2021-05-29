use super::{elementext::ElementExt, Config, Node, Parse, Result, SVDError};
use crate::svd::Usage;

impl Parse for Usage {
    type Object = Self;
    type Error = anyhow::Error;
    type Config = Config;

    fn parse(tree: &Node, _config: &Self::Config) -> Result<Self> {
        let text = tree.get_text()?;

        match text {
            "read" => Ok(Usage::Read),
            "write" => Ok(Usage::Write),
            "read-write" => Ok(Usage::ReadWrite),
            _ => Err(SVDError::UnknownUsageVariant.at(tree.id()).into()),
        }
    }
}
