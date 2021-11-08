use super::*;
use crate::svd::{Access, Protection, RegisterProperties};

impl Parse for RegisterProperties {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, config: &Self::Config) -> Result<Self, Self::Error> {
        RegisterProperties::new()
            .size(optional::<u32>("size", tree, &())?)
            .access(optional::<Access>("access", tree, config)?)
            .protection(optional::<Protection>("protection", tree, config)?)
            .reset_value(optional::<u64>("resetValue", tree, &())?)
            .reset_mask(optional::<u64>("resetMask", tree, &())?)
            .build(config.validate_level)
            .map_err(|e| SVDError::from(e).at(tree.id()))
    }
}
