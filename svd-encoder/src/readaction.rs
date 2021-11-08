use super::{Element, Encode, EncodeError, XMLNode};

impl Encode for crate::svd::ReadAction {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        let mut elem = Element::new("readAction");
        elem.children.push(XMLNode::Text(self.as_str().to_string()));
        Ok(elem)
    }
}
