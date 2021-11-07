use super::{Element, ElementMerge, Encode, EncodeError};

use crate::svd::Field;

impl Encode for Field {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        let info = self.info.encode();
        match &self.dim {
            None => info,
            Some(array_info) => {
                let mut base = Element::new("field");
                base.merge(&array_info.encode()?);
                base.merge(&info?);
                Ok(base)
            }
        }
    }
}
