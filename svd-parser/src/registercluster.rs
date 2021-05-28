use super::{Node, Parse, Result, SVDError};
use crate::svd::{Cluster, Register};

use crate::svd::RegisterCluster;
impl Parse for RegisterCluster {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Node) -> Result<Self> {
        match tree.tag_name().name() {
            "register" => Ok(RegisterCluster::Register(Register::parse(tree)?)),
            "cluster" => Ok(RegisterCluster::Cluster(Cluster::parse(tree)?)),
            _ => Err(SVDError::InvalidRegisterCluster(
                tree.id(),
                tree.tag_name().name().to_string(),
            )
            .into()),
        }
    }
}
