use super::{new_element, Element, Encode, EncodeChildren};

use crate::error::*;
use crate::svd::{Interrupt, Peripheral};

impl Encode for Peripheral {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        let mut elem = new_element("peripheral", None);
        elem.children
            .push(new_element("name", Some(self.name.clone())));

        if let Some(v) = &self.display_name {
            elem.children
                .push(new_element("displayName", Some(v.to_string())));
        }

        if let Some(v) = &self.version {
            elem.children
                .push(new_element("version", Some(v.to_string())));
        }

        if let Some(v) = &self.description {
            elem.children
                .push(new_element("description", Some(v.to_string())));
        }

        if let Some(v) = &self.group_name {
            elem.children
                .push(new_element("groupName", Some(v.to_string())));
        }
        elem.children.push(new_element(
            "baseAddress",
            Some(format!("0x{:.08X}", self.base_address)),
        ));

        elem.children
            .extend(self.default_register_properties.encode()?);

        if let Some(v) = &self.address_block {
            elem.children.push(v.encode()?);
        }

        let interrupts: Result<Vec<_>, _> = self.interrupt.iter().map(Interrupt::encode).collect();

        elem.children.append(&mut interrupts?);

        if let Some(v) = &self.registers {
            let children: Result<Vec<_>, _> = v.iter().map(|e| e.encode()).collect();

            elem.children.push({
                let mut e = new_element("registers", None);
                e.children = children?;
                e
            });
        }

        if let Some(v) = &self.derived_from {
            elem.attributes
                .insert(String::from("derivedFrom"), v.to_string());
        }

        Ok(elem)
    }
}
