use super::{new_node, Element, Encode, EncodeError, XMLNode};

impl Encode for crate::svd::AddressBlock {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        let children = vec![
            new_node("offset", format!("0x{:X}", self.offset)),
            new_node("size", format!("0x{:X}", self.size)),
            self.usage.encode_node()?,
        ];
        let mut elem = Element::new("addressBlock");
        elem.children = children;
        Ok(elem)
    }
}

impl Encode for crate::svd::AddressBlockUsage {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        let mut elem = Element::new("usage");
        elem.children.push(XMLNode::Text(self.to_str().to_string()));
        Ok(elem)
    }
}
