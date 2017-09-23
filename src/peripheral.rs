extern crate xmltree;
extern crate either;

use std::collections::HashMap;
use std::ops::Deref;

use xmltree::Element;
use either::Either;

#[macro_use]
use elementext::*;

use parse;
use helpers::*;
use interrupt::*;
use register::*;
use cluster::*;
use addressblock::*;
use registercluster::*;


#[derive(Clone, Debug)]
pub struct Peripheral {
    pub name: String,
    pub version: Option<String>,
    pub display_name: Option<String>,
    pub group_name: Option<String>,
    pub description: Option<String>,
    pub base_address: u32,
    pub address_block: Option<AddressBlock>,
    pub interrupt: Vec<Interrupt>,
    /// `None` indicates that the `<registers>` node is not present
    pub registers: Option<Vec<Either<Register, Cluster>>>,
    pub derived_from: Option<String>,
    // Reserve the right to add more fields to this struct
    pub(crate) _extensible: (),
}

impl ParseElem for Peripheral {
    fn parse(tree: &Element) -> Peripheral {
        assert_eq!(tree.name, "peripheral");

        Peripheral {
            name: try_get_child!(tree.get_child_text("name")),
            version: tree.get_child_text("version"),
            display_name: tree.get_child_text("displayName"),
            group_name: tree.get_child_text("groupName"),
            description: tree.get_child_text("description"),
            base_address: try_get_child!(parse::u32(try_get_child!(tree.get_child("baseAddress")))),
            address_block: tree.get_child("addressBlock").map(AddressBlock::parse),
            interrupt: tree.children
                .iter()
                .filter(|t| t.name == "interrupt")
                .map(Interrupt::parse)
                .collect::<Vec<_>>(),
            registers: tree.get_child("registers").map(|rs| {
                rs.children.iter().map(cluster_register_parse).collect()
            }),
            derived_from: tree.attributes.get("derivedFrom").map(|s| s.to_owned()),
            _extensible: (),
        }
    }
}

impl EncodeElem for Peripheral {
    fn encode(&self) -> Element {
        let mut elem = Element {
            name: String::from("peripheral"),
            attributes: HashMap::new(),
            children: vec![new_element("name", Some(self.name.clone()))],
            text: None,
        };

        match self.version {
            Some(ref v) => {
                elem.children.push(
                    new_element("version", Some(format!("{}", v))),
                );
            }
            None => (),
        };
        match self.display_name {
            Some(ref v) => {
                elem.children.push(new_element(
                    "displayName",
                    Some(format!("{}", v)),
                ));
            }
            None => (),
        };
        match self.group_name {
            Some(ref v) => {
                elem.children.push(new_element(
                    "groupName",
                    Some(format!("{}", v)),
                ));
            }
            None => (),
        };
        match self.description {
            Some(ref v) => {
                elem.children.push(new_element(
                    "description",
                    Some(format!("{}", v)),
                ));
            }
            None => (),
        };
        elem.children.push(new_element(
            "baseAddress",
            Some(format!("0x{:.08x}", self.base_address)),
        ));
        match self.address_block {
            Some(ref v) => {
                elem.children.push(v.encode());
            }
            None => (),
        };

        elem.children.append(&mut self.interrupt
            .iter()
            .map(Interrupt::encode)
            .collect());
        match self.registers {
            Some(ref v) => {
                elem.children.push(Element {
                    name: String::from("registers"),
                    attributes: HashMap::new(),
                    children: v.iter().map(|&ref e| {
                        if e.is_left() {
                            e.clone().left().unwrap().encode()
                        } else {
                            e.clone().right().unwrap().encode()
                        }
                    }).collect(),
                    text: None,
                });
            }
            None => (),
        };

        match self.derived_from {
            Some(ref v) => {
                elem.attributes.insert(
                    String::from("derivedFrom"),
                    format!("{}", v),
                );
            }
            None => (),
        }

        elem
    }
}
