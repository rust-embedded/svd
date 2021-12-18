use super::*;
use crate::svd::Field;

impl Parse for Field {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, config: &Self::Config) -> Result<Self, Self::Error> {
        parse_array("field", tree, config)
    }
}
