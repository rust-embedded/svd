use super::{EmptyToNone, EnumeratedValue, SvdError, Usage, ValidateLevel};

/// A map describing unsigned integers and their description and name.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct EnumeratedValues {
    /// Identifier for the whole enumeration section
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub name: Option<String>,

    /// Usage of the values
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub usage: Option<Usage>,

    /// Makes a copy from a previously defined enumeratedValues section.
    /// No modifications are allowed
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub derived_from: Option<String>,

    /// List of variants. The number of required items depends on the bit-width of the associated field.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub values: Vec<EnumeratedValue>,
}

/// Errors for [`EnumeratedValues::validate`]
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// Enum is empty
    #[error("EnumeratedValues is empty")]
    Empty,
}

/// Builder for [`EnumeratedValues`]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
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
    /// Set the name of the enumerated values
    pub fn name(mut self, value: Option<String>) -> Self {
        self.name = value;
        self
    }
    /// Set the usage of the enumerated values
    pub fn usage(mut self, value: Option<Usage>) -> Self {
        self.usage = value;
        self
    }
    /// Set the derived_from attribute for the enumerated values
    pub fn derived_from(mut self, value: Option<String>) -> Self {
        self.derived_from = value;
        self
    }
    /// Set the values
    pub fn values(mut self, value: Vec<EnumeratedValue>) -> Self {
        self.values = Some(value);
        self
    }
    /// Validate and build a [`EnumeratedValues`].
    pub fn build(self, lvl: ValidateLevel) -> Result<EnumeratedValues, SvdError> {
        let evs = EnumeratedValues {
            name: self.name.empty_to_none(),
            usage: self.usage,
            derived_from: self.derived_from,
            values: self.values.unwrap_or_default(),
        };
        evs.validate(lvl)?;
        Ok(evs)
    }
}

impl EnumeratedValues {
    /// Return default value if present
    pub fn default_value(&self) -> Option<&EnumeratedValue> {
        for v in &self.values {
            if v.is_default() {
                return Some(v);
            }
        }
        None
    }

    /// Make a builder for [`EnumeratedValues`]
    pub fn builder() -> EnumeratedValuesBuilder {
        EnumeratedValuesBuilder::default()
    }
    /// Modify an existing [`EnumeratedValues`] based on a [builder](EnumeratedValuesBuilder).
    pub fn modify_from(
        &mut self,
        builder: EnumeratedValuesBuilder,
        lvl: ValidateLevel,
    ) -> Result<(), SvdError> {
        if builder.derived_from.is_some() {
            self.name = None;
            self.usage = None;
            self.values = Vec::new();
        } else {
            if builder.name.is_some() {
                self.name = builder.name.empty_to_none();
            }
            if builder.usage.is_some() {
                self.usage = builder.usage;
            }
            if let Some(values) = builder.values {
                self.values = values;
            }
        }
        self.validate(lvl)
    }
    /// Validate the [`EnumeratedValues`].
    pub fn validate(&self, lvl: ValidateLevel) -> Result<(), SvdError> {
        if !lvl.is_disabled() {
            if lvl.is_strict() {
                if let Some(name) = self.name.as_ref() {
                    super::check_name(name, "name")?;
                }
            }
            if let Some(_dname) = self.derived_from.as_ref() {
                if lvl.is_strict() {
                    super::check_derived_name(_dname, "derivedFrom")?;
                }
                Ok(())
            } else if self.values.is_empty() {
                Err(Error::Empty.into())
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }
    pub(crate) fn check_range(&self, range: core::ops::Range<u64>) -> Result<(), SvdError> {
        for v in self.values.iter() {
            v.check_range(&range)?;
        }
        Ok(())
    }
    /// Get the usage of these enumerated values.
    pub fn usage(&self) -> Option<Usage> {
        if self.derived_from.is_some() {
            None
        } else {
            Some(self.usage.unwrap_or_default())
        }
    }

    /// Get `enumeratedValue` by name
    pub fn get_value(&self, name: &str) -> Option<&EnumeratedValue> {
        self.values.iter().find(|e| e.name == name)
    }

    /// Get mutable `enumeratedValue` by name
    pub fn get_mut_value(&mut self, name: &str) -> Option<&mut EnumeratedValue> {
        self.values.iter_mut().find(|e| e.name == name)
    }
}
