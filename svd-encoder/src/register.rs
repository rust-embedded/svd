use super::{Element, ElementMerge, Encode, EncodeError};

use crate::svd::Register;

impl Encode for Register {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        let info = self.info.encode();
        match &self.dim {
            None => info,
            Some(array_info) => {
                let mut base = Element::new("register");
                base.merge(&array_info.encode()?);
                base.merge(&info?);
                Ok(base)
            }
        }
    }
}
