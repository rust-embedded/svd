#[cfg(feature = "unproven")]
use std::collections::HashMap;

use crate::elementext::ElementExt;
use xmltree::Element;

use crate::types::Parse;

#[cfg(feature = "unproven")]
use crate::encode::Encode;
use crate::error::SVDError;
#[cfg(feature = "unproven")]
use crate::new_element;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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
            prefix: None,
            namespace: None,
            namespaces: None,
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
    use crate::run_test;

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
