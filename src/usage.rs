
use std::collections::HashMap;

use xmltree::Element;

use helpers::*;


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Usage {
    Read,
    Write,
    ReadWrite,
}

impl ParseElem for Usage {
    fn parse(tree: &Element) -> Usage {
        let text = try_get_child!(tree.text.as_ref());

        match &text[..] {
            "read" => Usage::Read,
            "write" => Usage::Write,
            "read-write" => Usage::ReadWrite,
            _ => panic!("unknown usage variant: {}", text),
        }
    }
}

impl EncodeElem for Usage {
    fn encode(&self) -> Element {
        let text = match *self {
            Usage::Read => String::from("read"),
            Usage::Write => String::from("write"),
            Usage::ReadWrite => String::from("read-write"),
        };

        Element {
            name: String::from("usage"),
            attributes: HashMap::new(),
            children: Vec::new(),
            text: Some(text),
        }
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
            let tree1 = &try_get_child!(Element::parse(s.as_bytes()));
            let elem = Usage::parse(tree1);
            assert_eq!(elem, e, "Parsing `{}` expected `{:?}`", s, e);
            let tree2 = &elem.encode();
            assert_eq!(tree1, tree2, "Encoding {:?} expected {}", e, s);
        }
    }
}
