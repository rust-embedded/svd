use super::{new_element, Element, Encode};

use crate::error::*;
use crate::svd::EnumeratedValues;

impl Encode for EnumeratedValues {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        let mut base = new_element("enumeratedValues", None);

        if let Some(d) = &self.name {
            base.children.push(new_element("name", Some((*d).clone())));
        };

        if let Some(v) = &self.usage {
            base.children.push(v.encode()?);
        };

        if let Some(v) = &self.derived_from {
            base.attributes
                .insert(String::from("derivedFrom"), (*v).clone());
        }

        for v in &self.values {
            base.children.push(v.encode()?);
        }

        Ok(base)
    }
}
