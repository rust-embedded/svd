#[cfg(feature = "unproven")]
use std::collections::HashMap;

use elementext::ElementExt;
use xmltree::Element;

use types::Parse;

#[cfg(feature = "unproven")]
use encode::Encode;
use error::SVDError;
#[cfg(feature = "unproven")]
use new_element;

#[cfg(feature = "serde_svd")]
use super::serde::{ Deserialize, Serialize };

#[cfg_attr(feature = "serde_svd", derive(Deserialize, Serialize))]
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

#[cfg(feature = "unproven")]
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
#[cfg(feature = "unproven")]
mod tests {
    use super::*;
    use run_test;

    #[test]
    fn decode_encode() {
        let tests = vec![(
            AddressBlock {
                offset: 0,
                size: 0x00000800,
                usage: String::from("registers"),
            },
            "<addressBlock>
                    <offset>0</offset>
                    <size>0x00000800</size>
                    <usage>registers</usage>
                </addressBlock>",
        )];

        run_test::<AddressBlock>(&tests[..]);
    }
}
