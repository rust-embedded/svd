use super::{new_element, Element, Encode, EncodeError};

use crate::svd::EnumeratedValue;

impl Encode for EnumeratedValue {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        let mut base = new_element("enumeratedValue", None);
        base.children
            .push(new_element("name", Some(self.name.clone())));

        if let Some(d) = &self.description {
            let s = (*d).clone();
            base.children.push(new_element("description", Some(s)));
        };

        if let Some(v) = &self.value {
            base.children
                .push(new_element("value", Some(format!("{}", v))));
        };

        if let Some(v) = &self.is_default {
            base.children
                .push(new_element("isDefault", Some(format!("{}", v))));
        };

        Ok(base)
    }
}
