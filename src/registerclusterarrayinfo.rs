extern crate xmltree;
extern crate either;

use std::ops::Deref;

use xmltree::Element;
use either::Either;

use ElementExt;

use helpers::*;
use register::*;
use cluster::*;
use parse;

macro_rules! try {
    ($e:expr) => {
        $e.expect(concat!(file!(), ":", line!(), " ", stringify!($e)))
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct RegisterClusterArrayInfo {
    pub dim: u32,
    pub dim_increment: u32,
    pub dim_index: Option<Vec<String>>,
}

impl ParseElem for RegisterClusterArrayInfo {
    fn parse(tree: &Element) -> RegisterClusterArrayInfo {
        RegisterClusterArrayInfo {
            dim: try!(tree.get_child_text("dim").unwrap().parse::<u32>()),
            dim_increment: try!(tree.get_child("dimIncrement").map(|t| {
                try!(parse::u32(t))
            })),
            dim_index: tree.get_child("dimIndex").map(|c| {
                parse::dim_index(try!(c.text.as_ref()))
            }),
        }
    }
}

