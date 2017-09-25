
use std::collections::HashMap;

use xmltree::Element;

use helpers::{ParseElem, EncodeElem};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Endian {
    Little,
    Big,
    Selectable,
    Other,
}

impl ParseElem for Endian {
    fn parse(tree: &Element) -> Endian {
        let text = try_get_child!(tree.text.as_ref());

        match &text[..] {
            "little" => Endian::Little,
            "big" => Endian::Big,
            "selectable" => Endian::Selectable,
            "other" => Endian::Other,
            _ => panic!("unknown endian variant: {}", text),
        }
    }
}

impl EncodeElem for Endian {
    fn encode(&self) -> Element {
        let text = match *self {
            Endian::Little => String::from("little"),
            Endian::Big => String::from("big"),
            Endian::Selectable => String::from("selectable"),
            Endian::Other => String::from("other"),
        };

        Element {
            name: String::from("endian"),
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
            (Endian::Little, String::from("<endian>little</endian>")),
            (Endian::Big, String::from("<endian>big</endian>")),
            (
                Endian::Selectable,
                String::from("<endian>selectable</endian>")
            ),
            (Endian::Other, String::from("<endian>other</endian>")),
        ];

        for (e, s) in types {
            let tree1 = &try_get_child!(Element::parse(s.as_bytes()));
            let endian = Endian::parse(tree1);
            assert_eq!(endian, e, "Parsing `{}` expected `{:?}`", s, e);
            let tree2 = &endian.encode();
            assert_eq!(tree1, tree2, "Encoding {:?} expected {}", e, s);
        }
    }
}
