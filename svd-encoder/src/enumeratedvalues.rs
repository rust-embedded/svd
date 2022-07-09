use super::{new_node, Config, Element, Encode, EncodeError};

use crate::{config::change_case, svd::EnumeratedValues};

impl Encode for EnumeratedValues {
    type Error = EncodeError;

    fn encode_with_config(&self, config: &Config) -> Result<Element, EncodeError> {
        let mut base = Element::new("enumeratedValues");

        if let Some(d) = &self.name {
            base.children.push(new_node(
                "name",
                change_case(d, config.enumerated_values_name),
            ));
        };

        if let Some(v) = &self.usage {
            base.children.push(v.encode_node()?);
        };

        if let Some(v) = &self.derived_from {
            base.attributes.insert(
                String::from("derivedFrom"),
                change_case(v, config.enumerated_values_name),
            );
        }

        for v in &self.values {
            base.children.push(v.encode_node_with_config(config)?);
        }

        Ok(base)
    }
}
