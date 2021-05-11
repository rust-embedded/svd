use crate::svd::{dimelement::DimElement, fieldinfo::FieldInfo};
use core::ops::{Deref, DerefMut};

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
