use super::{new_node, Element, Encode, EncodeError};

use crate::svd::EnumeratedValue;

impl Encode for EnumeratedValue {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        let mut base = Element::new("enumeratedValue");
        base.children.push(new_node("name", self.name.clone()));

        if let Some(d) = &self.description {
            base.children.push(new_node("description", d.clone()));
        };

        if let Some(v) = &self.value {
            base.children.push(new_node("value", format!("{}", v)));
        };

        if let Some(v) = &self.is_default {
            base.children.push(new_node("isDefault", format!("{}", v)));
        };

        Ok(base)
    }
}
