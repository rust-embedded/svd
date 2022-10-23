use super::{DimElement, Name};
use core::ops::{Deref, DerefMut};

/// A single SVD instance or array of instances
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MaybeArray<T> {
    /// A single instance
    Single(T),
    /// An array of instances
    Array(T, DimElement),
}

impl<T> Deref for MaybeArray<T> {
    type Target = T;

    fn deref(&self) -> &T {
        match self {
            Self::Single(info) => info,
            Self::Array(info, _) => info,
        }
    }
}

impl<T> DerefMut for MaybeArray<T> {
    fn deref_mut(&mut self) -> &mut T {
        match self {
            Self::Single(info) => info,
            Self::Array(info, _) => info,
        }
    }
}

impl<T> MaybeArray<T> {
    /// Return `true` if instance is single
    pub const fn is_single(&self) -> bool {
        matches!(self, Self::Single(_))
    }
    /// Return `true` if it is an array
    pub const fn is_array(&self) -> bool {
        matches!(self, Self::Array(_, _))
    }
}

impl<T> Name for MaybeArray<T>
where
    T: Name,
{
    fn name(&self) -> &str {
        T::name(self)
    }
}

/// Return list of names of instances in array
pub fn names<'a, T: Name>(info: &'a T, dim: &'a DimElement) -> impl Iterator<Item = String> + 'a {
    let name = info.name();
    dim.indexes()
        .map(move |i| name.replace("[%s]", &i).replace("%s", &i))
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

    impl<T> Serialize for MaybeArray<T>
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

    impl<'de, T> Deserialize<'de> for MaybeArray<T>
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
