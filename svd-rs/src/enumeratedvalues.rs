use super::{EnumeratedValue, SvdError, Usage};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct EnumeratedValues {
    /// Identifier for the whole enumeration section
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub name: Option<String>,

    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub usage: Option<Usage>,

    /// Makes a copy from a previously defined enumeratedValues section.
    /// No modifications are allowed
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub derived_from: Option<String>,

    pub values: Vec<EnumeratedValue>,
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("EnumeratedValues is empty")]
    Empty,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct EnumeratedValuesBuilder {
    name: Option<String>,
    usage: Option<Usage>,
    derived_from: Option<String>,
    values: Option<Vec<EnumeratedValue>>,
}

impl From<EnumeratedValues> for EnumeratedValuesBuilder {
    fn from(e: EnumeratedValues) -> Self {
        Self {
            name: e.name,
            usage: e.usage,
            derived_from: e.derived_from,
            values: Some(e.values),
        }
    }
}

impl EnumeratedValuesBuilder {
    pub fn name(mut self, value: Option<String>) -> Self {
        self.name = value;
        self
    }
    pub fn usage(mut self, value: Option<Usage>) -> Self {
        self.usage = value;
        self
    }
    pub fn derived_from(mut self, value: Option<String>) -> Self {
        self.derived_from = value;
        self
    }
    pub fn values(mut self, value: Vec<EnumeratedValue>) -> Self {
        self.values = Some(value);
        self
    }
    pub fn build(self) -> Result<EnumeratedValues, SvdError> {
        (EnumeratedValues {
            name: self.name,
            usage: self.usage,
            derived_from: self.derived_from,
            values: self.values.unwrap_or_default(),
        })
        .validate()
    }
}

impl EnumeratedValues {
    pub fn builder() -> EnumeratedValuesBuilder {
        EnumeratedValuesBuilder::default()
    }
    fn validate(self) -> Result<Self, SvdError> {
        #[cfg(feature = "strict")]
        {
            if let Some(name) = self.name.as_ref() {
                super::check_name(name, "name")?;
            }
        }
        if let Some(_dname) = self.derived_from.as_ref() {
            #[cfg(feature = "strict")]
            super::check_derived_name(_dname, "derivedFrom")?;
            Ok(self)
        } else if self.values.is_empty() {
            Err(Error::Empty.into())
        } else {
            Ok(self)
        }
    }
    pub(crate) fn check_range(&self, range: core::ops::Range<u64>) -> Result<(), SvdError> {
        for v in self.values.iter() {
            v.check_range(&range)?;
        }
        Ok(())
    }
}
