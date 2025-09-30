use super::{BuildError, EmptyToNone, EnumeratedValue, SvdError, ValidateLevel};
use std::borrow::Cow;
use std::ops::RangeInclusive;

/// Defines arrays and lists.
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "camelCase")
)]
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct DimElement {
    /// Defines the number of elements in an array or list
    pub dim: u32,

    /// Specify the address increment between two neighboring array or list members in the address map
    pub dim_increment: u32,

    /// Specify the strings that substitue the placeholder `%s` within `name` and `displayName`.
    /// By default, <dimIndex> is a value starting with 0
    #[cfg_attr(
        feature = "serde",
        serde(
            deserialize_with = "ser_de::deserialize_dim_index",
            serialize_with = "ser_de::serialize_dim_index",
            skip_serializing_if = "Option::is_none"
        )
    )]
    pub dim_index: Option<Vec<String>>,

    /// Specify the name of the structure. If not defined, then the entry of the `name` element is used
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub dim_name: Option<String>,

    /// Grouping element to create enumerations in the header file
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub dim_array_index: Option<DimArrayIndex>,
}

/// Grouping element to create enumerations in the header file
///
/// This information is used for generating an enum in the device header file.
/// The debugger may use this information to display the identifier string
/// as well as the description. Just like symbolic constants making source
/// code more readable, the system view in the debugger becomes more instructive
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "camelCase")
)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DimArrayIndex {
    /// Specify the base name of enumerations
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub header_enum_name: Option<String>,

    /// Specify the values contained in the enumeration
    pub values: Vec<EnumeratedValue>,
}

/// Builder for [`DimElement`]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DimElementBuilder {
    dim: Option<u32>,
    dim_increment: Option<u32>,
    dim_index: Option<Vec<String>>,
    dim_name: Option<String>,
    dim_array_index: Option<DimArrayIndex>,
}

impl From<DimElement> for DimElementBuilder {
    fn from(d: DimElement) -> Self {
        Self {
            dim: Some(d.dim),
            dim_increment: Some(d.dim_increment),
            dim_index: d.dim_index,
            dim_name: d.dim_name,
            dim_array_index: d.dim_array_index,
        }
    }
}

impl DimElementBuilder {
    /// Set the dim of the elements
    pub fn dim(mut self, value: u32) -> Self {
        self.dim = Some(value);
        self
    }
    /// Set the dim increment of the elements
    pub fn dim_increment(mut self, value: u32) -> Self {
        self.dim_increment = Some(value);
        self
    }
    /// Set the dim index of the elements
    pub fn dim_index(mut self, value: Option<Vec<String>>) -> Self {
        self.dim_index = value;
        self
    }
    /// Set the dim name of the elements
    pub fn dim_name(mut self, value: Option<String>) -> Self {
        self.dim_name = value;
        self
    }
    /// Set the dim_array_index of the elements
    pub fn dim_array_index(mut self, value: Option<DimArrayIndex>) -> Self {
        self.dim_array_index = value;
        self
    }
    /// Validate and build a [`DimElement`].
    pub fn build(self, lvl: ValidateLevel) -> Result<DimElement, SvdError> {
        let de = DimElement {
            dim: self
                .dim
                .ok_or_else(|| BuildError::Uninitialized("dim".to_string()))?,
            dim_increment: self
                .dim_increment
                .ok_or_else(|| BuildError::Uninitialized("dim_increment".to_string()))?,
            dim_index: self.dim_index.empty_to_none(),
            dim_name: self.dim_name.empty_to_none(),
            dim_array_index: self.dim_array_index,
        };
        de.validate(lvl)?;
        Ok(de)
    }
}

impl DimElement {
    /// Make a builder for [`DimElement`]
    pub fn builder() -> DimElementBuilder {
        DimElementBuilder::default()
    }

