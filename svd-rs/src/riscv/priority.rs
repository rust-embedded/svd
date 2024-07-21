use crate::{BuildError, Description, Name, SvdError, ValidateLevel};

/// Describes a priority level in the device
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Priority {
    /// The string represents the priority level
    pub name: String,

    /// The string describes the priority level
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub description: Option<String>,

    /// Represents the enumeration index value associated to the priority level
    pub value: u32,
}

/// Builder for [`Priority`]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PriorityBuilder {
    name: Option<String>,
    description: Option<String>,
    value: Option<u32>,
}

impl From<Priority> for PriorityBuilder {
    fn from(d: Priority) -> Self {
        Self {
            name: Some(d.name),
            description: d.description,
            value: Some(d.value),
        }
    }
}

impl PriorityBuilder {
    /// Set the name of the priority level
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    /// Set the description of the priority level
    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }
    /// Set the value of the priority level
    pub fn value(mut self, value: u32) -> Self {
        self.value = Some(value);
        self
    }
    /// Validate and build a [`Priority`].
    pub fn build(self, lvl: ValidateLevel) -> Result<Priority, SvdError> {
        let de = Priority {
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

impl Priority {
    /// Make a builder for [`Priority`]
    pub fn builder() -> PriorityBuilder {
        PriorityBuilder::default()
    }
    /// Modify an existing [`Priority`] based on a [builder](PriorityBuilder).
    pub fn modify_from(
        &mut self,
        builder: PriorityBuilder,
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
    /// Validate the [`Priority`].
    ///
    /// # Notes
    ///
    /// This doesn't do anything.
    pub fn validate(&self, _lvl: ValidateLevel) -> Result<(), SvdError> {
        Ok(())
    }
}

impl Name for Priority {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Description for Priority {
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}
