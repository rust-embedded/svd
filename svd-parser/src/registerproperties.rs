use super::{optional, Config, Node, Parse, SVDError, SVDErrorAt};
use crate::svd::{Access, RegisterProperties};

impl Parse for RegisterProperties {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, config: &Self::Config) -> Result<Self, Self::Error> {
        RegisterProperties::builder()
            .size(optional::<u32>("size", tree, &())?)
            .access(optional::<Access>("access", tree, config)?)
            .reset_value(optional::<u64>("resetValue", tree, &())?)
            .reset_mask(optional::<u64>("resetMask", tree, &())?)
            .build(config.validate_level)
            .map_err(|e| SVDError::from(e).at(tree.id()).into())
    }
}
