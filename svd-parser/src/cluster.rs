use super::*;
use crate::svd::Cluster;

impl Parse for Cluster {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, config: &Self::Config) -> Result<Self, Self::Error> {
        parse_array("cluster", tree, config)
    }
}
