use crate::config::format_number;

use super::{new_node, Config, Element, Encode, EncodeError, XMLNode};

impl Encode for crate::svd::AddressBlock {
    type Error = EncodeError;

    fn encode_with_config(&self, config: &Config) -> Result<Element, EncodeError> {
        let mut children = vec![
            new_node(
                "offset",
                format_number(self.offset, config.address_block_offset),
            ),
            new_node("size", format_number(self.size, config.address_block_size)),
            self.usage.encode_node()?,
        ];
        if let Some(v) = &self.protection {
            children.push(v.encode_node()?);
        };
        let mut elem = Element::new("addressBlock");
        elem.children = children;
        Ok(elem)
    }
}

impl Encode for crate::svd::AddressBlockUsage {
    type Error = EncodeError;

    fn encode_with_config(&self, _config: &Config) -> Result<Element, EncodeError> {
        let mut elem = Element::new("usage");
        elem.children.push(XMLNode::Text(self.as_str().to_string()));
        Ok(elem)
    }
}
