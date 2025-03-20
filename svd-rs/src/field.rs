use crate::MString;

use super::{
    array::{descriptions, names},
    bitrange, Access, BitRange, BuildError, Description, DimElement, EmptyToNone, EnumeratedValues,
    MaybeArray, ModifiedWriteValues, Name, ReadAction, SvdError, Usage, ValidateLevel,
    WriteConstraint,
};
use std::ops::Deref;

/// Describes a field or fields of a [register](crate::RegisterInfo).
pub type Field = MaybeArray<FieldInfo>;

/// Errors for [`FieldInfo::validate`]
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// The enumerated value is not recognized by svd-rs.
    #[error("You can have 0, 1 or 2 enumeratedValues with different usage")]
    IncompatibleEnumeratedValues,
}

/// A partition of a [register](crate::RegisterInfo)
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct FieldInfo {
    /// Name string used to identify the field.
    /// Field names must be unique within a register
    pub name: String,

    /// String describing the details of the register
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub description: Option<String>,

    /// Bit position of the field within the register
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub bit_range: BitRange,

    /// Predefined strings set the access type.
    /// The element can be omitted if access rights get inherited from parent elements
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub access: Option<Access>,

    /// Describe the manipulation of data written to a field.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub modified_write_values: Option<ModifiedWriteValues>,

    /// Specifies the subset of allowed write values
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub write_constraint: Option<WriteConstraint>,

    /// If set, it specifies the side effect following a read operation.
    /// If not set, the field is not modified
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub read_action: Option<ReadAction>,

    /// Describes the field
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub enumerated_values: Vec<EnumeratedValues>,

    /// Specify the field name from which to inherit data.
    /// Elements specified subsequently override inherited values
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub derived_from: Option<String>,
}

/// Return iterator over bit offsets of each field in array
pub fn bit_offsets<'a>(info: &'a FieldInfo, dim: &'a DimElement) -> impl Iterator<Item = u32> + 'a {
    (0..dim.dim).map(|i| info.bit_offset() + i * dim.dim_increment)
}

/// Extract `FieldInfo` items from array
pub fn expand<'a>(
    info: &'a FieldInfo,
    dim: &'a DimElement,
) -> impl Iterator<Item = FieldInfo> + 'a {
    names(info, dim)
        .zip(descriptions(info, dim))
        .zip(bit_offsets(info, dim))
        .map(|((name, description), bit_offset)| {
            let mut info = info.clone();
            info.name = name;
            info.description = description;
            info.bit_range = BitRange::from_offset_width(bit_offset, info.bit_width());
            info
        })
}

/// Builder for [`FieldInfo`]

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FieldInfoBuilder {
    name: Option<MString>,
    description: Option<MString>,
    bit_range: Option<BitRange>,
    bit_offset: Option<u32>,
    bit_width: Option<u32>,
    access: Option<Access>,
    modified_write_values: Option<ModifiedWriteValues>,
    write_constraint: Option<WriteConstraint>,
    read_action: Option<ReadAction>,
    enumerated_values: Option<Vec<EnumeratedValues>>,
    derived_from: Option<String>,
}

impl From<FieldInfo> for FieldInfoBuilder {
    fn from(f: FieldInfo) -> Self {
        Self {
            name: Some(f.name.into()),
            description: f.description.map(Into::into),
            bit_range: Some(f.bit_range),
            bit_offset: None,
            bit_width: None,
            access: f.access,
            modified_write_values: f.modified_write_values,
            write_constraint: f.write_constraint,
            read_action: f.read_action,
            enumerated_values: Some(f.enumerated_values),
            derived_from: f.derived_from,
        }
    }
}

