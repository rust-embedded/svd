use super::{
    BuildError, Cpu, EmptyToNone, Peripheral, RegisterProperties, SvdError, ValidateLevel,
};

/// Errors for [`Device::validate`]
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// Device has no peripherals
    #[error("Device must contain at least one peripheral")]
    EmptyDevice,
}

/// The top element in a SVD file. Describes information specific to a device.
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "camelCase")
)]
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Device {
    // vendor

    // vendorID
    /// The string identifies the device or device series. Device names are required to be unique
    pub name: String,

    // series
    /// Define the version of the SVD file
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub version: Option<String>,

    /// Describe the main features of the device (for example CPU, clock frequency, peripheral overview)
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub description: Option<String>,

    // licenseText
    /// Describe the processor included in the device
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub cpu: Option<Cpu>,

    /// Define the number of data bits uniquely selected by each address
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub address_unit_bits: Option<u32>,

    /// Define the number of data bit-width of the maximum single data transfer supported by the bus infrastructure
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub width: Option<u32>,

    /// Default properties for all registers
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub default_register_properties: RegisterProperties,

    /// Group to define peripherals
    pub peripherals: Vec<Peripheral>,

    /// Specify the compliant CMSIS-SVD schema version
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub schema_version: Option<String>,
}

/// Builder for [`Device`]
#[derive(Clone, Debug, Default)]
pub struct DeviceBuilder {
    name: Option<String>,
    version: Option<String>,
    description: Option<String>,
    cpu: Option<Cpu>,
    address_unit_bits: Option<u32>,
    width: Option<u32>,
    default_register_properties: RegisterProperties,
    peripherals: Option<Vec<Peripheral>>,
    schema_version: Option<String>,
}

impl From<Device> for DeviceBuilder {
    fn from(d: Device) -> Self {
        Self {
            name: Some(d.name),
            version: d.version,
            description: d.description,
            cpu: d.cpu.map(Into::into),
            address_unit_bits: d.address_unit_bits,
            width: d.width,
            default_register_properties: d.default_register_properties,
            peripherals: Some(d.peripherals),
            schema_version: d.schema_version,
        }
    }
}

impl DeviceBuilder {
    /// Set the name of the device.
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    /// Set the version of the device.
    pub fn version(mut self, value: Option<String>) -> Self {
        self.version = value;
        self
    }
    /// Set the description of the device.
    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }
    /// Set the cpu of the device.
    pub fn cpu(mut self, value: Option<Cpu>) -> Self {
        self.cpu = value;
        self
    }
    /// Set the address unit bits of the device.
    pub fn address_unit_bits(mut self, value: Option<u32>) -> Self {
        self.address_unit_bits = value;
        self
    }
    /// Set the width of the device.
    pub fn width(mut self, value: Option<u32>) -> Self {
        self.width = value;
        self
    }
    /// Set the default register properties of the device.
    pub fn default_register_properties(mut self, value: RegisterProperties) -> Self {
        self.default_register_properties = value;
        self
    }
    /// Set the peripherals of the device.
    pub fn peripherals(mut self, value: Vec<Peripheral>) -> Self {
        self.peripherals = Some(value);
        self
    }
    /// Set the schema version of the device.
    pub fn schema_version(mut self, value: Option<String>) -> Self {
        self.schema_version = value;
        self
    }
    /// Validate and build a [`Device`].
    pub fn build(self, lvl: ValidateLevel) -> Result<Device, SvdError> {
        let mut device = Device {
            name: self
                .name
                .ok_or_else(|| BuildError::Uninitialized("name".to_string()))?,
            version: self.version,
            description: self.description,
            cpu: self.cpu,
            address_unit_bits: self.address_unit_bits,
            width: self.width,
            default_register_properties: self.default_register_properties.build(lvl)?,
            peripherals: self
                .peripherals
                .ok_or_else(|| BuildError::Uninitialized("peripherals".to_string()))?,
            schema_version: self.schema_version,
        };
        if !lvl.is_disabled() {
            device.validate(lvl)?;
        }
        Ok(device)
    }
}

impl Device {
    /// Make a builder for [`Device`]
    pub fn builder() -> DeviceBuilder {
        DeviceBuilder::default()
    }
    /// Modify an existing [`Device`] based on a [builder](DeviceBuilder).
    pub fn modify_from(
        &mut self,
        builder: DeviceBuilder,
        lvl: ValidateLevel,
    ) -> Result<(), SvdError> {
        if let Some(name) = builder.name {
            self.name = name;
        }
        if builder.version.is_some() {
            self.version = builder.version.empty_to_none();
        }
        if builder.description.is_some() {
            self.description = builder.description.empty_to_none();
        }
        if builder.cpu.is_some() {
            self.cpu = builder.cpu;
        }
        if builder.address_unit_bits.is_some() {
            self.address_unit_bits = builder.address_unit_bits;
        }
        if builder.width.is_some() {
            self.width = builder.width;
        }
        self.default_register_properties
            .modify_from(builder.default_register_properties, lvl)?;
        if let Some(peripherals) = builder.peripherals {
            self.peripherals = peripherals;
        }
        if !lvl.is_disabled() {
            self.validate(lvl)
        } else {
            Ok(())
        }
    }
    /// Validate the [`Device`]
    pub fn validate(&mut self, _lvl: ValidateLevel) -> Result<(), SvdError> {
        // TODO
        if self.peripherals.is_empty() {
            return Err(Error::EmptyDevice.into());
        }
        Ok(())
    }
}
