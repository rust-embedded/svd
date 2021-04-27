use core::ops::{Deref, DerefMut};

use xmltree::Element;

use crate::types::Parse;

use crate::elementext::ElementExt;

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

impl DerefMut for Field {
    fn deref_mut(&mut self) -> &mut FieldInfo {
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
            check_has_placeholder(&info.name, "field")?;
            if let Some(indices) = &array_info.dim_index {
                assert_eq!(array_info.dim as usize, indices.len())
            }
            Ok(Field::Array(info, array_info))
        } else {
            Ok(Field::Single(info))
        }
    }
}

impl Encode for Field {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        match self {
            Field::Single(info) => info.encode(),
            Field::Array(info, array_info) => {
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
    use crate::bitrange::{BitRange, BitRangeType};
    use crate::dimelement::DimElementBuilder;
    use crate::fieldinfo::FieldInfoBuilder;

    use crate::run_test;
    #[test]
    fn decode_encode() {
        let tests = vec![(
            Field::Array(
                FieldInfoBuilder::default()
                    .name("MODE%s".to_string())
                    .derived_from(Some("other_field".to_string()))
                    .bit_range(BitRange {
                        offset: 24,
                        width: 2,
                        range_type: BitRangeType::OffsetWidth,
                    })
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
            <field derivedFrom=\"other_field\">
              <name>MODE%s</name>
              <bitOffset>24</bitOffset>
              <bitWidth>2</bitWidth>
              <dim>2</dim>
              <dimIncrement>4</dimIncrement>
              <dimIndex>10,20</dimIndex>
            </field>
            ",
        )];
        run_test::<Field>(&tests[..]);
    }
}
