use super::{
    register::{RegIter, RegIterMut},
    AddressBlock, BuildError, EmptyToNone, Interrupt, RegisterCluster, RegisterProperties,
    SvdError, ValidateLevel,
};

/// Errors from [Peripheral::validate]
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// The peripheral has no registers, but specified a `<registers>` tag.
    #[error("Peripheral have `registers` tag, but it is empty")]
    EmptyRegisters,
}

/// A description of a peripheral in the [device](crate::Device), describing, for example, the [memory mappings](crate::RegisterInfo).
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
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

    // alternatePeripheral
    /// Assigns this peripheral to a group of peripherals. This is only used bye the System View
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub group_name: Option<String>,

    // headerStructName
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

/// Builder for [`Peripheral`]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PeripheralInfoBuilder {
    name: Option<String>,
    display_name: Option<String>,
    version: Option<String>,
    description: Option<String>,
    group_name: Option<String>,
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
            group_name: p.group_name,
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
    /// Set the group name of the peripheral
    pub fn group_name(mut self, value: Option<String>) -> Self {
        self.group_name = value;
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
        let mut per = PeripheralInfo {
            name: self
                .name
                .ok_or_else(|| BuildError::Uninitialized("name".to_string()))?,
            display_name: self.display_name.empty_to_none(),
            version: self.version.empty_to_none(),
            description: self.description.empty_to_none(),
            group_name: self.group_name.empty_to_none(),
            base_address: self
                .base_address
                .ok_or_else(|| BuildError::Uninitialized("base_address".to_string()))?,
            default_register_properties: self.default_register_properties.build(lvl)?,
            address_block: self.address_block,
            interrupt: self.interrupt.unwrap_or_default(),
            registers: self.registers,
            derived_from: self.derived_from,
        };
        if !lvl.is_disabled() {
            per.validate(lvl)?;
        }
        Ok(per)
    }
}

impl PeripheralInfo {
    /// Make a builder for [`Peripheral`]
    pub fn builder() -> PeripheralInfoBuilder {
        PeripheralInfoBuilder::default()
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
        if builder.group_name.is_some() {
            self.group_name = builder.group_name.empty_to_none();
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
        if !lvl.is_disabled() {
            self.validate(lvl)
        } else {
            Ok(())
        }
    }

    /// Validate the [`Peripheral`]
    pub fn validate(&mut self, lvl: ValidateLevel) -> Result<(), SvdError> {
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
        Ok(())
    }

    /// returns iterator over all registers peripheral contains
    pub fn reg_iter(&self) -> RegIter {
        if let Some(regs) = &self.registers {
            let mut rem: Vec<&RegisterCluster> = Vec::with_capacity(regs.len());
            for r in regs.iter().rev() {
                rem.push(r);
            }
            RegIter { rem }
        } else {
            RegIter { rem: Vec::new() }
        }
    }

    /// returns mutable iterator over all registers peripheral contains
    pub fn reg_iter_mut(&mut self) -> RegIterMut {
        if let Some(regs) = &mut self.registers {
            let mut rem: Vec<&mut RegisterCluster> = Vec::with_capacity(regs.len());
            for r in regs.iter_mut().rev() {
                rem.push(r);
            }
            RegIterMut { rem }
        } else {
            RegIterMut { rem: Vec::new() }
        }
    }
}
