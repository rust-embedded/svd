use super::{
    new_node, Config, Element, ElementMerge, Encode, EncodeChildren, EncodeError, XMLNode,
};

use crate::{
    config::{change_case, format_number},
    svd::{Peripheral, PeripheralInfo},
};

impl Encode for Peripheral {
    type Error = EncodeError;

    fn encode_with_config(&self, config: &Config) -> Result<Element, EncodeError> {
        match self {
            Self::Single(info) => info.encode_with_config(config),
            Self::Array(info, array_info) => {
                let mut base = Element::new("peripheral");
                base.merge(&array_info.encode_with_config(config)?);
                base.merge(&info.encode_with_config(config)?);
                Ok(base)
            }
        }
    }
}

impl Encode for PeripheralInfo {
    type Error = EncodeError;

    fn encode_with_config(&self, config: &Config) -> Result<Element, EncodeError> {
        let mut elem = Element::new("peripheral");
        elem.children.push(new_node(
            "name",
            change_case(&self.name, config.peripheral_name),
        ));

        if let Some(v) = &self.display_name {
            elem.children.push(new_node("displayName", v.to_string()));
        }

        if let Some(v) = &self.version {
            elem.children.push(new_node("version", v.to_string()));
        }

        if let Some(v) = &self.description {
            elem.children.push(new_node("description", v.to_string()));
        }

        if let Some(v) = &self.alternate_peripheral {
            elem.children.push(new_node(
                "alternatePeripheral",
                change_case(v, config.peripheral_name),
            ));
        }

        if let Some(v) = &self.group_name {
            elem.children.push(new_node("groupName", v.to_string()));
        }

        if let Some(v) = &self.prepend_to_name {
            elem.children.push(new_node(
                "prependToName",
                change_case(v, config.peripheral_name),
            ));
        }

        if let Some(v) = &self.append_to_name {
            elem.children.push(new_node(
                "appendToName",
                change_case(v, config.peripheral_name),
            ));
        }

        if let Some(v) = &self.header_struct_name {
            elem.children.push(new_node(
                "headerStructName",
                change_case(v, config.peripheral_name),
            ));
        }

        elem.children.push(new_node(
            "baseAddress",
            format_number(self.base_address, config.peripheral_base_address),
        ));

        elem.children.extend(
            self.default_register_properties
                .encode_with_config(config)?,
        );

        if let Some(v) = &self.address_block {
            for ab in v {
                elem.children.push(ab.encode_node_with_config(config)?);
            }
        }

        let interrupts: Result<Vec<_>, _> = self
            .interrupt
            .iter()
            .map(|interrupt| interrupt.encode_node_with_config(config))
            .collect();

        elem.children.append(&mut interrupts?);

        if let Some(v) = &self.registers {
            let children: Result<Vec<_>, _> = v
                .iter()
                .map(|e| e.encode_node_with_config(config))
                .collect();

            elem.children.push({
                let mut e = Element::new("registers");
                e.children = children?;
                XMLNode::Element(e)
            });
        }

        if let Some(v) = &self.derived_from {
            elem.attributes.insert(
                String::from("derivedFrom"),
                change_case(v, config.peripheral_name),
            );
        }

        Ok(elem)
    }
}
