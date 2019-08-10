use xmltree::Element;

use svd::defaults::Defaults;
use parse::ParseDefaults;

#[cfg(feature = "unproven")]
use encode::Encode;

use error::{SVDError, SVDErrorKind};
use svd::cluster::Cluster;
use svd::register::Register;

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

impl ParseDefaults for RegisterCluster {
    type Object = RegisterCluster;
    type Error = SVDError;
    fn parse_defaults(tree: &Element, defaults: Defaults) -> Result<RegisterCluster, SVDError> {
        if tree.name == "register" {
            Ok(RegisterCluster::Register(Register::parse_defaults(
                tree,
                defaults,
            )?))
        } else if tree.name == "cluster" {
            Ok(RegisterCluster::Cluster(Cluster::parse_defaults(
                tree,
                defaults,
            )?))
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
