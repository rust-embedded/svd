use super::{Config, Element, Encode, EncodeError};

use crate::svd::RegisterCluster;

impl Encode for RegisterCluster {
    type Error = EncodeError;

    fn encode_with_config(&self, config: &Config) -> Result<Element, EncodeError> {
        match self {
            RegisterCluster::Register(r) => r.encode_with_config(config),
            RegisterCluster::Cluster(c) => c.encode_with_config(config),
        }
    }
}
