use super::{new_element, Element, Encode, EncodeError};

use crate::svd::Usage;

impl Encode for Usage {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        let text = match *self {
            Usage::Read => String::from("read"),
            Usage::Write => String::from("write"),
            Usage::ReadWrite => String::from("read-write"),
        };

        Ok(new_element("usage", Some(text)))
    }
}
