use super::{new_element, Element, Encode, EncodeChildren};

use crate::error::*;
use crate::svd::{Field, RegisterInfo};

impl Encode for RegisterInfo {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        let mut elem = new_element("register", None);
        elem.children
            .push(new_element("name", Some(self.name.clone())));

        if let Some(v) = &self.display_name {
            elem.children
                .push(new_element("displayName", Some(v.clone())));
        }

        if let Some(v) = &self.description {
            elem.children
                .push(new_element("description", Some(v.clone())));
        }

        if let Some(v) = &self.alternate_group {
            elem.children
                .push(new_element("alternateGroup", Some(v.to_string())));
        }

        if let Some(v) = &self.alternate_register {
            elem.children
                .push(new_element("alternateRegister", Some(v.to_string())));
        }

        elem.children.push(new_element(
            "addressOffset",
            Some(format!("0x{:X}", self.address_offset)),
        ));

        elem.children.extend(self.properties.encode()?);

        if let Some(v) = &self.modified_write_values {
            elem.children.push(v.encode()?);
        }

        if let Some(v) = &self.write_constraint {
            elem.children.push(v.encode()?);
        }

        if let Some(v) = &self.fields {
            let children = v
                .iter()
                .map(Field::encode)
                .collect::<Result<Vec<Element>>>()?;
            if !children.is_empty() {
                let mut fields = new_element("fields", None);
                fields.children = children;
                elem.children.push(fields);
            }
        }

        if let Some(v) = &self.derived_from {
            elem.attributes
                .insert(String::from("derivedFrom"), v.to_string());
        }

        Ok(elem)
    }
}
