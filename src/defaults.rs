extern crate xmltree;

use xmltree::Element;

use helpers::*;
use access::*;
use parse;

macro_rules! try {
    ($e:expr) => {
        $e.expect(concat!(file!(), ":", line!(), " ", stringify!($e)))
    }
}

/// Register default properties
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Defaults {
    pub size: Option<u32>,
    pub reset_value: Option<u32>,
    pub reset_mask: Option<u32>,
    pub access: Option<Access>,
    // Reserve the right to add more fields to this struct
    _extensible: (),
}

impl ParseElem for Defaults {
    fn parse(tree: &Element) -> Defaults {
        Defaults {
            size: tree.get_child("size").map(|t| try!(parse::u32(t))),
            reset_value: tree.get_child("resetValue").map(|t| try!(parse::u32(t))),
            reset_mask: tree.get_child("resetMask").map(|t| try!(parse::u32(t))),
            access: tree.get_child("access").map(Access::parse),
            _extensible: (),
        }
    }
}

impl EncodeChildren for Defaults {
    fn encode_children(&self) -> Vec<Element> {
        let mut children = Vec::new();

        match self.size {
            Some(ref v) => {
                children.push(new_element("size", Some(format!("0x{:08.x}", v))));
            }
            None => (),
        };

        match self.reset_value {
            Some(ref v) => {
                children.push(new_element("resetValue", Some(format!("0x{:08.x}", v))));
            }
            None => (),
        };

        match self.reset_mask {
            Some(ref v) => {
                children.push(new_element("resetMask", Some(format!("0x{:08.x}", v))));
            }
            None => (),
        };

        match self.access {
            Some(ref v) => {
                children.push(v.encode());
            }
            None => (),
        };

        children
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_encode() {
        let example = String::from(
            "
            <mock>
                <size>0xaabbccdd</size>
                <resetValue>0x11223344</resetValue>
                <resetMask>0x00000000</resetMask>
                <access>read-only</access>
            </mock>
        ",
        );

        let expected = Defaults {
            size: Some(0xaabbccdd),
            reset_value: Some(0x11223344),
            reset_mask: Some(0x00000000),
            access: Some(Access::ReadOnly),
            _extensible: (),
        };

        let tree1 = &try!(Element::parse(example.as_bytes()));

        let parsed = Defaults::parse(tree1);
        assert_eq!(parsed, expected, "Parsing tree failed");

        let mut tree2 = new_element("mock", None);
        tree2.children = parsed.encode_children();
        assert_eq!(tree1, &tree2, "Encoding value failed");
    }
}
