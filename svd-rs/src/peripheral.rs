use super::{
    array::{descriptions, names},
    registercluster::{
        AllRegistersIter, AllRegistersIterMut, ClusterIter, ClusterIterMut, RegisterIter,
        RegisterIterMut,
    },
    AddressBlock, BuildError, Cluster, Description, DimElement, EmptyToNone, Interrupt, MaybeArray,
    Name, Register, RegisterCluster, RegisterProperties, SvdError, ValidateLevel,
};
use std::ops::Deref;

/// A single peripheral or array of peripherals
pub type Peripheral = MaybeArray<PeripheralInfo>;

/// Errors from [Peripheral::validate]
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// The peripheral has no registers, but specified a `<registers>` tag.
    #[error("Peripheral have `registers` tag, but it is empty")]
    EmptyRegisters,
}

/// A description of a peripheral in the [device](crate::Device), describing, for example, the [memory mappings](crate::RegisterInfo).
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "camelCase")
)]
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct PeripheralInfo {
    /// The string identifies the peripheral. Peripheral names are required to be unique for a device
    pub name: String,

    /// Specifies a register name without the restrictions of an ANSI C identifier.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub display_name: Option<String>,

    /// The string specifies the version of this peripheral description
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub version: Option<String>,

    /// The string provides an overview of the purpose and functionality of the peripheral
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub description: Option<String>,

    /// Specifies peripheral assigned to the same address blocks
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub alternate_peripheral: Option<String>,

    /// Assigns this peripheral to a group of peripherals. This is only used by the System View
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub group_name: Option<String>,

    /// Define a string as prefix. All register names of this peripheral get this prefix
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub prepend_to_name: Option<String>,

    /// Define a string as suffix. All register names of this peripheral get this suffix
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub append_to_name: Option<String>,

    /// Specify the struct type name created in the device header file
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub header_struct_name: Option<String>,

    /// Lowest address reserved or used by the peripheral
    pub base_address: u64,

    /// Default properties for all registers
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub default_register_properties: RegisterProperties,

    /// Specify an address range uniquely mapped to this peripheral
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub address_block: Option<Vec<AddressBlock>>,

    /// A peripheral can have multiple associated interrupts
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub interrupt: Vec<Interrupt>,

    /// Group to enclose register definitions.
    /// `None` indicates that the `<registers>` node is not present
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub registers: Option<Vec<RegisterCluster>>,

    /// Specify the peripheral name from which to inherit data. Elements specified subsequently override inherited values
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub derived_from: Option<String>,
}

/// Return iterator over base addresses of each peripheral in array
pub fn base_addresses<'a>(
    info: &'a PeripheralInfo,
    dim: &'a DimElement,
) -> impl Iterator<Item = u64> + 'a {
    (0..dim.dim as u64).map(|i| info.base_address + i * dim.dim_increment as u64)
}

/// Extract `PeripheralInfo` items from array
pub fn expand<'a>(
    info: &'a PeripheralInfo,
    dim: &'a DimElement,
) -> impl Iterator<Item = PeripheralInfo> + 'a {
    dim.indexes()
        .zip(names(info, dim))
        .zip(descriptions(info, dim))
        .zip(base_addresses(info, dim))
        .map(|(((idx, name), description), base_address)| {
            let mut info = info.clone();
            info.name = name;
            info.description = description;
            info.base_address = base_address;
            info.display_name = info
                .display_name
                .map(|d| d.replace("[%s]", &idx).replace("%s", &idx));
            info
        })
}

/// Builder for [`Peripheral`]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PeripheralInfoBuilder {
    name: Option<String>,
    display_name: Option<String>,
    version: Option<String>,
    description: Option<String>,
    alternate_peripheral: Option<String>,
    group_name: Option<String>,
    prepend_to_name: Option<String>,
    append_to_name: Option<String>,
    header_struct_name: Option<String>,
    base_address: Option<u64>,
    default_register_properties: RegisterProperties,
    address_block: Option<Vec<AddressBlock>>,
    interrupt: Option<Vec<Interrupt>>,
    registers: Option<Vec<RegisterCluster>>,
    derived_from: Option<String>,
}

