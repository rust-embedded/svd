use xmltree::Element;

use types::Parse;

#[cfg(feature = "unproven")]
use encode::Encode;

use error::{SVDError, SVDErrorKind};
use svd::cluster::Cluster;
use svd::register::Register;

#[cfg(feature = "serde_svd")]
use super::serde::{ Deserialize, Serialize };

#[cfg_attr(feature = "serde_svd", derive(Deserialize, Serialize))]
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
            &RegisterCluster::Register(ref r) => r.encode(),
            &RegisterCluster::Cluster(ref c) => c.encode(),
        }
    }
}

// TODO: test RegisterCluster encode and decode
