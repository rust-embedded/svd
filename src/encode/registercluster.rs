use xmltree::Element;

use crate::encode::Encode;
use crate::error::*;

use crate::svd::RegisterCluster;
impl Encode for RegisterCluster {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        match self {
            RegisterCluster::Register(r) => r.encode(),
            RegisterCluster::Cluster(c) => c.encode(),
        }
    }
}
