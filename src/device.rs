use xmltree::Element;

use std::collections::HashMap;


use elementext::ElementExt;
use helpers::{ParseElem, EncodeElem, new_element};
use parse;

use cpu::Cpu;
use defaults::Defaults;
use peripheral::Peripheral;



#[derive(Clone, Debug)]
pub struct Device {
    pub name: String,
    schema_version: String,
    pub version: String,
    pub description: String,
    pub address_unit_bits: u32,
    pub width: u32,
    pub cpu: Option<Cpu>,
    pub peripherals: Vec<Peripheral>,
    pub defaults: Defaults,
    // Reserve the right to add more fields to this struct
    pub(crate) _extensible: (),
}

impl ParseElem for Device {
    fn parse(tree: &Element) -> Device {
        Device {
            schema_version: tree.attributes.get("schemaVersion").unwrap().clone(),
            name: try_get_child!(tree.get_child_text("name")),
            version: try_get_child!(tree.get_child_text("version")),
            description: try_get_child!(tree.get_child_text("description")),
            address_unit_bits: try_get_child!(parse::u32(
                try_get_child!(tree.get_child("addressUnitBits")),
            )),
            width: try_get_child!(parse::u32(try_get_child!(tree.get_child("width")))),
            cpu: tree.get_child("cpu").map(Cpu::parse),
            peripherals: try_get_child!(tree.get_child("peripherals"))
                .children
                .iter()
                .map(Peripheral::parse)
                .collect(),
            defaults: Defaults::parse(tree),
            _extensible: (),
        }
    }
}

impl EncodeElem for Device {
    fn encode(&self) -> Element {
        let mut elem = Element {
            name: String::from("device"),
            attributes: HashMap::new(),
            children: vec![
                new_element("name", Some(self.name.clone())),
                new_element("version", Some(self.version.clone())),
                new_element("description", Some(self.description.clone())),
                new_element(
                    "addressUnitBits",
                    Some(format!("{}", self.address_unit_bits))
                ),
                new_element("width", Some(format!("{}", self.width))),
            ],
            text: None,
        };

        elem.attributes.insert(
            String::from("xmlns:xs"),
            String::from("http://www.w3.org/2001/XMLSchema-instance"),
        );
        elem.attributes.insert(
            String::from("schemaVersion"),
            format!("{}", self.schema_version),
        );
        elem.attributes.insert(
            String::from("xs:noNamespaceSchemaLocation"),
            format!("CMSIS-SVD_Schema_{}.xsd", self.schema_version),
        );

        match self.cpu {
            Some(ref v) => {
                elem.children.push(v.encode());
            }
            None => (),
        };
        elem.children.push(Element {
            name: String::from("peripherals"),
            attributes: HashMap::new(),
            children: self.peripherals.iter().map(Peripheral::encode).collect(),
            text: None,
        });

        elem
    }
}