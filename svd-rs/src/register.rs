use super::{
    array::{descriptions, names},
    Access, BuildError, DataType, Description, DimElement, EmptyToNone, Field, MaybeArray,
    ModifiedWriteValues, Name, ReadAction, RegisterProperties, SvdError, ValidateLevel,
    WriteConstraint,
};
use std::ops::Deref;

/// A single register or array of registers. A register is a named, programmable resource that belongs to a [peripheral](crate::Peripheral).
pub type Register = MaybeArray<RegisterInfo>;

/// Errors from [`RegisterInfo::validate`]
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// Register had no fields, but specified a `<fields>` tag.
    #[error("Register have `fields` tag, but it is empty")]
    EmptyFields,
}

/// A register is a named, programmable resource that belongs to a [peripheral](crate::Peripheral).
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "camelCase")
)]
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct RegisterInfo {
    /// String to identify the register.
    /// Register names are required to be unique within the scope of a peripheral
    pub name: String,

    /// Specifies a register name without the restrictions of an ANSI C identifier.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub display_name: Option<String>,

    /// String describing the details of the register
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub description: Option<String>,

    /// Specifies a group name associated with all alternate register that have the same name
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub alternate_group: Option<String>,

    /// This tag can reference a register that has been defined above to
    /// current location in the description and that describes the memory location already
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub alternate_register: Option<String>,

    /// Define the address offset relative to the enclosing element
    #[cfg_attr(feature = "serde", serde(serialize_with = "crate::as_hex"))]
    pub address_offset: u32,

    /// Specifies register size, access permission and reset value
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub properties: RegisterProperties,

    /// Register data type
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub datatype: Option<DataType>,

    /// Specifies the write side effects
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
    /// If not set, the register is not modified
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub read_action: Option<ReadAction>,

    /// `None` indicates that the `<fields>` node is not present
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub fields: Option<Vec<Field>>,

    /// Specify the register name from which to inherit data.
    /// Elements specified subsequently override inherited values
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub derived_from: Option<String>,
}

/// Return iterator over address offsets of each register in array
pub fn address_offsets<'a>(
    info: &'a RegisterInfo,
    dim: &'a DimElement,
) -> impl Iterator<Item = u32> + 'a {
    (0..dim.dim).map(move |i| info.address_offset + i * dim.dim_increment)
}

/// Extract `RegisterInfo` items from array
pub fn expand<'a>(
    info: &'a RegisterInfo,
    dim: &'a DimElement,
) -> impl Iterator<Item = RegisterInfo> + 'a {
    dim.indexes()
        .zip(names(info, dim))
        .zip(descriptions(info, dim))
        .zip(address_offsets(info, dim))
        .map(|(((idx, name), description), address_offset)| {
            let mut info = info.clone();
            info.name = name;
            info.description = description;
            info.address_offset = address_offset;
            info.display_name = info
                .display_name
                .map(|d| d.replace("[%s]", &idx).replace("%s", &idx));
            info
        })
}

/// Builder for [`RegisterInfo`]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RegisterInfoBuilder {
    name: Option<String>,
    display_name: Option<String>,
    description: Option<String>,
    alternate_group: Option<String>,
    alternate_register: Option<String>,
    address_offset: Option<u32>,
    properties: RegisterProperties,
    datatype: Option<DataType>,
    modified_write_values: Option<ModifiedWriteValues>,
    write_constraint: Option<WriteConstraint>,
    read_action: Option<ReadAction>,
    fields: Option<Vec<Field>>,
    derived_from: Option<String>,
}

impl From<RegisterInfo> for RegisterInfoBuilder {
    fn from(r: RegisterInfo) -> Self {
        Self {
            name: Some(r.name),
            display_name: r.display_name,
            description: r.description,
            alternate_group: r.alternate_group,
            alternate_register: r.alternate_register,
            address_offset: Some(r.address_offset),
            properties: r.properties,
            datatype: r.datatype,
            modified_write_values: r.modified_write_values,
            write_constraint: r.write_constraint,
            read_action: r.read_action,
            fields: r.fields,
            derived_from: r.derived_from,
        }
    }
}

