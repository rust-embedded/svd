extern crate xmltree;

use std::collections::HashMap;

use xmltree::Element;

#[macro_use]
use elementext::*;

use helpers::*;
use access::*;
use parse;
use Field;
use writeconstraint::*;


#[derive(Clone, Debug, PartialEq)]
pub struct RegisterInfo {
    pub name: String,
    pub description: String,
    pub address_offset: u32,
    pub size: Option<u32>,
    pub access: Option<Access>,
    pub reset_value: Option<u32>,
    pub reset_mask: Option<u32>,
    /// `None` indicates that the `<fields>` node is not present
    pub fields: Option<Vec<Field>>,
    pub write_constraint: Option<WriteConstraint>,
    // Reserve the right to add more fields to this struct
    pub _extensible: (),
}

impl ParseElem for RegisterInfo {
    fn parse(tree: &Element) -> RegisterInfo {
        RegisterInfo {
            name: try_get_child!(tree.get_child_text("name")),
            description: try_get_child!(tree.get_child_text("description")),
            address_offset: {
                try_get_child!(parse::u32(try_get_child!(tree.get_child("addressOffset"))))
            },
            size: tree.get_child("size").map(|t| try_get_child!(parse::u32(t))),
            access: tree.get_child("access").map(Access::parse),
            reset_value: tree.get_child("resetValue").map(|t| try_get_child!(parse::u32(t))),
            reset_mask: tree.get_child("resetMask").map(|t| try_get_child!(parse::u32(t))),
            fields: tree.get_child("fields").map(|fs| {
                fs.children.iter().map(Field::parse).collect()
            }),
            write_constraint: tree.get_child("writeConstraint").map(
                WriteConstraint::parse,
            ),
            _extensible: (),
        }
    }
}

impl EncodeElem for RegisterInfo {
    fn encode(&self) -> Element {
        let mut elem = Element {
            name: String::from("register"),
            attributes: HashMap::new(),
            children: vec![
                new_element("name", Some(self.name.clone())),
                new_element("description", Some(self.description.clone())),
                new_element(
                    "addressOffset",
                    Some(format!("0x{:x}", self.address_offset))
                ),
            ],
            text: None,
        };

        match self.size {
            Some(ref v) => {
                elem.children.push(
                    new_element("size", Some(format!("{}", v))),
                );
            }
            None => (),
        };

        match self.access {
            Some(ref v) => {
                elem.children.push(v.encode());
            }
            None => (),
        };

        match self.reset_value {
            Some(ref v) => {
                elem.children.push(new_element(
                    "resetValue",
                    Some(format!("0x{:08.x}", v)),
                ));
            }
            None => (),
        };

        match self.reset_mask {
            Some(ref v) => {
                elem.children.push(new_element(
                    "resetMask",
                    Some(format!("0x{:08.x}", v)),
                ));
            }
            None => (),
        };

        match self.fields {
            Some(ref v) => {
                let fields = Element {
                    name: String::from("fields"),
                    attributes: HashMap::new(),
                    children: v.iter().map(Field::encode).collect(),
                    text: None,
                };
                elem.children.push(fields);
            }
            None => (),
        };

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
    use bitrange::*;

    #[test]
    fn decode_encode() {
        let types = vec![
            (
                RegisterInfo {
                    name: String::from("WRITECTRL"),
                    description: String::from("Write Control Register"),
                    address_offset: 8,
                    size: Some(32),
                    access: Some(Access::ReadWrite),
                    reset_value: Some(0x00000000),
                    reset_mask: Some(0x00000023),
                    fields: Some(vec![
                        Field {
                            name: String::from("WREN"),
                            description: Some(String::from("Enable Write/Erase Controller")),
                            bit_range: BitRange {
                                offset: 0,
                                width: 1,
                                range_type: BitRangeType::OffsetWidth,
                            },
                            access: Some(Access::ReadWrite),
                            enumerated_values: Vec::new(),
                            write_constraint: None,
                            _extensible: (),
                        },
                    ]),
                    write_constraint: None,
                    _extensible: (),
                },
                String::from(
                    "
            <register>
                <name>WRITECTRL</name>
                <description>Write Control Register</description>
                <addressOffset>0x8</addressOffset>
                <size>32</size>
                <access>read-write</access>
                <resetValue>0x00000000</resetValue>
                <resetMask>0x00000023</resetMask>
                <fields>
                    <field>
                        <name>WREN</name>
                        <description>Enable Write/Erase Controller</description>
                        <bitOffset>0</bitOffset>
                        <bitWidth>1</bitWidth>
                        <access>read-write</access>
                    </field>
                </fields>
            </register>
            ",
                )
            ),
        ];

        for (a, s) in types {
            let tree1 = &try_get_child!(Element::parse(s.as_bytes()));
            let v = RegisterInfo::parse(tree1);
            assert_eq!(v, a, "Parsing `{}` expected `{:?}`", s, a);
            let tree2 = &v.encode();
            assert_eq!(tree1, tree2, "Encoding {:?} expected {}", a, s);
        }
    }
}
