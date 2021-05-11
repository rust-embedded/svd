use crate::error::*;

use crate::svd::{
    access::Access, bitrange::BitRange, enumeratedvalues::EnumeratedValues,
    modifiedwritevalues::ModifiedWriteValues, writeconstraint::WriteConstraint,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct FieldInfo {
    /// Name string used to identify the field.
    /// Field names must be unique within a register
    pub name: String,

    /// String describing the details of the register
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<String>,

    /// Bit position of the field within the register
    pub bit_range: BitRange,

    /// Predefined strings set the access type.
    /// The element can be omitted if access rights get inherited from parent elements
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub access: Option<Access>,

    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub modified_write_values: Option<ModifiedWriteValues>,

    /// Specifies the subset of allowed write values
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub write_constraint: Option<WriteConstraint>,

    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub enumerated_values: Vec<EnumeratedValues>,

    /// Specify the field name from which to inherit data.
    /// Elements specified subsequently override inherited values
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub derived_from: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FieldInfoBuilder {
    name: Option<String>,
    description: Option<String>,
    bit_range: Option<BitRange>,
    access: Option<Access>,
    modified_write_values: Option<ModifiedWriteValues>,
    write_constraint: Option<WriteConstraint>,
    enumerated_values: Option<Vec<EnumeratedValues>>,
    derived_from: Option<String>,
}

impl From<FieldInfo> for FieldInfoBuilder {
    fn from(f: FieldInfo) -> Self {
        Self {
            name: Some(f.name),
            description: f.description,
            bit_range: Some(f.bit_range),
            access: f.access,
            modified_write_values: f.modified_write_values,
            write_constraint: f.write_constraint,
            enumerated_values: Some(f.enumerated_values),
            derived_from: f.derived_from,
        }
    }
}

impl FieldInfoBuilder {
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }
    pub fn bit_range(mut self, value: BitRange) -> Self {
        self.bit_range = Some(value);
        self
    }
    pub fn access(mut self, value: Option<Access>) -> Self {
        self.access = value;
        self
    }
    pub fn modified_write_values(mut self, value: Option<ModifiedWriteValues>) -> Self {
        self.modified_write_values = value;
        self
    }
    pub fn write_constraint(mut self, value: Option<WriteConstraint>) -> Self {
        self.write_constraint = value;
        self
    }
    pub fn enumerated_values(mut self, value: Vec<EnumeratedValues>) -> Self {
        self.enumerated_values = Some(value);
        self
    }
    pub fn derived_from(mut self, value: Option<String>) -> Self {
        self.derived_from = value;
        self
    }
    pub fn build(self) -> Result<FieldInfo> {
        (FieldInfo {
            name: self
                .name
                .ok_or_else(|| BuildError::Uninitialized("name".to_string()))?,
            description: self.description,
            bit_range: self
                .bit_range
                .ok_or_else(|| BuildError::Uninitialized("bit_range".to_string()))?,
            access: self.access,
            modified_write_values: self.modified_write_values,
            write_constraint: self.write_constraint,
            enumerated_values: self.enumerated_values.unwrap_or_default(),
            derived_from: self.derived_from,
        })
        .validate()
    }
}

impl FieldInfo {
    pub fn builder() -> FieldInfoBuilder {
        FieldInfoBuilder::default()
    }
    fn validate(self) -> Result<Self> {
        #[cfg(feature = "strict")]
        check_dimable_name(&self.name, "name")?;
        #[cfg(feature = "strict")]
        {
            if let Some(name) = self.derived_from.as_ref() {
                check_derived_name(name, "derivedFrom")?;
            }
        }

        if self.bit_range.width == 0 {
            anyhow::bail!("bitRange width of 0 does not make sense");
        }

        // If the bit_range has its maximum width, all enumerated values will of
        // course fit in so we can skip validation.
        if self.bit_range.width < 64 {
            for ev in &self.enumerated_values {
                ev.check_range(0..2_u64.pow(self.bit_range.width))?;
            }
        }
        Ok(self)
    }
}
