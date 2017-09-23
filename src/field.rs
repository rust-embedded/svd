extern crate xmltree;

use std::collections::HashMap;
use xmltree::Element;

#[macro_use]
use elementext::*;

use helpers::*;
use access::*;
use writeconstraint::*;
use bitrange::*;
use enumeratedvalues::*;


#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    pub name: String,
    pub description: Option<String>,
    pub bit_range: BitRange,
    pub access: Option<Access>,
    pub enumerated_values: Vec<EnumeratedValues>,
    pub write_constraint: Option<WriteConstraint>,
    // Reserve the right to add more fields to this struct
    pub(crate) _extensible: (),
}

impl ParseElem for Field {
    fn parse(tree: &Element) -> Field {
        assert_eq!(tree.name, "field");

        Field {
            name: try_get_child!(tree.get_child_text("name")),
            description: tree.get_child_text("description"),
            bit_range: BitRange::parse(tree),
            access: tree.get_child("access").map(Access::parse),
            enumerated_values: tree.children
                .iter()
                .filter(|t| t.name == "enumeratedValues")
                .map(EnumeratedValues::parse)
                .collect::<Vec<_>>(),
            write_constraint: tree.get_child("writeConstraint").map(
                WriteConstraint::parse,
            ),
            _extensible: (),
        }
    }
}

impl EncodeElem for Field {
    fn encode(&self) -> Element {
        let mut elem = Element {
            name: String::from("field"),
            attributes: HashMap::new(),
            children: vec![
                new_element("name", Some(self.name.clone())),
                new_element("description", self.description.clone()),
            ],
            text: None,
        };

        // Add bit range
        elem.children.append(&mut self.bit_range.encode_children());

        match self.access {
            Some(ref v) => {
                elem.children.push(v.encode());
            }
            None => (),
        };

        let mut enumerated_values: Vec<Element> = self.enumerated_values.iter().map(|v| v.encode()).collect();
        elem.children.append(&mut enumerated_values);

        match self.write_constraint {
            Some(ref v) => {
                elem.children.push(v.encode());
            }
            None => (),
        };

        elem
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use enumeratedvalue::EnumeratedValue;

    #[test]
    fn decode_encode() {
        let types = vec![
            (
                Field {
                    name: String::from("MODE"),
                    description: Some(String::from("Read Mode")),
                    bit_range: BitRange {
                        offset: 24,
                        width: 2,
                        range_type: BitRangeType::OffsetWidth,
                    },
                    access: Some(Access::ReadWrite),
                    enumerated_values: vec![
                        EnumeratedValues {
                            name: None,
                            usage: None,
                            derived_from: None,
                            values: vec![
                                EnumeratedValue {
                                    name: String::from("WS0"),
                                    description: Some(String::from(
                                        "Zero wait-states inserted in fetch or read transfers",
                                    )),
                                    value: Some(0),
                                    is_default: None,
                                    _extensible: (),
                                },
                            ],
                            _extensible: (),
                        },
                    ],
                    write_constraint: None,
                    _extensible: (),
                },
                String::from(
                    "
            <field>
              <name>MODE</name>
              <description>Read Mode</description>
              <bitOffset>24</bitOffset>
              <bitWidth>2</bitWidth>
              <access>read-write</access>
              <enumeratedValues>
                <enumeratedValue>
                  <name>WS0</name>
                  <description>Zero wait-states inserted in fetch or read transfers</description>
                  <value>0x00000000</value>
                </enumeratedValue>
              </enumeratedValues>
            </field>
            ",
                )
            ),
        ];

        for (a, s) in types {
            let tree1 = &try_get_child!(Element::parse(s.as_bytes()));
            let v = Field::parse(tree1);
            assert_eq!(v, a, "Parsing `{}` expected `{:?}`", s, a);
            let tree2 = &v.encode();
            assert_eq!(tree1, tree2, "Encoding {:?} expected {}", a, s);
        }
    }
}
