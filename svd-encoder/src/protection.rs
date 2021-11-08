use super::{Element, Encode, EncodeError, XMLNode};

impl Encode for crate::svd::Protection {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        let mut elem = Element::new("protection");
        elem.children.push(XMLNode::Text(self.as_str().to_string()));
        Ok(elem)
    }
}
