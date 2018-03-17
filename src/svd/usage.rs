use std::collections::HashMap;

use xmltree::Element;

use ::parse;
use ::types::{Parse, Encode};
use ::error::SVDError;


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Usage {
    Read,
    Write,
    ReadWrite,
}

impl Parse for Usage {
    type Object = Usage;
    type Error = SVDError;

    fn parse(tree: &Element) -> Result<Usage, SVDError> {
        let text = parse::get_text(tree)?;

        match &text[..] {
            "read" => Ok(Usage::Read),
            "write" => Ok(Usage::Write),
            "read-write" => Ok(Usage::ReadWrite),
            _ => Err(SVDError::UnknownUsageVariant(tree.clone())),
        }
    }
}

impl Encode for Usage {
    type Error = SVDError;

    fn encode(&self) -> Result<Element, SVDError> {
        let text = match *self {
            Usage::Read => String::from("read"),
            Usage::Write => String::from("write"),
            Usage::ReadWrite => String::from("read-write"),
        };

        Ok(Element {
            name: String::from("usage"),
            attributes: HashMap::new(),
            children: Vec::new(),
            text: Some(text),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_encode() {
        let types = vec![
            (Usage::Read, String::from("<usage>read</usage>")),
            (Usage::Write, String::from("<usage>write</usage>")),
            (Usage::ReadWrite, String::from("<usage>read-write</usage>")),
        ];

        for (e, s) in types {
            let tree1 = Element::parse(s.as_bytes()).unwrap();
            let elem = Usage::parse(&tree1).unwrap();
            assert_eq!(elem, e, "Parsing `{}` expected `{:?}`", s, e);
            let tree2 = elem.encode().unwrap();
            assert_eq!(tree1, tree2, "Encoding {:?} expected {}", e, s);
        }
    }
}
