use super::{new_node, Element, Encode, EncodeError};

use crate::svd::Interrupt;

impl Encode for Interrupt {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        let mut children = vec![new_node("name", self.name.clone())];
        if let Some(d) = self.description.clone() {
            children.push(new_node("description", d));
        }
        children.push(new_node("value", format!("{}", self.value)));
        let mut elem = Element::new("interrupt");
        elem.children = children;
        Ok(elem)
    }
}
