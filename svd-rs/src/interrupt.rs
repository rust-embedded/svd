use super::{BuildError, SvdError, ValidateLevel};

/// Describes an interrupt in the device
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Interrupt {
    /// The string represents the interrupt name
    pub name: String,

    /// The string describes the interrupt
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub description: Option<String>,

    /// Represents the enumeration index value associated to the interrupt
    pub value: u32,
}

/// Builder for [`Interrupt`]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct InterruptBuilder {
    name: Option<String>,
    description: Option<String>,
    value: Option<u32>,
}

impl From<Interrupt> for InterruptBuilder {
    fn from(d: Interrupt) -> Self {
        Self {
            name: Some(d.name),
            description: d.description,
            value: Some(d.value),
        }
    }
}

impl InterruptBuilder {
    /// Set the name of the interrupt
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    /// Set the description of the interrupt
    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }
    /// Set the value of the interrupt
    pub fn value(mut self, value: u32) -> Self {
        self.value = Some(value);
        self
    }
    /// Validate and build a [`Interrupt`].
    pub fn build(self, lvl: ValidateLevel) -> Result<Interrupt, SvdError> {
        let mut de = Interrupt {
            name: self
                .name
                .ok_or_else(|| BuildError::Uninitialized("name".to_string()))?,
            description: self.description,
            value: self
                .value
                .ok_or_else(|| BuildError::Uninitialized("value".to_string()))?,
        };
        if !lvl.is_disabled() {
            de.validate(lvl)?;
        }
        Ok(de)
    }
}

impl Interrupt {
    /// Make a builder for [`Interrupt`]
    pub fn builder() -> InterruptBuilder {
        InterruptBuilder::default()
    }
    /// Modify an existing [`Interrupt`] based on a [builder](InterruptBuilder).
    pub fn modify_from(
        &mut self,
        builder: InterruptBuilder,
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
        if !lvl.is_disabled() {
            self.validate(lvl)
        } else {
            Ok(())
        }
    }
    /// Validate the [`Interrupt`].
    ///
    /// # Notes
    ///
    /// This doesn't do anything.
    pub fn validate(&mut self, _lvl: ValidateLevel) -> Result<(), SvdError> {
        Ok(())
    }
}
