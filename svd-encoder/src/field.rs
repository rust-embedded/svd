use super::{Element, ElementMerge, Encode, EncodeError};

use crate::svd::Field;

impl Encode for Field {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        match self {
            Field::Single(info) => info.encode(),
            Field::Array(info, array_info) => {
                let mut base = Element::new("field");
                base.merge(&array_info.encode()?);
                base.merge(&info.encode()?);
                Ok(base)
            }
        }
    }
}
