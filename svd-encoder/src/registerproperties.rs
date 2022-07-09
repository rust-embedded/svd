use super::{new_node, Config, Encode, EncodeChildren, EncodeError, XMLNode};

use crate::{config::format_number, svd::RegisterProperties};

impl EncodeChildren for RegisterProperties {
    type Error = EncodeError;

    fn encode_with_config(&self, config: &Config) -> Result<Vec<XMLNode>, EncodeError> {
        let mut children = Vec::new();

        if let Some(v) = &self.size {
            children.push(new_node("size", format_number(*v, config.register_size)));
        };

        if let Some(v) = &self.access {
            children.push(v.encode_node_with_config(config)?);
        };

        if let Some(v) = &self.protection {
            children.push(v.encode_node_with_config(config)?);
        };

        if let Some(v) = &self.reset_value {
            children.push(new_node(
                "resetValue",
                format_number(*v, config.register_reset_value),
            ));
        };

        if let Some(v) = &self.reset_mask {
            children.push(new_node(
                "resetMask",
                format_number(*v, config.register_reset_mask),
            ));
        };

        Ok(children)
    }
}
