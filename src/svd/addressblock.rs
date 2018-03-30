
use std::collections::HashMap;

use xmltree::Element;
use ElementExt;

use types::{Parse, Encode, new_element};
use error::SVDError;

#[derive(Clone, Debug, PartialEq)]
pub struct AddressBlock {
    pub offset: u32,
    pub size: u32,
    pub usage: String,
}

impl Parse for AddressBlock {
    type Object = AddressBlock;
    type Error = SVDError;

    fn parse(tree: &Element) -> Result<AddressBlock, SVDError> {
        Ok(AddressBlock {
            offset: tree.get_child_u32("offset")?,
            size: tree.get_child_u32("size")?,
            usage: tree.get_child_text("usage")?,
        })
    }
}

impl Encode for AddressBlock {
    type Error = SVDError;

    fn encode(&self) -> Result<Element, SVDError> {
        Ok(Element {
            name: String::from("addressBlock"),
            attributes: HashMap::new(),
            children: vec![
                new_element("offset", Some(format!("{}", self.offset))),
                new_element("size", Some(format!("0x{:08.x}", self.size))),
                new_element("usage", Some(self.usage.clone())),
            ],
            text: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::test;


    #[test]
    fn decode_encode() {
        let tests = vec![
            (
                AddressBlock {
                    offset: 0,
                    size: 0x00000800,
                    usage: String::from("registers"),
                },
                "<addressBlock>
                    <offset>0</offset>
                    <size>0x00000800</size>
                    <usage>registers</usage>
                </addressBlock>"
            ),
        ];

        test::<AddressBlock>(&tests[..]);
    }
}