impl From<PeripheralInfo> for PeripheralInfoBuilder {
    fn from(p: PeripheralInfo) -> Self {
        Self {
            name: Some(p.name),
            display_name: p.display_name,
            version: p.version,
            description: p.description,
            alternate_peripheral: p.alternate_peripheral,
            group_name: p.group_name,
            prepend_to_name: p.prepend_to_name,
            append_to_name: p.append_to_name,
            header_struct_name: p.header_struct_name,
            base_address: Some(p.base_address),
            default_register_properties: p.default_register_properties,
            address_block: p.address_block,
            interrupt: Some(p.interrupt),
            registers: p.registers,
            derived_from: p.derived_from,
        }
    }
}

impl PeripheralInfoBuilder {
    /// Set the name of the peripheral
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    /// Set the display name of the peripheral
    pub fn display_name(mut self, value: Option<String>) -> Self {
        self.display_name = value;
        self
    }
    /// Set the version of the peripheral
    pub fn version(mut self, value: Option<String>) -> Self {
        self.version = value;
        self
    }
    /// Set the description of the peripheral
    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }
    /// Set the alternate peripheral
    pub fn alternate_peripheral(mut self, value: Option<String>) -> Self {
        self.alternate_peripheral = value;
        self
    }
    /// Set the group name of the peripheral
    pub fn group_name(mut self, value: Option<String>) -> Self {
        self.group_name = value;
        self
    }
    /// Set the prefix for names of all registers of the peripheral
    pub fn prepend_to_name(mut self, value: Option<String>) -> Self {
        self.prepend_to_name = value;
        self
    }
    /// Set the suffix for names of all registers of the peripheral
    pub fn append_to_name(mut self, value: Option<String>) -> Self {
        self.append_to_name = value;
        self
    }
    /// Set the header struct name of the peripheral
    pub fn header_struct_name(mut self, value: Option<String>) -> Self {
        self.header_struct_name = value;
        self
    }
    /// Set the base address of the peripheral
    pub fn base_address(mut self, value: u64) -> Self {
        self.base_address = Some(value);
        self
    }
    /// Set the default register properties of the peripheral
    pub fn default_register_properties(mut self, value: RegisterProperties) -> Self {
        self.default_register_properties = value;
        self
    }
    /// Set the address block of the peripheral
    pub fn address_block(mut self, value: Option<Vec<AddressBlock>>) -> Self {
        self.address_block = value;
        self
    }
    /// Set the interrupts of the peripheral
    pub fn interrupt(mut self, value: Option<Vec<Interrupt>>) -> Self {
        self.interrupt = value;
        self
    }
    /// Set the registers of the peripheral
    pub fn registers(mut self, value: Option<Vec<RegisterCluster>>) -> Self {
        self.registers = value;
        self
    }
    /// Set the derived_from attribute of the peripheral
    pub fn derived_from(mut self, value: Option<String>) -> Self {
        self.derived_from = value;
        self
    }
    /// Validate and build a [`PeripheralInfo`].
    pub fn build(self, lvl: ValidateLevel) -> Result<PeripheralInfo, SvdError> {
        let per = PeripheralInfo {
            name: self
                .name
                .ok_or_else(|| BuildError::Uninitialized("name".to_string()))?,
            display_name: self.display_name.empty_to_none(),
            version: self.version.empty_to_none(),
            description: self.description.empty_to_none(),
            alternate_peripheral: self.alternate_peripheral.empty_to_none(),
            group_name: self.group_name.empty_to_none(),
            prepend_to_name: self.prepend_to_name.empty_to_none(),
            append_to_name: self.append_to_name.empty_to_none(),
            header_struct_name: self.header_struct_name.empty_to_none(),
            base_address: self
                .base_address
                .ok_or_else(|| BuildError::Uninitialized("base_address".to_string()))?,
            default_register_properties: self.default_register_properties.build(lvl)?,
            address_block: self.address_block,
            interrupt: self.interrupt.unwrap_or_default(),
            registers: self.registers,
            derived_from: self.derived_from,
        };
        per.validate(lvl)?;
        Ok(per)
    }
}

