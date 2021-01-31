use minidom::Element;

use crate::types::Parse;

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
        match tree.name() {
            "register" => Ok(RegisterCluster::Register(Register::parse(tree)?)),
            "cluster" => Ok(RegisterCluster::Cluster(Cluster::parse(tree)?)),
            _ => {
                Err(SVDError::InvalidRegisterCluster(tree.clone(), tree.name().to_string()).into())
            }
        }
    }
}

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
