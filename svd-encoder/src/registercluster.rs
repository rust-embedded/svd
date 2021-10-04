use super::{Element, Encode, EncodeError};

use crate::svd::RegisterCluster;

impl Encode for RegisterCluster {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        match self {
            RegisterCluster::Register(r) => r.encode(),
            RegisterCluster::Cluster(c) => c.encode(),
        }
    }
}
