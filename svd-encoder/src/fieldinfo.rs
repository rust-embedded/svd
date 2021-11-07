use super::{new_node, Element, Encode, EncodeError, XMLNode};
use crate::bitrange::encode_bitrange;

use crate::svd::FieldInfo;

impl Encode for FieldInfo {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        let mut elem = Element::new("field");
        elem.children.push(new_node("name", self.name.clone()));

        if let Some(description) = &self.description {
            elem.children
                .push(new_node("description", description.clone()))
        }

        // Add bit range
        elem.children.append(&mut encode_bitrange(&self.bit_range)?);

        if let Some(v) = &self.access {
            elem.children.push(v.encode_node()?);
        }

        if let Some(v) = &self.modified_write_values {
            elem.children.push(v.encode_node()?);
        }

        if let Some(v) = &self.write_constraint {
            elem.children.push(v.encode_node()?);
        }

        if let Some(v) = &self.read_action {
            elem.children.push(v.encode_node()?);
        }

        let enumerated_values: Result<Vec<XMLNode>, EncodeError> = self
            .enumerated_values
            .iter()
            .map(|v| v.encode_node())
            .collect();
        elem.children.append(&mut enumerated_values?);

        if let Some(v) = &self.derived_from {
            elem.attributes
                .insert(String::from("derivedFrom"), v.to_string());
        }

        Ok(elem)
    }
}