impl RegisterInfoBuilder {
    /// Set the name of the register.
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    /// Set the display name of the register.
    pub fn display_name(mut self, value: Option<String>) -> Self {
        self.display_name = value;
        self
    }
    /// Set the description of the register.
    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }
    /// Set the alternate group of the register.
    pub fn alternate_group(mut self, value: Option<String>) -> Self {
        self.alternate_group = value;
        self
    }
    /// Set the alternate register of the register.
    pub fn alternate_register(mut self, value: Option<String>) -> Self {
        self.alternate_register = value;
        self
    }
    /// Set the address offset of the register.
    pub fn address_offset(mut self, value: u32) -> Self {
        self.address_offset = Some(value);
        self
    }
    /// Set the properties of the register.
    pub fn properties(mut self, value: RegisterProperties) -> Self {
        self.properties = value;
        self
    }
    /// Set the datatype of the register.
    pub fn datatype(mut self, value: Option<DataType>) -> Self {
        self.datatype = value;
        self
    }
    /// Set the size of the register.
    pub fn size(mut self, value: Option<u32>) -> Self {
        self.properties.size = value;
        self
    }
    /// Set the access of the register.
    pub fn access(mut self, value: Option<Access>) -> Self {
        self.properties.access = value;
        self
    }
    /// Set the reset value of the register.
    pub fn reset_value(mut self, value: Option<u64>) -> Self {
        self.properties.reset_value = value;
        self
    }
    /// Set the reset mask of the register.
    pub fn reset_mask(mut self, value: Option<u64>) -> Self {
        self.properties.reset_mask = value;
        self
    }
    /// Set the modified write values of the register.
    pub fn modified_write_values(mut self, value: Option<ModifiedWriteValues>) -> Self {
        self.modified_write_values = value;
        self
    }
    /// Set the write constraint of the register.
    pub fn write_constraint(mut self, value: Option<WriteConstraint>) -> Self {
        self.write_constraint = value;
        self
    }
    /// Set the read action of the register.
    pub fn read_action(mut self, value: Option<ReadAction>) -> Self {
        self.read_action = value;
        self
    }
    /// Set the fields of the register.
    pub fn fields(mut self, value: Option<Vec<Field>>) -> Self {
        self.fields = value;
        self
    }
    /// Set the derived_from attribute of the register.
    pub fn derived_from(mut self, value: Option<String>) -> Self {
        self.derived_from = value;
        self
    }
    /// Validate and build a [`RegisterInfo`].
    pub fn build(self, lvl: ValidateLevel) -> Result<RegisterInfo, SvdError> {
        let reg = RegisterInfo {
            name: self
                .name
                .ok_or_else(|| BuildError::Uninitialized("name".to_string()))?,
            display_name: self.display_name,
            description: self.description,
            alternate_group: self.alternate_group,
            alternate_register: self.alternate_register,
            address_offset: self
                .address_offset
                .ok_or_else(|| BuildError::Uninitialized("address_offset".to_string()))?,
            properties: self.properties.build(lvl)?,
            datatype: self.datatype,
            modified_write_values: self.modified_write_values,
            write_constraint: self.write_constraint,
            read_action: self.read_action,
            fields: self.fields,
            derived_from: self.derived_from,
        };
        reg.validate(lvl)?;
        Ok(reg)
    }
}

