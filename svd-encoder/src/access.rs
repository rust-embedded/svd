use super::{Element, Encode, EncodeError, XMLNode};

impl Encode for crate::svd::Access {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        let mut elem = Element::new("access");
        elem.children.push(XMLNode::Text(self.to_str().to_string()));
        Ok(elem)
    }
}
