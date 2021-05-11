use super::{new_element, Element, Encode};

use crate::error::*;
use crate::svd::AddressBlock;

impl Encode for AddressBlock {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        let children = vec![
            new_element("offset", Some(format!("0x{:X}", self.offset))),
            new_element("size", Some(format!("0x{:X}", self.size))),
            new_element("usage", Some(self.usage.clone())),
        ];
        let mut elem = new_element("addressBlock", None);
        elem.children = children;
        Ok(elem)
    }
}
