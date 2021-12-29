use super::*;
use crate::svd::Register;

impl Parse for Register {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, config: &Self::Config) -> Result<Self, Self::Error> {
        parse_array("register", tree, config)
    }
}
