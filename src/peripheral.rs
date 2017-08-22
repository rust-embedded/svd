extern crate xmltree;

use std::collections::HashMap;

use xmltree::Element;
use ElementExt;

use parse;
use helpers::*;
use interrupt::*;
use register::*;

macro_rules! try {
    ($e:expr) => {
        $e.expect(concat!(file!(), ":", line!(), " ", stringify!($e)))
    }
}

#[derive(Clone, Debug)]
pub struct Peripheral {
    pub name: String,
    pub version: Option<String>,
    pub group_name: Option<String>,
    pub description: Option<String>,
    pub base_address: u32,
    pub interrupt: Vec<Interrupt>,
    /// `None` indicates that the `<registers>` node is not present
    pub registers: Option<Vec<Register>>,
    pub derived_from: Option<String>,
    // Reserve the right to add more fields to this struct
    _extensible: (),
}

impl ParseElem for Peripheral {
    fn parse(tree: &Element) -> Peripheral {
        assert_eq!(tree.name, "peripheral");

        Peripheral {
            name: try!(tree.get_child_text("name")),
            version: tree.get_child_text("version"),
            group_name: tree.get_child_text("groupName"),
            description: tree.get_child_text("description"),
            base_address: try!(parse::u32(try!(tree.get_child("baseAddress")))),
            interrupt: tree.children
                .iter()
                .filter(|t| t.name == "interrupt")
                .map(Interrupt::parse)
                .collect::<Vec<_>>(),
            registers: tree.get_child("registers").map(|rs| {
                rs.children
                    .iter()
                    .filter(|v| v.name == "register")
                    .map(Register::parse)
                    .collect()
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
            Some(format!("{:.08x}", self.base_address)),
        ));
        elem.children.append(&mut self.interrupt
            .iter()
            .map(Interrupt::encode)
            .collect());
        match self.registers {
            Some(ref v) => {
                elem.children.push(Element {
                    name: String::from("registers"),
                    attributes: HashMap::new(),
                    children: v.iter().map(Register::encode).collect(),
                    text: None,
                });
            }
            None => (),
        };

        elem
    }
}