impl RegisterInfo {
    /// Make a builder for [`RegisterInfo`]
    pub fn builder() -> RegisterInfoBuilder {
        RegisterInfoBuilder::default()
    }
    /// Construct single [`Register`]
    pub const fn single(self) -> Register {
        Register::Single(self)
    }
    /// Construct [`Register`] array
    pub const fn array(self, dim: DimElement) -> Register {
        Register::Array(self, dim)
    }
    /// Construct single [`Register`] or array
    pub fn maybe_array(self, dim: Option<DimElement>) -> Register {
        if let Some(dim) = dim {
            self.array(dim)
        } else {
            self.single()
        }
    }
    /// Modify an existing [`RegisterInfo`] based on a [builder](RegisterInfoBuilder).
    pub fn modify_from(
        &mut self,
        builder: RegisterInfoBuilder,
        lvl: ValidateLevel,
    ) -> Result<(), SvdError> {
        if let Some(name) = builder.name {
            self.name = name;
        }
        if builder.display_name.is_some() {
            self.display_name = builder.display_name.empty_to_none();
        }
        if builder.description.is_some() {
            self.description = builder.description.empty_to_none();
        }
        if builder.alternate_group.is_some() {
            self.alternate_group = builder.alternate_group.empty_to_none();
        }
        if builder.alternate_register.is_some() {
            self.alternate_register = builder.alternate_register.empty_to_none();
        }
        if let Some(address_offset) = builder.address_offset {
            self.address_offset = address_offset;
        }
        if builder.derived_from.is_some() {
            self.derived_from = builder.derived_from;
            self.fields = None;
            self.properties = RegisterProperties::default();
            self.datatype = None;
            self.modified_write_values = None;
            self.write_constraint = None;
        } else {
            self.properties.modify_from(builder.properties, lvl)?;
            if builder.datatype.is_some() {
                self.datatype = builder.datatype;
            }
            if builder.modified_write_values.is_some() {
                self.modified_write_values = builder.modified_write_values;
            }
            if builder.write_constraint.is_some() {
                self.write_constraint = builder.write_constraint;
            }
            if builder.read_action.is_some() {
                self.read_action = builder.read_action;
            }
            if builder.fields.is_some() {
                self.fields = builder.fields.empty_to_none();
            }
        }
        self.validate(lvl)
    }
    /// Validate the [`RegisterInfo`]
    pub fn validate(&self, lvl: ValidateLevel) -> Result<(), SvdError> {
        if !lvl.is_disabled() {
            if lvl.is_strict() {
                super::check_dimable_name(&self.name, "name")?;
                if let Some(name) = self.alternate_group.as_ref() {
                    super::check_name(name, "alternateGroup")?;
                }
                if let Some(name) = self.alternate_register.as_ref() {
                    super::check_dimable_name(name, "alternateRegister")?;
                }
            }
            if let Some(name) = self.derived_from.as_ref() {
                if lvl.is_strict() {
                    super::check_derived_name(name, "derivedFrom")?;
                }
            } else if let Some(fields) = self.fields.as_ref() {
                if fields.is_empty() && lvl.is_strict() {
                    return Err(Error::EmptyFields.into());
                }
            }
        }
        Ok(())
    }
    /// Validate the [`RegisterInfo`] recursively
    pub fn validate_all(&self, lvl: ValidateLevel) -> Result<(), SvdError> {
        self.properties.validate(lvl)?;
        for f in self.fields() {
            f.validate_all(lvl)?;
        }
        self.validate(lvl)
    }

    /// Returns iterator over child fields
    pub fn fields(&self) -> std::slice::Iter<Field> {
        match &self.fields {
            Some(fields) => fields.iter(),
            None => [].iter(),
        }
    }

    /// Returns mutable iterator over child fields
    pub fn fields_mut(&mut self) -> std::slice::IterMut<Field> {
        match &mut self.fields {
            Some(fields) => fields.iter_mut(),
            None => [].iter_mut(),
        }
    }

    /// Get field by name
    pub fn get_field(&self, name: &str) -> Option<&Field> {
        self.fields().find(|f| f.name == name)
    }

    /// Get mutable field by name
    pub fn get_mut_field(&mut self, name: &str) -> Option<&mut Field> {
        self.fields_mut().find(|f| f.name == name)
    }
}

impl Register {
    /// Validate the [`Register`] recursively
    pub fn validate_all(&self, lvl: ValidateLevel) -> Result<(), SvdError> {
        if let Self::Array(_, dim) = self {
            dim.validate(lvl)?;
        }
        self.deref().validate_all(lvl)
    }
}

impl Name for RegisterInfo {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Description for RegisterInfo {
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}
