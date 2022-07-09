use super::{Config, Element, Encode, EncodeError, XMLNode};

impl Encode for crate::svd::Usage {
    type Error = EncodeError;

    fn encode_with_config(&self, _config: &Config) -> Result<Element, EncodeError> {
        let mut elem = Element::new("usage");
        elem.children.push(XMLNode::Text(self.as_str().to_string()));
        Ok(elem)
    }
}