impl FieldInfoBuilder {
    /// Set the name of the field
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value.into());
        self
    }
    /// Set the description of the field
    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = value.map(Into::into);
        self
    }
    /// Set the bit range of the field
    pub fn bit_range(mut self, value: BitRange) -> Self {
        self.bit_range = Some(value);
        self.bit_offset = None;
        self.bit_width = None;
        self
    }
    /// Set the bit offset of the field
    pub fn bit_offset(mut self, value: u32) -> Self {
        if let Some(bit_range) = self.bit_range.as_mut() {
            bit_range.offset = value;
        } else if let Some(width) = self.bit_offset {
            self.bit_range = Some(BitRange::from_offset_width(value, width));
            self.bit_width = None;
        } else {
            self.bit_offset = Some(value);
        }
        self
    }
    /// Set the bit width of the field
    pub fn bit_width(mut self, value: u32) -> Self {
        if let Some(bit_range) = self.bit_range.as_mut() {
            bit_range.width = value;
        } else if let Some(offset) = self.bit_offset {
            self.bit_range = Some(BitRange::from_offset_width(offset, value));
            self.bit_offset = None;
        } else {
            self.bit_width = Some(value);
        }
        self
    }
    /// Set the access of the field
    pub fn access(mut self, value: Option<Access>) -> Self {
        self.access = value;
        self
    }
    /// Set the modified write values of the field
    pub fn modified_write_values(mut self, value: Option<ModifiedWriteValues>) -> Self {
        self.modified_write_values = value;
        self
    }
    /// Set the write constraint of the field
    pub fn write_constraint(mut self, value: Option<WriteConstraint>) -> Self {
        self.write_constraint = value;
        self
    }
    /// Set the read action of the register.
    pub fn read_action(mut self, value: Option<ReadAction>) -> Self {
        self.read_action = value;
        self
    }
    /// Set the enumerated values of the field
    pub fn enumerated_values(mut self, value: Vec<EnumeratedValues>) -> Self {
        self.enumerated_values = Some(value);
        self
    }
    /// Set the derived_from attribute of the field
    pub fn derived_from(mut self, value: Option<String>) -> Self {
        self.derived_from = value;
        self
    }
    /// Validate and build a [`FieldInfo`].
    pub fn build(self, lvl: ValidateLevel) -> Result<FieldInfo, SvdError> {
        let field = FieldInfo {
            name: self
                .name
                .ok_or_else(|| BuildError::Uninitialized("name".to_string()))?
                .as_str()?,
            description: self.description.empty_to_none(),
            bit_range: self
                .bit_range
                .ok_or_else(|| BuildError::Uninitialized("bit_range".to_string()))?,
            access: self.access,
            modified_write_values: self.modified_write_values,
            write_constraint: self.write_constraint,
            read_action: self.read_action,
            enumerated_values: self.enumerated_values.unwrap_or_default(),
            derived_from: self.derived_from,
        };
        field.validate(lvl)?;
        Ok(field)
    }
}

