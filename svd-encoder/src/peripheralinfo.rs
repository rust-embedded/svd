use super::{new_node, Element, Encode, EncodeChildren, EncodeError, XMLNode};

use crate::svd::{Interrupt, PeripheralInfo};

impl Encode for PeripheralInfo {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        let mut elem = Element::new("peripheral");
        elem.children.push(new_node("name", self.name.clone()));

        if let Some(v) = &self.display_name {
            elem.children.push(new_node("displayName", v.to_string()));
        }

        if let Some(v) = &self.version {
            elem.children.push(new_node("version", v.to_string()));
        }

        if let Some(v) = &self.description {
            elem.children.push(new_node("description", v.to_string()));
        }

        if let Some(v) = &self.group_name {
            elem.children.push(new_node("groupName", v.to_string()));
        }
        elem.children.push(new_node(
            "baseAddress",
            format!("0x{:.08X}", self.base_address),
        ));

        elem.children
            .extend(self.default_register_properties.encode()?);

        if let Some(v) = &self.address_block {
            for ab in v {
                elem.children.push(ab.encode_node()?);
            }
        }

        let interrupts: Result<Vec<_>, _> =
            self.interrupt.iter().map(Interrupt::encode_node).collect();

        elem.children.append(&mut interrupts?);

        if let Some(v) = &self.registers {
            let children: Result<Vec<_>, _> = v.iter().map(|e| e.encode_node()).collect();

            elem.children.push({
                let mut e = Element::new("registers");
                e.children = children?;
                XMLNode::Element(e)
            });
        }

        if let Some(v) = &self.derived_from {
            elem.attributes
                .insert(String::from("derivedFrom"), v.to_string());
        }

        Ok(elem)
    }
}
