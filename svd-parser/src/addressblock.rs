use super::*;
use crate::svd::{AddressBlock, AddressBlockUsage, Protection};

impl Parse for AddressBlock {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, config: &Self::Config) -> Result<Self, Self::Error> {
        Self::builder()
            .offset(tree.get_child_u32("offset")?)
            .size(tree.get_child_u32("size")?)
            .usage(AddressBlockUsage::parse(
                &tree.get_child_elem("usage")?,
                config,
            )?)
            .protection(optional::<Protection>("protection", tree, config)?)
            .build(config.validate_level)
            .map_err(|e| SVDError::from(e).at(tree.id()))
    }
}

impl Parse for AddressBlockUsage {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, _config: &Self::Config) -> Result<Self, Self::Error> {
        let text = tree.get_text()?;

        Self::parse_str(text).ok_or_else(|| SVDError::UnknownAddressBlockUsageVariant.at(tree.id()))
    }
}
