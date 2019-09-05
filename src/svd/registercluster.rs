use xmltree::Element;

use crate::types::Parse;

#[cfg(feature = "unproven")]
use crate::encode::Encode;

use crate::error::{SVDError, SVDErrorKind};
use crate::svd::{
    cluster::Cluster,
    register::Register,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum RegisterCluster {
    Register(Register),
    Cluster(Cluster),
}

impl From<Register> for RegisterCluster {
    fn from(reg: Register) -> RegisterCluster {
        RegisterCluster::Register(reg)
    }
}

impl From<Cluster> for RegisterCluster {
    fn from(cluser: Cluster) -> RegisterCluster {
        RegisterCluster::Cluster(cluser)
    }
}

impl Parse for RegisterCluster {
    type Object = RegisterCluster;
    type Error = SVDError;
    fn parse(tree: &Element) -> Result<RegisterCluster, SVDError> {
        if tree.name == "register" {
            Ok(RegisterCluster::Register(Register::parse(
                tree,
            )?))
        } else if tree.name == "cluster" {
            Ok(RegisterCluster::Cluster(Cluster::parse(tree)?))
        } else {
            Err(SVDError::from(
                SVDErrorKind::InvalidRegisterCluster(
                    tree.clone(),
                    tree.name.clone(),
                ),
            ))
        }
    }
}

#[cfg(feature = "unproven")]
impl Encode for RegisterCluster {
    type Error = SVDError;
    fn encode(&self) -> Result<Element, SVDError> {
        match self {
            RegisterCluster::Register(r) => r.encode(),
            RegisterCluster::Cluster(c) => c.encode(),
        }
    }
}

// TODO: test RegisterCluster encode and decode
