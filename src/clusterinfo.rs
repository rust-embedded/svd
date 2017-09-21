extern crate xmltree;
extern crate either;

use xmltree::Element;
use either::Either;

use ElementExt;

use helpers::*;
use parse;
use access::*;
use register::*;
use cluster::*;
use registercluster::*;

macro_rules! try {
    ($e:expr) => {
        $e.expect(concat!(file!(), ":", line!(), " ", stringify!($e)))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClusterInfo {
    pub name: String,
    pub description: String,
    pub header_struct_name: String,
    pub address_offset: u32,
    pub size: Option<u32>,
    pub access: Option<Access>,
    pub reset_value: Option<u32>,
    pub reset_mask: Option<u32>,
    pub children: Vec<Either<Register, Cluster>>,
    // Reserve the right to add more fields to this struct
    _extensible: (),
}

impl ParseElem for ClusterInfo {
    fn parse(tree: &Element) -> ClusterInfo {
        ClusterInfo {
            name: try!(tree.get_child_text("name")),
            description: try!(tree.get_child_text("description")),
            header_struct_name: try!(tree.get_child_text("headerStructName")),
            address_offset: {
                try!(parse::u32(try!(tree.get_child("addressOffset"))))
            },
            size: tree.get_child("size").map(|t| try!(parse::u32(t))),
            access: tree.get_child("access").map(Access::parse),
            reset_value: tree.get_child("resetValue").map(|t| try!(parse::u32(t))),
            reset_mask: tree.get_child("resetMask").map(|t| try!(parse::u32(t))),
            children: tree.children
                .iter()
                .filter(|t| t.name == "register" || t.name == "cluster")
                .map(cluster_register_parse)
                .collect(),
            _extensible: (),
        }
    }
}