impl PeripheralInfo {
    /// Make a builder for [`PeripheralInfo`]
    pub fn builder() -> PeripheralInfoBuilder {
        PeripheralInfoBuilder::default()
    }
    /// Construct single [`Peripheral`]
    pub const fn single(self) -> Peripheral {
        Peripheral::Single(self)
    }
    /// Construct [`Peripheral`] array
    pub const fn array(self, dim: DimElement) -> Peripheral {
        Peripheral::Array(self, dim)
    }
    /// Construct single [`Peripheral`] or array
    pub fn maybe_array(self, dim: Option<DimElement>) -> Peripheral {
        if let Some(dim) = dim {
            self.array(dim)
        } else {
            self.single()
        }
    }
    /// Modify an existing [`Peripheral`] based on a [builder](PeripheralInfoBuilder).
    pub fn modify_from(
        &mut self,
        builder: PeripheralInfoBuilder,
        lvl: ValidateLevel,
    ) -> Result<(), SvdError> {
        if let Some(name) = builder.name {
            self.name = name;
        }
        if builder.display_name.is_some() {
            self.display_name = builder.display_name.empty_to_none();
        }
        if builder.version.is_some() {
            self.version = builder.version.empty_to_none();
        }
        if builder.description.is_some() {
            self.description = builder.description.empty_to_none();
        }
        if builder.alternate_peripheral.is_some() {
            self.alternate_peripheral = builder.alternate_peripheral.empty_to_none();
        }
        if builder.group_name.is_some() {
            self.group_name = builder.group_name.empty_to_none();
        }
        if builder.prepend_to_name.is_some() {
            self.prepend_to_name = builder.prepend_to_name.empty_to_none();
        }
        if builder.append_to_name.is_some() {
            self.append_to_name = builder.append_to_name.empty_to_none();
        }
        if builder.header_struct_name.is_some() {
            self.header_struct_name = builder.header_struct_name.empty_to_none();
        }
        if let Some(base_address) = builder.base_address {
            self.base_address = base_address;
        }
        if let Some(interrupt) = builder.interrupt {
            self.interrupt = interrupt;
        }
        if builder.derived_from.is_some() {
            self.derived_from = builder.derived_from;
            self.registers = None;
            self.address_block = None;
            self.default_register_properties = RegisterProperties::default();
        } else {
            if builder.address_block.is_some() {
                self.address_block = builder.address_block;
            }
            self.default_register_properties
                .modify_from(builder.default_register_properties, lvl)?;
            if builder.registers.is_some() {
                self.registers = builder.registers.empty_to_none();
            }
        }
        self.validate(lvl)
    }

    /// Validate the [`PeripheralInfo`]
    pub fn validate(&self, lvl: ValidateLevel) -> Result<(), SvdError> {
        if !lvl.is_disabled() {
            // TODO
            if lvl.is_strict() {
                super::check_dimable_name(&self.name, "name")?;
            }
            if let Some(name) = self.derived_from.as_ref() {
                if lvl.is_strict() {
                    super::check_dimable_name(name, "derivedFrom")?;
                }
            } else if let Some(registers) = self.registers.as_ref() {
                if registers.is_empty() && lvl.is_strict() {
                    return Err(Error::EmptyRegisters.into());
                }
            }
        }
        Ok(())
    }
    /// Validate the [`PeripheralInfo`] recursively
    pub fn validate_all(&self, lvl: ValidateLevel) -> Result<(), SvdError> {
        if let Some(abs) = self.address_block.as_ref() {
            for ab in abs {
                ab.validate(lvl)?;
            }
        }
        for i in &self.interrupt {
            i.validate(lvl)?;
        }
        self.default_register_properties.validate(lvl)?;
        for r in self.registers() {
            r.validate_all(lvl)?;
        }
        for c in self.clusters() {
            c.validate_all(lvl)?;
        }
        self.validate(lvl)
    }

