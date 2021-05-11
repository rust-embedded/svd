use super::{new_element, Element, Encode, EncodeError};

use crate::svd::Interrupt;

impl Encode for Interrupt {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        let children = vec![
            new_element("name", Some(self.name.clone())),
            new_element("description", self.description.clone()),
            new_element("value", Some(format!("{}", self.value))),
        ];
        let mut elem = new_element("interrupt", None);
        elem.children = children;
        Ok(elem)
    }
}
