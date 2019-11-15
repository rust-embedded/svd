use xmltree::Element;

#[cfg(feature = "unproven")]
use crate::encode::Encode;
#[cfg(feature = "unproven")]
use crate::encode::EncodeChildren;
use crate::error::*;
#[cfg(feature = "unproven")]
use crate::new_element;
use crate::parse;
use crate::types::Parse;

use crate::svd::access::Access;

/// Register default properties
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RegisterProperties {
    /// Default bit-width of any register
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub size: Option<u32>,

    /// Default value for all registers at RESET
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub reset_value: Option<u32>,

    /// Define which register bits have a defined reset value
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub reset_mask: Option<u32>,

    /// Default access rights for all registers
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub access: Option<Access>,

    // Reserve the right to add more fields to this struct
    #[cfg_attr(feature = "serde", serde(skip))]
    _extensible: (),
}

impl Parse for RegisterProperties {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        let mut p = RegisterProperties::default();
        p.size = parse::optional::<u32>("size", tree)?;
        p.reset_value = parse::optional::<u32>("resetValue", tree)?;
        p.reset_mask = parse::optional::<u32>("resetMask", tree)?;
        p.access = parse::optional::<Access>("access", tree)?;
        Ok(p)
    }
}

#[cfg(feature = "unproven")]
impl EncodeChildren for RegisterProperties {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Vec<Element>> {
        let mut children = Vec::new();

        if let Some(v) = &self.size {
            children.push(new_element("size", Some(format!("0x{:08.x}", v))));
        };

        if let Some(v) = &self.reset_value {
            children.push(new_element("resetValue", Some(format!("0x{:08.x}", v))));
        };

        if let Some(v) = &self.reset_mask {
            children.push(new_element("resetMask", Some(format!("0x{:08.x}", v))));
        };

        if let Some(v) = &self.access {
            children.push(v.encode()?);
        };

        Ok(children)
    }
}

#[cfg(test)]
#[cfg(feature = "unproven")]
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

        let mut expected = RegisterProperties::default();
        expected.size = Some(0xaabbccdd);
        expected.reset_value = Some(0x11223344);
        expected.reset_mask = Some(0x00000000);
        expected.access = Some(Access::ReadOnly);

        let tree1 = Element::parse(example.as_bytes()).unwrap();

        let parsed = RegisterProperties::parse(&tree1).unwrap();
        assert_eq!(parsed, expected, "Parsing tree failed");

        let mut tree2 = new_element("mock", None);
        tree2.children = parsed.encode().unwrap();
        assert_eq!(tree1, tree2, "Encoding value failed");
    }
}
