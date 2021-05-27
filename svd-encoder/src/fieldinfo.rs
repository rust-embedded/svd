use super::{new_element, Element, Encode, EncodeError};
use crate::bitrange::encode_bitrange;

use crate::svd::FieldInfo;

impl Encode for FieldInfo {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        let mut elem = new_element("field", None);
        elem.children
            .push(new_element("name", Some(self.name.clone())));

        if let Some(description) = &self.description {
            elem.children
                .push(new_element("description", Some(description.clone())))
        }

        // Add bit range
        elem.children.append(&mut encode_bitrange(&self.bit_range)?);

        if let Some(v) = &self.access {
            elem.children.push(v.encode()?);
        }

        if let Some(v) = &self.modified_write_values {
            elem.children.push(v.encode()?);
        }

        if let Some(v) = &self.write_constraint {
            elem.children.push(v.encode()?);
        }

        let enumerated_values: Result<Vec<Element>, EncodeError> =
            self.enumerated_values.iter().map(|v| v.encode()).collect();
        elem.children.append(&mut enumerated_values?);

        if let Some(v) = &self.derived_from {
            elem.attributes
                .insert(String::from("derivedFrom"), v.to_string());
        }

        Ok(elem)
    }
}
