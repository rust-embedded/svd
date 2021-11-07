use super::{Element, ElementMerge, Encode, EncodeError};

use crate::svd::Peripheral;

impl Encode for Peripheral {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        let info = self.info.encode();
        match &self.dim {
            None => info,
            Some(array_info) => {
                let mut base = Element::new("peripheral");
                base.merge(&array_info.encode()?);
                base.merge(&info?);
                Ok(base)
            }
        }
    }
}
