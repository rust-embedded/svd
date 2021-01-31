use crate::elementext::ElementExt;
use crate::NS;
use minidom::Element;

use crate::types::Parse;

use crate::encode::Encode;
use crate::error::*;
use crate::new_element;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct AddressBlock {
    pub offset: u32,
    pub size: u32,
    pub usage: String,
}

impl Parse for AddressBlock {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        Ok(Self {
            offset: tree.get_child_u32("offset")?,
            size: tree.get_child_u32("size")?,
            usage: tree.get_child_text("usage")?,
        })
    }
}

impl Encode for AddressBlock {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        Ok(Element::builder("addressBlock", NS)
            .append(new_element("offset", Some(format!("{}", self.offset))))
            .append(new_element("size", Some(format!("0x{:08.x}", self.size))))
            .append(new_element("usage", Some(self.usage.clone())))
            .build())
    }
}

#[cfg(test)]
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
