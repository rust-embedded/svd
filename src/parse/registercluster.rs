use super::{Element, Parse};

use crate::error::*;
use crate::svd::{Cluster, Register};

use crate::svd::RegisterCluster;
impl Parse for RegisterCluster {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        if tree.name == "register" {
            Ok(RegisterCluster::Register(Register::parse(tree)?))
        } else if tree.name == "cluster" {
            Ok(RegisterCluster::Cluster(Cluster::parse(tree)?))
        } else {
            Err(SVDError::InvalidRegisterCluster(tree.clone(), tree.name.clone()).into())
        }
    }
}
