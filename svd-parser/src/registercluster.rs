use super::*;
use crate::svd::{Cluster, Register};

use crate::svd::RegisterCluster;
impl Parse for RegisterCluster {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, config: &Self::Config) -> Result<Self, Self::Error> {
        match tree.tag_name().name() {
            "register" => Register::parse(tree, config).map(RegisterCluster::Register),
            "cluster" => Cluster::parse(tree, config).map(RegisterCluster::Cluster),
            _ => Err(
                SVDError::InvalidRegisterCluster(tree.tag_name().name().to_string()).at(tree.id()),
            ),
        }
    }
}
