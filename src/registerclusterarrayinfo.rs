extern crate xmltree;
extern crate either;

use xmltree::Element;

#[macro_use]
use elementext::*;

use helpers::*;
use parse;


#[derive(Clone, Debug, PartialEq)]
pub struct RegisterClusterArrayInfo {
    pub dim: u32,
    pub dim_increment: u32,
    pub dim_index: Option<Vec<String>>,
}

impl ParseElem for RegisterClusterArrayInfo {
    fn parse(tree: &Element) -> RegisterClusterArrayInfo {
        RegisterClusterArrayInfo {
            dim: try_get_child!(tree.get_child_text("dim").unwrap().parse::<u32>()),
            dim_increment: try_get_child!(tree.get_child("dimIncrement").map(|t| {
                try_get_child!(parse::u32(t))
            })),
            dim_index: tree.get_child("dimIndex").map(|c| {
                parse::dim_index(try_get_child!(c.text.as_ref()))
            }),
        }
    }
}

