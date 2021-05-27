use super::{new_element, Element, Encode, EncodeError};
use crate::svd::Access;

impl Encode for Access {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        let text = match *self {
            Access::ReadOnly => String::from("read-only"),
            Access::ReadWrite => String::from("read-write"),
            Access::ReadWriteOnce => String::from("read-writeOnce"),
            Access::WriteOnly => String::from("write-only"),
            Access::WriteOnce => String::from("writeOnce"),
        };

        Ok(new_element("access", Some(text)))
    }
}
