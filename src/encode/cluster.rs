use super::{new_element, Element, Encode};

use crate::elementext::ElementExt;
use crate::error::*;
use crate::svd::Cluster;

impl Encode for Cluster {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        match self {
            Cluster::Single(i) => i.encode(),
            Cluster::Array(i, a) => {
                let mut e = new_element("cluster", None);
                e.merge(&a.encode()?);
                e.merge(&i.encode()?);
                Ok(e)
            }
        }
    }
}
