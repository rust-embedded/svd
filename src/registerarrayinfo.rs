extern crate xmltree;

use std::collections::HashMap;

use xmltree::Element;
use ElementExt;

use parse;
use helpers::*;

macro_rules! try {
    ($e:expr) => {
        $e.expect(concat!(file!(), ":", line!(), " ", stringify!($e)))
    }
}

#[derive(Clone, Debug)]
pub struct RegisterArrayInfo {
    pub dim: u32,
    pub dim_increment: u32,
    pub dim_index: Option<Vec<String>>,
}

impl ParseElem for RegisterArrayInfo {
    fn parse(tree: &Element) -> RegisterArrayInfo {
        RegisterArrayInfo {
            dim: try!(tree.get_child_text("dim").unwrap().parse::<u32>()),
            dim_increment: try!(
                tree.get_child("dimIncrement")
                    .map(|t| try!(parse::u32(t)))
            ),
            dim_index: tree.get_child("dimIndex")
                .map(|c| parse::dim_index(try!(c.text.as_ref()))),
        }
    }
}

impl EncodeElem for RegisterArrayInfo {
    fn encode(&self) -> Element {
        Element{
            name: String::from("NOPE"),
            attributes: HashMap::new(),
            children: Vec::new(),
            text: None,
        }
    }
}

