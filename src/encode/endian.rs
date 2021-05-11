use std::collections::HashMap;
use xmltree::Element;

use crate::encode::Encode;

use crate::error::*;

use crate::svd::Endian;
impl Encode for Endian {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        let text = match *self {
            Endian::Little => String::from("little"),
            Endian::Big => String::from("big"),
            Endian::Selectable => String::from("selectable"),
            Endian::Other => String::from("other"),
        };

        Ok(Element {
            prefix: None,
            namespace: None,
            namespaces: None,
            name: String::from("endian"),
            attributes: HashMap::new(),
            children: Vec::new(),
            text: Some(text),
        })
    }
}
