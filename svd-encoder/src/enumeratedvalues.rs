use super::{new_node, Element, Encode, EncodeError};

use crate::svd::EnumeratedValues;

impl Encode for EnumeratedValues {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        let mut base = Element::new("enumeratedValues");

        if let Some(d) = &self.name {
            base.children.push(new_node("name", (*d).clone()));
        };

        if let Some(v) = &self.usage {
            base.children.push(v.encode_node()?);
        };

        if let Some(v) = &self.derived_from {
            base.attributes
                .insert(String::from("derivedFrom"), (*v).clone());
        }

        for v in &self.values {
            base.children.push(v.encode_node()?);
        }

        Ok(base)
    }
}
