use xmltree::Element;

use crate::types::Parse;

#[cfg(feature = "unproven")]
use crate::encode::Encode;

use crate::error::*;
use crate::svd::{cluster::Cluster, register::Register};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum RegisterCluster {
    Register(Register),
    Cluster(Cluster),
}

impl From<Register> for RegisterCluster {
    fn from(reg: Register) -> Self {
        RegisterCluster::Register(reg)
    }
}

impl From<Cluster> for RegisterCluster {
    fn from(cluser: Cluster) -> Self {
        RegisterCluster::Cluster(cluser)
    }
}

impl Parse for RegisterCluster {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        if tree.name == "register" {
            Ok(RegisterCluster::Register(Register::parse(tree)?))
        } else if tree.name == "cluster" {
            Ok(RegisterCluster::Cluster(Cluster::parse(tree)?))
        } else {
            Err(RegisterClusterError::Invalid(tree.clone(), tree.name.clone()).into())
        }
    }
}

#[cfg(feature = "unproven")]
impl Encode for RegisterCluster {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        match self {
            RegisterCluster::Register(r) => r.encode(),
            RegisterCluster::Cluster(c) => c.encode(),
        }
    }
}

// TODO: test RegisterCluster encode and decode
