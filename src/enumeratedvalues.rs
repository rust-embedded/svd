extern crate xmltree;

use std::collections::HashMap;

use xmltree::Element;

#[macro_use]
use elementext::*;

use helpers::*;
use usage::*;
use enumeratedvalue::*;

#[derive(Clone, Debug, PartialEq)]
pub struct EnumeratedValues {
    pub name: Option<String>,
    pub usage: Option<Usage>,
    pub derived_from: Option<String>,
    pub values: Vec<EnumeratedValue>,
    // Reserve the right to add more fields to this struct
    pub _extensible: (),
}

impl ParseElem for EnumeratedValues {
    fn parse(tree: &Element) -> EnumeratedValues {
        assert_eq!(tree.name, "enumeratedValues");

        EnumeratedValues {
            name: tree.get_child_text("name"),
            usage: tree.get_child("usage").map(Usage::parse),
            derived_from: tree.attributes.get(&"derivedFrom".to_owned()).map(|s| {
                s.to_owned()
            }),
            values: tree.children
                .iter()
                .filter_map(EnumeratedValue::parse)
                .collect(),
            _extensible: (),
        }
    }
}

impl EncodeElem for EnumeratedValues {
    fn encode(&self) -> Element {
        let mut base = Element {
            name: String::from("enumeratedValues"),
            attributes: HashMap::new(),
            children: Vec::new(),
            text: None,
        };

        match self.name {
            Some(ref d) => {
                base.children.push(new_element("name", Some((*d).clone())));
            }
            None => (),
        };

        match self.usage {
            Some(ref v) => {
                base.children.push(v.encode());
            }
            None => (),
        };

        match self.derived_from {
            Some(ref v) => {
                base.attributes.insert(
                    String::from("derivedFrom"),
                    (*v).clone(),
                );
            }
            None => (),
        }

        for v in &self.values {
            base.children.push(v.encode());
        }

        base
    }
}

#[cfg(test)]
mod tests {
    macro_rules! try {
        ($e:expr) => {
            $e.expect(concat!(file!(), ":", line!(), " ", stringify!($e)))
        }
    }


    use super::*;

    #[test]
    fn decode_encode() {
        let example = String::from(
            "
            <enumeratedValues derivedFrom=\"fake-derivation.png\">
                <enumeratedValue>
                    <name>WS0</name>
                    <description>Zero wait-states inserted in fetch or read transfers</description>
                    <value>0x00000000</value>
                    <isDefault>true</isDefault>
                </enumeratedValue>
                <enumeratedValue>
                    <name>WS1</name>
                    <description>One wait-state inserted for each fetch or read transfer. See Flash Wait-States table for details</description>
                    <value>0x00000001</value>
                </enumeratedValue>
            </enumeratedValues>
        ",
        );

        let expected = EnumeratedValues {
            name: None,
            usage: None,
            derived_from: Some(String::from("fake-derivation.png")),
            values: vec![
                EnumeratedValue {
                    name: String::from("WS0"),
                    description: Some(String::from(
                        "Zero wait-states inserted in fetch or read transfers",
                    )),
                    value: Some(0),
                    is_default: Some(true),
                    _extensible: (),
                },
                EnumeratedValue {
                    name: String::from("WS1"),
                    description: Some(String::from(
                        "One wait-state inserted for each fetch or read transfer. See Flash Wait-States table for details",
                    )),
                    value: Some(1),
                    is_default: None,
                    _extensible: (),
                },
            ],
            _extensible: (),
        };

        let tree1 = &try_get_child!(Element::parse(example.as_bytes()));

        let parsed = EnumeratedValues::parse(tree1);
        assert_eq!(parsed, expected, "Parsing tree failed");

        let tree2 = &parsed.encode();
        assert_eq!(tree1, tree2, "Encoding value failed");
    }
}
