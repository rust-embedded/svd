use super::{new_element, Element, Encode};

use crate::error::*;
use crate::svd::FieldInfo;

impl Encode for FieldInfo {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        let mut elem = new_element("field", None);
        elem.children
            .push(new_element("name", Some(self.name.clone())));

        if let Some(description) = &self.description {
            elem.children
                .push(new_element("description", Some(description.clone())))
        }

        // Add bit range
        elem.children.append(&mut self.bit_range.encode()?);

        if let Some(v) = &self.access {
            elem.children.push(v.encode()?);
        }

        if let Some(v) = &self.modified_write_values {
            elem.children.push(v.encode()?);
        }

        if let Some(v) = &self.write_constraint {
            elem.children.push(v.encode()?);
        }

        let enumerated_values: Result<Vec<Element>> =
            self.enumerated_values.iter().map(|v| v.encode()).collect();
        elem.children.append(&mut enumerated_values?);

        if let Some(v) = &self.derived_from {
            elem.attributes
                .insert(String::from("derivedFrom"), v.to_string());
        }

        Ok(elem)
    }
}
