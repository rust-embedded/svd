use super::{new_element, Element, Encode, EncodeError};

impl Encode for crate::svd::AddressBlock {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        let children = vec![
            new_element("offset", Some(format!("0x{:X}", self.offset))),
            new_element("size", Some(format!("0x{:X}", self.size))),
            self.usage.encode()?,
        ];
        let mut elem = new_element("addressBlock", None);
        elem.children = children;
        Ok(elem)
    }
}

impl Encode for crate::svd::AddressBlockUsage {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        Ok(new_element("usage", Some(self.to_str().to_string())))
    }
}
