use super::{
    new_node, Config, Element, ElementMerge, Encode, EncodeChildren, EncodeError, XMLNode,
};

use crate::{
    config::{change_case, format_number},
    svd::{Register, RegisterInfo},
};

impl Encode for Register {
    type Error = EncodeError;

    fn encode_with_config(&self, config: &Config) -> Result<Element, EncodeError> {
        match self {
            Self::Single(info) => info.encode_with_config(config),
            Self::Array(info, array_info) => {
                let mut base = Element::new("register");
                base.merge(&array_info.encode_with_config(config)?);
                base.merge(&info.encode_with_config(config)?);
                Ok(base)
            }
        }
    }
}

impl Encode for RegisterInfo {
    type Error = EncodeError;

    fn encode_with_config(&self, config: &Config) -> Result<Element, EncodeError> {
        let mut elem = Element::new("register");
        elem.children.push(new_node(
            "name",
            change_case(&self.name, config.register_name),
        ));

        if let Some(v) = &self.display_name {
            elem.children.push(new_node("displayName", v));
        }

        if let Some(v) = &self.description {
            elem.children.push(new_node("description", v));
        }

        if let Some(v) = &self.alternate_group {
            elem.children.push(new_node("alternateGroup", v));
        }

        if let Some(v) = &self.alternate_register {
            elem.children.push(new_node(
                "alternateRegister",
                change_case(v, config.register_name),
            ));
        }

        elem.children.push(new_node(
            "addressOffset",
            format_number(self.address_offset, config.register_address_offset),
        ));

        elem.children
            .extend(self.properties.encode_with_config(config)?);

        if let Some(v) = &self.modified_write_values {
            elem.children.push(v.encode_node_with_config(config)?);
        }

        if let Some(v) = &self.write_constraint {
            elem.children.push(v.encode_node()?);
        }

        if let Some(v) = &self.read_action {
            elem.children.push(v.encode_node()?);
        }

        if let Some(v) = &self.fields {
            let children = v
                .iter()
                .map(|field| field.encode_node_with_config(config))
                .collect::<Result<Vec<_>, EncodeError>>()?;
            if !children.is_empty() {
                let mut fields = Element::new("fields");
                fields.children = children;
                elem.children.push(XMLNode::Element(fields));
            }
        }

        if let Some(v) = &self.derived_from {
            elem.attributes.insert(
                String::from("derivedFrom"),
                change_case(v, config.register_name),
            );
        }

        Ok(elem)
    }
}
