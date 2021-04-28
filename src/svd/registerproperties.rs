use xmltree::Element;

use crate::encode::Encode;
use crate::encode::EncodeChildren;
use crate::error::*;

use crate::new_element;
use crate::parse;
use crate::types::Parse;

use crate::svd::access::Access;

/// Register default properties
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct RegisterProperties {
    /// Bit-width of register
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub size: Option<u32>,

    /// Access rights for register
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub access: Option<Access>,

    /// Register value at RESET
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub reset_value: Option<u64>,

    /// Define which register bits have a defined reset value
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub reset_mask: Option<u64>,
}

impl Parse for RegisterProperties {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        let p = RegisterProperties {
            size: parse::optional::<u32>("size", tree)?,
            access: parse::optional::<Access>("access", tree)?,
            reset_value: parse::optional::<u64>("resetValue", tree)?,
            reset_mask: parse::optional::<u64>("resetMask", tree)?,
        };
        check_reset_value(p.size, p.reset_value, p.reset_mask)?;
        Ok(p)
    }
}

impl EncodeChildren for RegisterProperties {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Vec<Element>> {
        let mut children = Vec::new();

        if let Some(v) = &self.size {
            children.push(new_element("size", Some(format!("{}", v))));
        };

        if let Some(v) = &self.access {
            children.push(v.encode()?);
        };

        if let Some(v) = &self.reset_value {
            children.push(new_element(
                "resetValue",
                Some(if *v > u32::MAX as u64 {
                    format!("0x{:016X}", v)
                } else {
                    format!("0x{:08X}", v)
                }),
            ));
        };

        if let Some(v) = &self.reset_mask {
            children.push(new_element(
                "resetMask",
                Some(if *v > u32::MAX as u64 {
                    format!("0x{:016X}", v)
                } else {
                    format!("0x{:08X}", v)
                }),
            ));
        };

        Ok(children)
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
                <size>64</size>
                <access>read-only</access>
                <resetValue>0x11223344</resetValue>
                <resetMask>0xFFFFFFFF</resetMask>
            </mock>
        ",
        );

        let mut expected = RegisterProperties::default();
        expected.size = Some(64);
        expected.reset_value = Some(0x11223344);
        expected.reset_mask = Some(0xffffffff);
        expected.access = Some(Access::ReadOnly);

        let tree1 = Element::parse(example.as_bytes()).unwrap();

        let parsed = RegisterProperties::parse(&tree1).unwrap();
        assert_eq!(parsed, expected, "Parsing tree failed");

        let mut tree2 = new_element("mock", None);
        tree2.children = parsed.encode().unwrap();
        assert_eq!(tree1, tree2, "Encoding value failed");
    }
}
