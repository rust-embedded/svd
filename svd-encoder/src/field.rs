use super::{new_node, Config, Element, ElementMerge, Encode, EncodeError, XMLNode};
use crate::bitrange::encode_bitrange;

use crate::config::change_case;
use crate::svd::{Field, FieldInfo};

impl Encode for Field {
    type Error = EncodeError;

    fn encode_with_config(&self, config: &Config) -> Result<Element, EncodeError> {
        match self {
            Self::Single(info) => info.encode_with_config(config),
            Self::Array(info, array_info) => {
                let mut base = Element::new("field");
                base.merge(&array_info.encode_with_config(config)?);
                base.merge(&info.encode_with_config(config)?);
                Ok(base)
            }
        }
    }
}

impl Encode for FieldInfo {
    type Error = EncodeError;

    fn encode_with_config(&self, config: &Config) -> Result<Element, EncodeError> {
        let mut elem = Element::new("field");
        elem.children
            .push(new_node("name", change_case(&self.name, config.field_name)));

        if let Some(description) = &self.description {
            elem.children
                .push(new_node("description", description.clone()))
        }

        // Add bit range
        elem.children
            .append(&mut encode_bitrange(&self.bit_range, config)?);

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
            .map(|v| v.encode_node_with_config(config))
            .collect();
        elem.children.append(&mut enumerated_values?);

        if let Some(v) = &self.derived_from {
            elem.attributes.insert(
                String::from("derivedFrom"),
                change_case(v, config.field_name),
            );
        }

        Ok(elem)
    }
}
