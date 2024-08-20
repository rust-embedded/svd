use crate::{BuildError, Description, Name, SvdError, ValidateLevel};

/// Describes a exception source in the device
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Exception {
    /// The string represents the exception source name
    pub name: String,

    /// The string describes the exception source
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub description: Option<String>,

    /// Represents the enumeration index value associated to the exception source
    pub value: u32,
}

/// Builder for [`Exception`]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ExceptionBuilder {
    name: Option<String>,
    description: Option<String>,
    value: Option<u32>,
}

impl From<Exception> for ExceptionBuilder {
    fn from(d: Exception) -> Self {
        Self {
            name: Some(d.name),
            description: d.description,
            value: Some(d.value),
        }
    }
}

impl ExceptionBuilder {
    /// Set the name of the exception source
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    /// Set the description of the exception source
    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }
    /// Set the value of the exception source
    pub fn value(mut self, value: u32) -> Self {
        self.value = Some(value);
        self
    }
    /// Validate and build an [`Exception`].
    pub fn build(self, lvl: ValidateLevel) -> Result<Exception, SvdError> {
        let de = Exception {
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

impl Exception {
    /// Make a builder for [`Exception`]
    pub fn builder() -> ExceptionBuilder {
        ExceptionBuilder::default()
    }
    /// Modify an existing [`Exception`] based on a [builder](ExceptionBuilder).
    pub fn modify_from(
        &mut self,
        builder: ExceptionBuilder,
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
    /// Validate the [`Exception`].
    ///
    /// # Notes
    ///
    /// This doesn't do anything.
    pub fn validate(&self, _lvl: ValidateLevel) -> Result<(), SvdError> {
        Ok(())
    }
}

impl Name for Exception {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Description for Exception {
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}
