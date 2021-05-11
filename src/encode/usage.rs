use std::collections::HashMap;

use xmltree::Element;

use crate::encode::Encode;
use crate::error::*;

use crate::svd::Usage;
impl Encode for Usage {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        let text = match *self {
            Usage::Read => String::from("read"),
            Usage::Write => String::from("write"),
            Usage::ReadWrite => String::from("read-write"),
        };

        Ok(Element {
            prefix: None,
            namespace: None,
            namespaces: None,
            name: String::from("usage"),
            attributes: HashMap::new(),
            children: Vec::new(),
            text: Some(text),
        })
    }
}
