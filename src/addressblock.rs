
use std::collections::HashMap;

use xmltree::Element;


use elementext::*;

use helpers::*;
use parse;


#[derive(Clone, Debug, PartialEq)]
pub struct AddressBlock {
    pub offset: u32,
    pub size: u32,
    pub usage: String,
}

impl ParseElem for AddressBlock {
    fn parse(tree: &Element) -> AddressBlock {
        AddressBlock {
            offset: try_get_child!(parse::u32(try_get_child!(tree.get_child("offset")))),
            size: try_get_child!(parse::u32(try_get_child!(tree.get_child("size")))),
            usage: try_get_child!(tree.get_child_text("usage")),
        }
    }
}

impl EncodeElem for AddressBlock {
    fn encode(&self) -> Element {
        Element {
            name: String::from("addressBlock"),
            attributes: HashMap::new(),
            children: vec![
                new_element("offset", Some(format!("{}", self.offset))),
                new_element("size", Some(format!("0x{:08.x}", self.size))),
                new_element("usage", Some(self.usage.clone())),
            ],
            text: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_encode() {
        let types = vec![
            (
                AddressBlock {
                    offset: 0,
                    size: 0x00000800,
                    usage: String::from("registers"),
                },
                String::from(
                    "<addressBlock>
                        <offset>0</offset>
                        <size>0x00000800</size>
                        <usage>registers</usage>
                    </addressBlock>",
                )
            ),
        ];

        for (a, s) in types {
            let tree1 = &try_get_child!(Element::parse(s.as_bytes()));
            let v = AddressBlock::parse(tree1);
            assert_eq!(v, a, "Parsing `{}` expected `{:?}`", s, a);
            let tree2 = &v.encode();
            assert_eq!(tree1, tree2, "Encoding {:?} expected {}", a, s);
        }
    }
}
