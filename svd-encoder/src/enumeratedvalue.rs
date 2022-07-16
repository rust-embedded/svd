use super::{new_node, Config, Element, Encode, EncodeError};

use crate::{
    config::{change_case, format_number},
    svd::EnumeratedValue,
};

impl Encode for EnumeratedValue {
    type Error = EncodeError;

    fn encode_with_config(&self, config: &Config) -> Result<Element, EncodeError> {
        let mut base = Element::new("enumeratedValue");
        base.children.push(new_node(
            "name",
            change_case(&self.name, config.enumerated_value_name),
        ));

        if let Some(d) = &self.description {
            base.children.push(new_node("description", d));
        };

        if let Some(v) = &self.value {
            base.children.push(new_node(
                "value",
                format_number(*v, config.enumerated_value_value),
            ));
        };

        if let Some(v) = &self.is_default {
            base.children.push(new_node("isDefault", format!("{}", v)));
        };

        Ok(base)
    }
}
