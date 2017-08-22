extern crate xmltree;

use std::collections::HashMap;

use xmltree::Element;
use ElementExt;

use helpers::*;

macro_rules! try {
    ($e:expr) => {
        $e.expect(concat!(file!(), ":", line!(), " ", stringify!($e)))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WriteConstraintRange {
    pub min: u32,
    pub max: u32,
}

impl ParseElem for WriteConstraintRange {
    fn parse(tree: &Element) -> WriteConstraintRange {
        WriteConstraintRange {
            min: try!(try!(tree.get_child_text("minimum")).parse()),
            max: try!(try!(tree.get_child_text("maximum")).parse()),
        }
    }
}

impl EncodeElem for WriteConstraintRange {
    fn encode(&self) -> Element {
        Element {
            name: String::from("range"),
            attributes: HashMap::new(),
            children: vec![
                new_element("min", Some(format!("0x{:08.x}", self.min))),
                new_element("max", Some(format!("0x{:08.x}", self.max))),
            ],
            text: None,
        }
    }
}
