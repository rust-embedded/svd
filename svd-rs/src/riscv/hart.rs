use crate::{BuildError, Description, Name, SvdError, ValidateLevel};

/// Describes a HART ID in the device
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Hart {
    /// The string represents the HART ID
    pub name: String,

    /// The string describes the HART ID
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub description: Option<String>,

    /// Represents the enumeration index value associated to the HART ID
    pub value: u32,
}

/// Builder for [`Hart`]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HartBuilder {
    name: Option<String>,
    description: Option<String>,
    value: Option<u32>,
}

impl From<Hart> for HartBuilder {
    fn from(d: Hart) -> Self {
        Self {
            name: Some(d.name),
            description: d.description,
            value: Some(d.value),
        }
    }
}

impl HartBuilder {
    /// Set the name of the HART
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    /// Set the description of the HART
    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }
    /// Set the value of the HART
    pub fn value(mut self, value: u32) -> Self {
        self.value = Some(value);
        self
    }
    /// Validate and build a [`Hart`].
    pub fn build(self, lvl: ValidateLevel) -> Result<Hart, SvdError> {
        let de = Hart {
            name: self
                .name
                .ok_or_else(|| BuildError::Uninitialized("name".to_string()))?,
            description: self.description,
            value: self
                .value
                .ok_or_else(|| BuildError::Uninitialized("value".to_string()))?,
        };
        de.validate(lvl)?;
        Ok(de)
    }
}

impl Hart {
    /// Make a builder for [`Hart`]
    pub fn builder() -> HartBuilder {
        HartBuilder::default()
    }
    /// Modify an existing [`Hart`] based on a [builder](HartBuilder).
    pub fn modify_from(
        &mut self,
        builder: HartBuilder,
        lvl: ValidateLevel,
    ) -> Result<(), SvdError> {
        if let Some(name) = builder.name {
            self.name = name;
        }
        if builder.description.is_some() {
            self.description = builder.description;
        }
        if let Some(value) = builder.value {
            self.value = value;
        }
        self.validate(lvl)
    }
    /// Validate the [`Hart`].
    ///
    /// # Notes
    ///
    /// This doesn't do anything.
    pub fn validate(&self, _lvl: ValidateLevel) -> Result<(), SvdError> {
        Ok(())
    }
}

impl Name for Hart {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Description for Hart {
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}
