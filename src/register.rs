extern crate xmltree;

use std::ops::Deref;

use xmltree::Element;

#[macro_use]
use elementext::*;

use helpers::*;
use registerinfo::*;
use registerarrayinfo::*;


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


impl ParseElem for Register {
    // TODO handle "clusters", return `Register` not an `Option`
    fn parse(tree: &Element) -> Register {
        assert_eq!(tree.name, "register");

        let info = RegisterInfo::parse(tree);

        if tree.get_child("dimIncrement").is_some() {
            let array_info = RegisterArrayInfo::parse(tree);
            assert!(info.name.contains("%s"));
            if let Some(ref indices) = array_info.dim_index {
                assert_eq!(array_info.dim as usize, indices.len())
            }
            Register::Array(info, array_info)
        } else {
            Register::Single(info)
        }
    }
}

impl EncodeElem for Register {
    fn encode(&self) -> Element {
        match *self {
            Register::Single(ref info) => info.encode(),
            Register::Array(ref info, ref array_info) => {
                // TODO: fix this (does not encode array stuff)
                // Not even slightly sure what to do here
                info.encode()
            }
        }
    }
}
