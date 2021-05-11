use super::{new_element, Element, Encode, EncodeError};

use crate::elementext::ElementExt;
use crate::svd::Cluster;

impl Encode for Cluster {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
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
