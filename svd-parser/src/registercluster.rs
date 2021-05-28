use super::{Node, Parse, Result, SVDError};
use crate::svd::{Cluster, Register};

use crate::svd::RegisterCluster;
impl Parse for RegisterCluster {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Node) -> Result<Self> {
        match tree.tag_name().name() {
            "register" => Register::parse(tree).map(RegisterCluster::Register),
            "cluster" => Cluster::parse(tree).map(RegisterCluster::Cluster),
            _ => Err(
                SVDError::InvalidRegisterCluster(tree.tag_name().name().to_string())
                    .at(tree.id())
                    .into(),
            ),
        }
    }
}
