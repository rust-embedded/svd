use super::{new_element, Element, ElementMerge, Encode, EncodeError};

use crate::svd::Register;

impl Encode for Register {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        match self {
            Register::Single(info) => info.encode(),
            Register::Array(info, array_info) => {
                let mut base = new_element("register", None);
                base.merge(&array_info.encode()?);
                base.merge(&info.encode()?);
                Ok(base)
            }
        }
    }
}