    /// Returns iterator over child registers
    pub fn registers(&self) -> RegisterIter<'_> {
        RegisterIter {
            all: match &self.registers {
                Some(regs) => regs.iter(),
                None => [].iter(),
            },
        }
    }

    /// Returns mutable iterator over child registers
    pub fn registers_mut(&mut self) -> RegisterIterMut<'_> {
        RegisterIterMut {
            all: match &mut self.registers {
                Some(regs) => regs.iter_mut(),
                None => [].iter_mut(),
            },
        }
    }

    /// Returns iterator over child clusters
    pub fn clusters(&self) -> ClusterIter<'_> {
        ClusterIter {
            all: match &self.registers {
                Some(regs) => regs.iter(),
                None => [].iter(),
            },
        }
    }

    /// Returns mutable iterator over child clusters
    pub fn clusters_mut(&mut self) -> ClusterIterMut<'_> {
        ClusterIterMut {
            all: match &mut self.registers {
                Some(regs) => regs.iter_mut(),
                None => [].iter_mut(),
            },
        }
    }

    /// Returns iterator over all descendant registers
    #[deprecated(since = "0.12.1", note = "Please use `all_registers` instead")]
    pub fn reg_iter(&self) -> AllRegistersIter<'_> {
        self.all_registers()
    }

    /// Returns iterator over all descendant registers
    pub fn all_registers(&self) -> AllRegistersIter<'_> {
        AllRegistersIter {
            rem: match &self.registers {
                Some(regs) => regs.iter().rev().collect(),
                None => Vec::new(),
            },
        }
    }

    /// Returns mutable iterator over all descendant registers
    #[deprecated(since = "0.12.1", note = "Please use `all_registers_mut` instead")]
    pub fn reg_iter_mut(&mut self) -> AllRegistersIterMut<'_> {
        self.all_registers_mut()
    }

    /// Returns mutable iterator over all descendant registers
    pub fn all_registers_mut(&mut self) -> AllRegistersIterMut<'_> {
        AllRegistersIterMut {
            rem: match &mut self.registers {
                Some(regs) => regs.iter_mut().rev().collect(),
                None => Vec::new(),
            },
        }
    }

    /// Get register by name
    pub fn get_register(&self, name: &str) -> Option<&Register> {
        self.registers().find(|f| f.name == name)
    }

    /// Get mutable register by name
    pub fn get_mut_register(&mut self, name: &str) -> Option<&mut Register> {
        self.registers_mut().find(|f| f.name == name)
    }

    /// Get cluster by name
    pub fn get_cluster(&self, name: &str) -> Option<&Cluster> {
        self.clusters().find(|f| f.name == name)
    }

    /// Get mutable cluster by name
    pub fn get_mut_cluster(&mut self, name: &str) -> Option<&mut Cluster> {
        self.clusters_mut().find(|f| f.name == name)
    }

    /// Get interrupt by name
    pub fn get_interrupt(&self, name: &str) -> Option<&Interrupt> {
        self.interrupt.iter().find(|e| e.name == name)
    }

    /// Get mutable enumeratedValue by name
    pub fn get_mut_interrupt(&mut self, name: &str) -> Option<&mut Interrupt> {
        self.interrupt.iter_mut().find(|e| e.name == name)
    }
}

impl Peripheral {
    /// Validate the [`Peripheral`] recursively
    pub fn validate_all(&self, lvl: ValidateLevel) -> Result<(), SvdError> {
        if let Self::Array(_, dim) = self {
            dim.validate(lvl)?;
        }
        self.deref().validate_all(lvl)
    }
}

impl Name for PeripheralInfo {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Description for PeripheralInfo {
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}
