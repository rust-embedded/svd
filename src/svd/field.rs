use crate::svd::{dimelement::DimElement, fieldinfo::FieldInfo};
use core::ops::{Deref, DerefMut};

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

#[cfg(feature = "serde")]
mod ser_de {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(serde::Deserialize, serde::Serialize)]
    struct FieldArray {
        #[cfg_attr(feature = "serde", serde(flatten))]
        #[cfg_attr(feature = "serde", serde(default))]
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        dim: Option<DimElement>,
        #[cfg_attr(feature = "serde", serde(flatten))]
        info: FieldInfo,
    }

    impl Serialize for Field {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Field::Single(info) => info.serialize(serializer),
                Field::Array(info, dim) => FieldArray {
                    dim: Some(dim.clone()),
                    info: info.clone(),
                }
                .serialize(serializer),
            }
        }
    }

    impl<'de> Deserialize<'de> for Field {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let FieldArray { dim, info } = FieldArray::deserialize(deserializer)?;
            if let Some(dim) = dim {
                Ok(Field::Array(info, dim))
            } else {
                Ok(Field::Single(info))
            }
        }
    }
}
