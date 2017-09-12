use xmltree::Element;

use std::collections::HashMap;

use helpers::*;
use ElementExt;
use cpu::*;
use defaults::*;
use peripheral::*;

macro_rules! try {
    ($e:expr) => {
        $e.expect(concat!(file!(), ":", line!(), " ", stringify!($e)))
    }
}

#[derive(Clone, Debug)]
pub struct Device {
    pub name: String,
    pub cpu: Option<Cpu>,
    pub peripherals: Vec<Peripheral>,
    pub defaults: Defaults,
    // Reserve the right to add more fields to this struct
    _extensible: (),
}

impl ParseElem for Device {
    fn parse(tree: &Element) -> Device {
        Device {
            name: try!(tree.get_child_text("name")),
            cpu: tree.get_child("cpu").map(Cpu::parse),
            peripherals: try!(tree.get_child("peripherals"))
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
            children: vec![new_element("name", Some(self.name.clone()))],
            text: None,
        };

        elem.attributes.insert(String::from("xmlns:xs"), String::from("http://www.w3.org/2001/XMLSchema-instance"));
        elem.attributes.insert(String::from("schemaVersion"), String::from("1.0"));
        elem.attributes.insert(String::from("xs:noNamespaceSchemaLocation"), String::from("CMSIS-SVD_Schema_1.0.xsd"));

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
