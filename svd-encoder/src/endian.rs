use super::{new_element, Element, Encode, EncodeError};

use crate::svd::Endian;

impl Encode for Endian {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        let text = match *self {
            Endian::Little => String::from("little"),
            Endian::Big => String::from("big"),
            Endian::Selectable => String::from("selectable"),
            Endian::Other => String::from("other"),
        };

        Ok(new_element("endian", Some(text)))
    }
}