    /// Get array of indexes from string
    pub fn parse_indexes(text: &str) -> Option<Vec<String>> {
        (if text.contains('-') {
            let (start, end) = text.split_once('-')?;
            if let (Ok(start), Ok(end)) = (start.parse::<u32>(), end.parse::<u32>()) {
                Some((start..=end).map(|i| i.to_string()).collect::<Vec<_>>())
            } else {
                let mut start = start.bytes();
                let mut end = end.bytes();
                match (start.next(), start.next(), end.next(), end.next()) {
                    (Some(start), None, Some(end), None)
                        if (start.is_ascii_lowercase() && end.is_ascii_lowercase())
                            || (start.is_ascii_uppercase() && end.is_ascii_uppercase()) =>
                    {
                        Some((start..=end).map(|c| char::from(c).to_string()).collect())
                    }
                    _ => None,
                }
            }
        } else {
            Some(text.split(',').map(|s| s.to_string()).collect())
        })
        .filter(|v| !v.is_empty())
    }
    /// Try to represent [`DimElement`] as range of integer indexes
    pub fn indexes_as_range(&self) -> Option<RangeInclusive<u32>> {
        let mut integers = Vec::with_capacity(self.dim as usize);
        for idx in self.indexes() {
            // XXX: indexes that begin with leading zero are not compatible with range (`0-x`) syntax in serialization
            // see https://github.com/rust-embedded/svdtools/pull/178#issuecomment-1801433808
            let val = idx.parse::<u32>().ok()?;
            if val.to_string() != idx {
                return None;
            }
            integers.push(val);
        }
        let min = *integers.iter().min()?;
        let max = *integers.iter().max()?;
        if max - min + 1 != self.dim {
            return None;
        }
        for (&i, r) in integers.iter().zip(min..=max) {
            if i != r {
                return None;
            }
        }
        Some(min..=max)
    }
    /// Modify an existing [`DimElement`] based on a [builder](DimElementBuilder).
    pub fn modify_from(
        &mut self,
        builder: DimElementBuilder,
        lvl: ValidateLevel,
    ) -> Result<(), SvdError> {
        if let Some(dim) = builder.dim {
            self.dim = dim;
        }
        if let Some(dim_increment) = builder.dim_increment {
            self.dim_increment = dim_increment;
        }
        if builder.dim_index.is_some() {
            self.dim_index = builder.dim_index.empty_to_none();
        }
        if builder.dim_name.is_some() {
            self.dim_name = builder.dim_name.empty_to_none();
        }
        if builder.dim_array_index.is_some() {
            self.dim_array_index = builder.dim_array_index;
        }
        self.validate(lvl)
    }
    /// Validate the [`DimElement`].
    ///
    /// # Notes
    ///
    /// This doesn't do anything.
    pub fn validate(&self, _lvl: ValidateLevel) -> Result<(), SvdError> {
        // TODO
        Ok(())
    }
    /// Get the indexes of the array or list.
    pub fn indexes(&self) -> Indexes<'_> {
        Indexes {
            i: 0,
            dim: self.dim,
            dim_index: &self.dim_index,
        }
    }
}

/// Indexes into a [DimElement]
pub struct Indexes<'a> {
    i: u32,
    dim: u32,
    dim_index: &'a Option<Vec<String>>,
}

impl<'a> Iterator for Indexes<'a> {
    type Item = Cow<'a, str>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.dim {
            return None;
        }
        let i = self.i;
        self.i += 1;
        if let Some(index) = self.dim_index.as_ref() {
            Some(index[i as usize].as_str().into())
        } else {
            Some(i.to_string().into())
        }
    }
}

#[cfg(feature = "serde")]
mod ser_de {
    use super::*;
    use serde::{de, Deserialize, Deserializer, Serializer};
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(untagged)]
    enum DimIndex {
        Array(Vec<String>),
        String(String),
    }

    pub fn deserialize_dim_index<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(match Option::<DimIndex>::deserialize(deserializer)? {
            None => None,
            Some(DimIndex::Array(a)) => Some(a),
            Some(DimIndex::String(s)) => Some(
                DimElement::parse_indexes(&s)
                    .ok_or_else(|| de::Error::custom("Failed to deserialize dimIndex"))?,
            ),
        })
    }

    pub fn serialize_dim_index<S>(val: &Option<Vec<String>>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_str(&val.as_ref().unwrap().join(","))
    }
}
