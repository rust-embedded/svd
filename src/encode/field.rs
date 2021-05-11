use super::{new_element, Element, Encode};

use crate::elementext::ElementExt;
use crate::error::*;
use crate::svd::Field;

impl Encode for Field {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        match self {
            Field::Single(info) => info.encode(),
            Field::Array(info, array_info) => {
                let mut base = new_element("field", None);
                base.merge(&array_info.encode()?);
                base.merge(&info.encode()?);
                Ok(base)
            }
        }
    }
}