impl FieldInfo {
    /// Make a builder for [`FieldInfo`]
    pub fn builder() -> FieldInfoBuilder {
        FieldInfoBuilder::default()
    }
    /// Construct single [`Field`]
    pub const fn single(self) -> Field {
        Field::Single(self)
    }
    /// Construct [`Field`] array
    pub const fn array(self, dim: DimElement) -> Field {
        Field::Array(self, dim)
    }
    /// Construct single [`Field`] or array
    pub fn maybe_array(self, dim: Option<DimElement>) -> Field {
        if let Some(dim) = dim {
            self.array(dim)
        } else {
            self.single()
        }
    }
    /// Modify an existing [`FieldInfo`] based on a [builder](FieldInfoBuilder).
    pub fn modify_from(
        &mut self,
        builder: FieldInfoBuilder,
        lvl: ValidateLevel,
    ) -> Result<(), SvdError> {
        if let Some(name) = builder.name {
            self.name = name;
        }
        if builder.description.is_some() {
            self.description = builder.description.empty_to_none();
        }
        if let Some(bit_range) = builder.bit_range {
            self.bit_range = bit_range;
        }
        if let Some(offset) = builder.bit_offset {
            self.bit_range.offset = offset;
        }
        if let Some(width) = builder.bit_width {
            self.bit_range.width = width;
        }
        if builder.access.is_some() {
            self.access = builder.access;
        }
        if builder.derived_from.is_some() {
            self.derived_from = builder.derived_from;
            self.modified_write_values = None;
            self.write_constraint = None;
            self.enumerated_values = Vec::new();
        } else {
            if builder.modified_write_values.is_some() {
                self.modified_write_values = builder.modified_write_values;
            }
            if builder.write_constraint.is_some() {
                self.write_constraint = builder.write_constraint;
            }
            if builder.read_action.is_some() {
                self.read_action = builder.read_action;
            }
            if let Some(enumerated_values) = builder.enumerated_values {
                self.enumerated_values = enumerated_values;
            }
        }
        self.validate(lvl)
    }
    /// Validate the [`FieldInfo`].
    pub fn validate(&self, lvl: ValidateLevel) -> Result<(), SvdError> {
        if !lvl.is_disabled() {
            if lvl.is_strict() {
                super::check_dimable_name(&self.name, "name")?;
                if let Some(name) = self.derived_from.as_ref() {
                    super::check_derived_name(name, "derivedFrom")?;
                }
            }

            if self.bit_range.width == 0 {
                return Err(bitrange::Error::ZeroWidth.into());
            }

            // If the bit_range has its maximum width, all enumerated values will of
            // course fit in so we can skip validation.
            if self.bit_range.width < 64 {
                for ev in &self.enumerated_values {
                    ev.check_range(0..2_u64.pow(self.bit_range.width))?;
                }
            }

            if lvl.is_strict() {
                match self.enumerated_values.as_slice() {
                    [] | [_] => {}
                    [ev1, ev2]
                        if matches!(ev1.usage(), None | Some(Usage::Read))
                            && matches!(ev2.usage(), None | Some(Usage::Write)) => {}
                    [ev1, ev2]
                        if matches!(ev2.usage(), None | Some(Usage::Read))
                            && matches!(ev1.usage(), None | Some(Usage::Write)) => {}
                    _ => return Err(Error::IncompatibleEnumeratedValues.into()),
                }
            }

            match self.write_constraint {
                // If the bit_range has its maximum width, all values will of
                // course fit in so we can skip validation.
                Some(WriteConstraint::Range(constraint)) if self.bit_range.width < 64 => {
                    constraint.check_range(0..2_u64.pow(self.bit_range.width))?;
                }
                _ => (),
            }
        }

        Ok(())
    }
    /// Validate the [`FieldInfo`] recursively
    pub fn validate_all(&self, lvl: ValidateLevel) -> Result<(), SvdError> {
        for evs in &self.enumerated_values {
            evs.validate_all(lvl)?;
        }
        self.validate(lvl)
    }

    /// Get bit offset
    pub fn bit_offset(&self) -> u32 {
        self.bit_range.offset
    }

    /// Get bit width
    pub fn bit_width(&self) -> u32 {
        self.bit_range.width
    }

    /// Get the position of the least significant bit
    pub fn lsb(&self) -> u32 {
        self.bit_range.lsb()
    }
    /// Get the position of the most significant bit
    pub fn msb(&self) -> u32 {
        self.bit_range.msb()
    }

    /// Get bits which is affected by field
    pub fn bitmask(&self) -> u64 {
        let BitRange { offset, width, .. } = self.bit_range;
        (!0u64 >> (64 - width)) << offset
    }

    /// Get enumeratedValues cluster by usage
    pub fn get_enumerated_values(&self, usage: Usage) -> Option<&EnumeratedValues> {
        match self.enumerated_values.len() {
            1 | 2 => self
                .enumerated_values
                .iter()
                .find(|ev| ev.usage() == Some(usage)),
            _ => None,
        }
    }

    /// Get mutable enumeratedValues by usage
    pub fn get_mut_enumerated_values(&mut self, usage: Usage) -> Option<&mut EnumeratedValues> {
        match self.enumerated_values.len() {
            1 | 2 => self
                .enumerated_values
                .iter_mut()
                .find(|ev| ev.usage() == Some(usage)),
            _ => None,
        }
    }
}

impl Field {
    /// Validate the [`Field`] recursively
    pub fn validate_all(&self, lvl: ValidateLevel) -> Result<(), SvdError> {
        if let Self::Array(_, dim) = self {
            dim.validate(lvl)?;
        }
        self.deref().validate_all(lvl)
    }

    /// Get bits which is affected by field or field array
    pub fn bitmask(&self) -> u64 {
        match self {
            Field::Single(f) => f.bitmask(),
            Field::Array(f, d) => {
                let mask = f.bitmask();
                let mut bits = 0;
                for i in 0..d.dim {
                    bits |= mask << (i * d.dim_increment);
                }
                bits
            }
        }
    }
}

impl Name for FieldInfo {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Description for FieldInfo {
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}
