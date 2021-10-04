use super::{Element, Encode, EncodeError, XMLNode};

impl Encode for crate::svd::ModifiedWriteValues {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        let mut elem = Element::new("modifiedWriteValues");
        elem.children.push(XMLNode::Text(self.as_str().to_string()));
        Ok(elem)
    }
}
