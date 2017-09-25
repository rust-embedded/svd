
use std::collections::HashMap;

use xmltree::Element;


use elementext::*;

use helpers::*;
use parse;


#[derive(Clone, Debug, PartialEq)]
pub struct EnumeratedValue {
    pub name: String,
    pub description: Option<String>,
    pub value: Option<u32>,
    pub is_default: Option<bool>,
    // Reserve the right to add more fields to this struct
    pub(crate) _extensible: (),
}

impl EnumeratedValue {
    pub fn parse(tree: &Element) -> Option<EnumeratedValue> {
        assert_eq!(tree.name, "enumeratedValue");

        Some(EnumeratedValue {
            name: try_get_child!(tree.get_child_text("name")),
            description: tree.get_child_text("description"),
            value: tree.get_child("value").map(
                |t| try_get_child!(parse::u32(t)),
            ),
            is_default: tree.get_child_text("isDefault").map(|t| {
                try_get_child!(t.parse())
            }),
            _extensible: (),
        })
    }
}

impl EncodeElem for EnumeratedValue {
    fn encode(&self) -> Element {
        let mut base = Element {
            name: String::from("enumeratedValue"),
            attributes: HashMap::new(),
            children: vec![new_element("name", Some(self.name.clone()))],
            text: None,
        };

        match self.description {
            Some(ref d) => {
                let s = (*d).clone();
                base.children.push(new_element("description", Some(s)));
            }
            None => (),
        };

        match self.value {
            Some(ref v) => {
                base.children.push(new_element(
                    "value",
                    Some(format!("0x{:08.x}", *v)),
                ));
            }
            None => (),
        };

        match self.is_default {
            Some(ref v) => {
                base.children.push(new_element(
                    "isDefault",
                    Some(format!("{}", v)),
                ));
            }
            None => (),
        };

        base
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_encode() {
        let example = String::from(
            "
            <enumeratedValue>
                <name>WS0</name>
                <description>Zero wait-states inserted in fetch or read transfers</description>
                <value>0x00000000</value>
                <isDefault>true</isDefault>
            </enumeratedValue>
        ",
        );
        let expected = EnumeratedValue {
            name: String::from("WS0"),
            description: Some(String::from(
                "Zero wait-states inserted in fetch or read transfers",
            )),
            value: Some(0),
            is_default: Some(true),
            _extensible: (),
        };

        let tree1 = &try_get_child!(Element::parse(example.as_bytes()));

        let parsed = EnumeratedValue::parse(tree1).unwrap();
        assert_eq!(parsed, expected, "Parsing tree failed");

        let tree2 = &parsed.encode();
        assert_eq!(tree1, tree2, "Encoding value failed");
    }
}
