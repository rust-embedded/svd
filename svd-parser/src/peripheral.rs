use super::*;
use crate::svd::Peripheral;

impl Parse for Peripheral {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, config: &Self::Config) -> Result<Self, Self::Error> {
        parse_array("peripheral", tree, config)
    }
}
