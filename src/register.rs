extern crate xmltree;

use std::ops::Deref;

use xmltree::Element;
use ElementExt;

use helpers::*;
use access::*;
use parse;
use Field;
use writeconstraint::*;


macro_rules! try {
    ($e:expr) => {
        $e.expect(concat!(file!(), ":", line!(), " ", stringify!($e)))
    }
}



#[derive(Clone, Debug)]
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
    _extensible: (),
}

#[derive(Clone, Debug)]
pub struct RegisterArrayInfo {
    pub dim: u32,
    pub dim_increment: u32,
    pub dim_index: Option<Vec<String>>,
}

#[derive(Clone, Debug)]
pub enum Register {
    Single(RegisterInfo),
    Array(RegisterInfo, RegisterArrayInfo),
}

impl Deref for Register {
    type Target = RegisterInfo;

    fn deref(&self) -> &RegisterInfo {
        match *self {
            Register::Single(ref info) => info,
            Register::Array(ref info, _) => info,
        }
    }
}

impl ParseElem for RegisterInfo {
    fn parse(tree: &Element) -> RegisterInfo {
        RegisterInfo {
            name: try!(tree.get_child_text("name")),
            description: try!(tree.get_child_text("description")),
            address_offset: {
                try!(parse::u32(try!(tree.get_child("addressOffset"))))
            },
            size: tree.get_child("size").map(|t| try!(parse::u32(t))),
            access: tree.get_child("access").map(Access::parse),
            reset_value:
                tree.get_child("resetValue").map(|t| try!(parse::u32(t))),
            reset_mask:
                tree.get_child("resetMask").map(|t| try!(parse::u32(t))),
            fields:
                tree.get_child("fields")
                    .map(|fs| fs.children.iter().map(Field::parse).collect()),
            write_constraint: tree.get_child("writeConstraint")
                .map(WriteConstraint::parse),
            _extensible: (),
        }
    }
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

impl Register {
    // TODO handle "clusters", return `Register` not an `Option`
    pub fn parse(tree: &Element) -> Option<Register> {
        assert_eq!(tree.name, "register");

        let info = RegisterInfo::parse(tree);

        if tree.get_child("dimIncrement").is_some() {
            let array_info = RegisterArrayInfo::parse(tree);
            assert!(info.name.contains("%s"));
            if let Some(ref indices) = array_info.dim_index {
                assert_eq!(array_info.dim as usize, indices.len())
            }
            Some(Register::Array(info, array_info))
        } else {
            Some(Register::Single(info))
        }
    }
}

