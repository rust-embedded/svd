use super::{
    register::{RegIter, RegIterMut},
    AddressBlock, BuildError, Interrupt, RegisterCluster, RegisterProperties, SvdError,
    ValidateLevel,
};

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("Peripheral have `registers` tag, but it is empty")]
    EmptyRegisters,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Peripheral {
    /// The string identifies the peripheral. Peripheral names are required to be unique for a device
    pub name: String,

    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub display_name: Option<String>,

    /// The string specifies the version of this peripheral description
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub version: Option<String>,

    /// The string provides an overview of the purpose and functionality of the peripheral
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<String>,

    // alternatePeripheral
    /// Assigns this peripheral to a group of peripherals. This is only used bye the System View
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub group_name: Option<String>,

    // headerStructName
    /// Lowest address reserved or used by the peripheral
    pub base_address: u64,

    /// Default properties for all registers
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub default_register_properties: RegisterProperties,

    /// Specify an address range uniquely mapped to this peripheral
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub address_block: Option<Vec<AddressBlock>>,

    /// A peripheral can have multiple associated interrupts
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub interrupt: Vec<Interrupt>,

    /// Group to enclose register definitions.
    /// `None` indicates that the `<registers>` node is not present
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub registers: Option<Vec<RegisterCluster>>,

    /// Specify the peripheral name from which to inherit data. Elements specified subsequently override inherited values
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub derived_from: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PeripheralBuilder {
    name: Option<String>,
    display_name: Option<String>,
    version: Option<String>,
    description: Option<String>,
    group_name: Option<String>,
    base_address: Option<u64>,
    default_register_properties: RegisterProperties,
    address_block: Option<Vec<AddressBlock>>,
    interrupt: Vec<Interrupt>,
    registers: Option<Vec<RegisterCluster>>,
    derived_from: Option<String>,
}

impl From<Peripheral> for PeripheralBuilder {
    fn from(p: Peripheral) -> Self {
        Self {
            name: Some(p.name),
            display_name: p.display_name,
            version: p.version,
            description: p.description,
            group_name: p.group_name,
            base_address: Some(p.base_address),
            default_register_properties: p.default_register_properties.into(),
            address_block: p.address_block,
            interrupt: p.interrupt,
            registers: p.registers,
            derived_from: p.derived_from,
        }
    }
}

impl PeripheralBuilder {
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    pub fn display_name(mut self, value: Option<String>) -> Self {
        self.display_name = value;
        self
    }
    pub fn version(mut self, value: Option<String>) -> Self {
        self.version = value;
        self
    }
    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }
    pub fn group_name(mut self, value: Option<String>) -> Self {
        self.group_name = value;
        self
    }
    pub fn base_address(mut self, value: u64) -> Self {
        self.base_address = Some(value);
        self
    }
    pub fn default_register_properties(mut self, value: RegisterProperties) -> Self {
        self.default_register_properties = value;
        self
    }
    pub fn address_block(mut self, value: Option<Vec<AddressBlock>>) -> Self {
        self.address_block = value;
        self
    }
    pub fn interrupt(mut self, value: Vec<Interrupt>) -> Self {
        self.interrupt = value;
        self
    }
    pub fn registers(mut self, value: Option<Vec<RegisterCluster>>) -> Self {
        self.registers = value;
        self
    }
    pub fn derived_from(mut self, value: Option<String>) -> Self {
        self.derived_from = value;
        self
    }
    pub fn build(self, lvl: ValidateLevel) -> Result<Peripheral, SvdError> {
        (Peripheral {
            name: self
                .name
                .ok_or_else(|| BuildError::Uninitialized("name".to_string()))?,
            display_name: self.display_name,
            version: self.version,
            description: self.description,
            group_name: self.group_name,
            base_address: self
                .base_address
                .ok_or_else(|| BuildError::Uninitialized("base_address".to_string()))?,
            default_register_properties: self.default_register_properties.build(lvl)?,
            address_block: self.address_block,
            interrupt: self.interrupt,
            registers: self.registers,
            derived_from: self.derived_from,
        })
        .validate(lvl)
    }
}

impl Peripheral {
    pub fn builder() -> PeripheralBuilder {
        PeripheralBuilder::default()
    }

    fn validate(self, lvl: ValidateLevel) -> Result<Self, SvdError> {
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
        Ok(self)
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
