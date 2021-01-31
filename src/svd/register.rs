use core::ops::Deref;

use crate::NS;
use minidom::Element;

use crate::types::Parse;

use crate::elementext::ElementExt;

use crate::encode::Encode;
use crate::error::*;
use crate::svd::{dimelement::DimElement, registerinfo::RegisterInfo};
use anyhow::Result;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum Register {
    Single(RegisterInfo),
    Array(RegisterInfo, DimElement),
}

impl Deref for Register {
    type Target = RegisterInfo;

    fn deref(&self) -> &RegisterInfo {
        match self {
            Register::Single(info) => info,
            Register::Array(info, _) => info,
        }
    }
}

impl Parse for Register {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        assert_eq!(tree.name(), "register");

        let info = RegisterInfo::parse(tree)?;

        if tree.get_child("dimIncrement", NS).is_some() {
            let array_info = DimElement::parse(tree)?;
            check_has_placeholder(&info.name, "register")?;
            if let Some(indices) = &array_info.dim_index {
                assert_eq!(array_info.dim as usize, indices.len())
            }
            Ok(Register::Array(info, array_info))
        } else {
            Ok(Register::Single(info))
        }
    }
}

impl Encode for Register {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        match self {
            Register::Single(info) => info.encode(),
            Register::Array(info, array_info) => {
                // TODO: is this correct? probably not, need tests
                let mut base = info.encode()?;
                base.merge(&array_info.encode()?);
                Ok(base)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dimelement::DimElementBuilder;
    use crate::registerinfo::RegisterInfoBuilder;

    use crate::run_test;
    #[test]
    fn decode_encode() {
        let tests = vec![(
            Register::Array(
                RegisterInfoBuilder::default()
                    .name("MODE%s".to_string())
                    .address_offset(8)
                    .build()
                    .unwrap(),
                DimElementBuilder::default()
                    .dim(2)
                    .dim_increment(4)
                    .dim_index(Some(vec!["10".to_string(), "20".to_string()]))
                    .build()
                    .unwrap(),
            ),
            "
            <register>
              <name>MODE%s</name>
              <addressOffset>0x8</addressOffset>
              <dim>2</dim>
              <dimIncrement>4</dimIncrement>
              <dimIndex>10,20</dimIndex>
            </register>
            ",
        )];
        run_test::<Register>(&tests[..]);
    }
}
