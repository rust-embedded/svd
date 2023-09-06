use super::*;
use crate::svd::Usage;

impl Parse for Usage {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, config: &Self::Config) -> Result<Self, Self::Error> {
        let text = trim_spaces(tree.get_text()?, config).map_err(|e| e.at(tree.id()))?;

        Self::parse_str(text).ok_or_else(|| SVDError::UnknownUsageVariant.at(tree.id()))
    }
}
