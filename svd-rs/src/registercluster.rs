use super::{Cluster, Register};

/// A [cluster](crate::Cluster) or a [register](crate::Register)
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[derive(Clone, Debug, PartialEq)]
pub enum RegisterCluster {
    /// Register
    Register(Register),
    /// Cluster
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
