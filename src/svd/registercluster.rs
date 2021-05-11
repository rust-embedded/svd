use crate::svd::{Cluster, Register};

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
