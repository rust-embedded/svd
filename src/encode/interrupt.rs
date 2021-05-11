use super::{new_element, Element, Encode};

use crate::error::*;
use crate::svd::Interrupt;

impl Encode for Interrupt {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
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
