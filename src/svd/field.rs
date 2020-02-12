use core::ops::Deref;

use xmltree::Element;

use crate::types::Parse;

#[cfg(feature = "unproven")]
use crate::elementext::ElementExt;
#[cfg(feature = "unproven")]
use crate::encode::Encode;
use crate::error::*;
use crate::svd::{dimelement::DimElement, fieldinfo::FieldInfo};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum Field {
    Single(FieldInfo),
    Array(FieldInfo, DimElement),
}

impl Deref for Field {
    type Target = FieldInfo;

    fn deref(&self) -> &FieldInfo {
        match self {
            Field::Single(info) => info,
            Field::Array(info, _) => info,
        }
    }
}

impl Parse for Field {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        assert_eq!(tree.name, "field");

        let info = FieldInfo::parse(tree)?;

        if tree.get_child("dimIncrement").is_some() {
            let array_info = DimElement::parse(tree)?;
            assert!(info.name.contains("%s"));
            if let Some(indices) = &array_info.dim_index {
                assert_eq!(array_info.dim as usize, indices.len())
            }
            Ok(Self::Array(info, array_info))
        } else {
            Ok(Self::Single(info))
        }
    }
}

#[cfg(feature = "unproven")]
impl Encode for Field {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        match self {
            Field::Single(info) => info.encode(),
            Field::Array(info, array_info) => {
                // TODO: is this correct? probably not, need tests
                let base = info.encode()?;
                base.merge(&array_info.encode()?);
                Ok(base)
            }
        }
    }
}

// TODO: add Field encode and decode tests
