use super::{DimElement, Name};
use core::ops::{Deref, DerefMut};

/// A single SVD instance or array of instances
#[derive(Clone, Debug, PartialEq)]
pub enum SingleArray<T> {
    /// A single instance
    Single(T),
    /// An array of instances
    Array(T, DimElement),
}

impl<T> Deref for SingleArray<T> {
    type Target = T;

    fn deref(&self) -> &T {
        match self {
            Self::Single(info) => info,
            Self::Array(info, _) => info,
        }
    }
}

impl<T> DerefMut for SingleArray<T> {
    fn deref_mut(&mut self) -> &mut T {
        match self {
            Self::Single(info) => info,
            Self::Array(info, _) => info,
        }
    }
}

impl<T> SingleArray<T> {
    /// Return `true` if instance is single
    pub const fn is_single(&self) -> bool {
        matches!(self, Self::Single(_))
    }
    /// Return `true` if it is an array
    pub const fn is_array(&self) -> bool {
        matches!(self, Self::Array(_, _))
    }
}

impl<T> Name for SingleArray<T>
where
    T: Name,
{
    fn name(&self) -> &str {
        T::name(self)
    }
}

#[cfg(feature = "serde")]
mod ser_de {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(serde::Serialize)]
    struct SerArray<'a, T> {
        #[serde(flatten)]
        dim: &'a DimElement,
        #[serde(flatten)]
        info: &'a T,
    }

    #[derive(serde::Deserialize)]
    struct DeserArray<T> {
        #[serde(flatten, default)]
        dim: Option<DimElement>,
        #[serde(flatten)]
        info: T,
    }

    impl<T> Serialize for SingleArray<T>
    where
        T: Serialize,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Single(info) => info.serialize(serializer),
                Self::Array(info, dim) => SerArray::<T> { dim, info }.serialize(serializer),
            }
        }
    }

    impl<'de, T> Deserialize<'de> for SingleArray<T>
    where
        T: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let DeserArray { dim, info } = DeserArray::<T>::deserialize(deserializer)?;
            if let Some(dim) = dim {
                Ok(Self::Array(info, dim))
            } else {
                Ok(Self::Single(info))
            }
        }
    }
}
