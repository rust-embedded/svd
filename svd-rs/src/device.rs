use super::{
    BuildError, Cpu, Description, EmptyToNone, Name, Peripheral, RegisterProperties, SvdError,
    ValidateLevel,
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
    /// Specify the vendor of the device using the full name
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub vendor: Option<String>,

    /// Specify the vendor abbreviation without spaces or special characters
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none", rename = "vendorID")
    )]
    pub vendor_id: Option<String>,

    /// The string identifies the device or device series. Device names are required to be unique
    pub name: String,

    /// Specify the name of the device series
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub series: Option<String>,

    /// Define the version of the SVD file
    pub version: String,

    /// Describe the main features of the device (for example CPU, clock frequency, peripheral overview)
    pub description: String,

    /// The text will be copied into the header section of the generated device header file and shall contain the legal disclaimer
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub license_text: Option<String>,

    /// Describe the processor included in the device
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub cpu: Option<Cpu>,

    /// Specify the file name (without extension) of the device-specific system include file
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub header_system_filename: Option<String>,

    /// This string is prepended to all type definition names generated in the CMSIS-Core device header file
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub header_definitions_prefix: Option<String>,

    /// Define the number of data bits uniquely selected by each address
    pub address_unit_bits: u32,

    /// Define the number of data bit-width of the maximum single data transfer supported by the bus infrastructure
    pub width: u32,

    /// Default properties for all registers
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub default_register_properties: RegisterProperties,

    /// Group to define peripherals
    pub peripherals: Vec<Peripheral>,

    /// Specify the underlying XML schema to which the CMSIS-SVD schema is compliant.
    #[cfg_attr(feature = "serde", serde(skip, default = "default_xmlns_xs"))]
    pub xmlns_xs: String,

    /// Specify the file path and file name of the CMSIS-SVD Schema
    #[cfg_attr(
        feature = "serde",
        serde(skip, default = "default_no_namespace_schema_location")
    )]
    pub no_namespace_schema_location: String,

    /// Specify the compliant CMSIS-SVD schema version
    #[cfg_attr(feature = "serde", serde(skip, default = "default_schema_version"))]
    pub schema_version: String,

    /// XML global comments (usually license)
    #[cfg_attr(feature = "serde", serde(skip, default))]
    pub comments: Vec<String>,
}

fn default_xmlns_xs() -> String {
    "http://www.w3.org/2001/XMLSchema-instance".into()
}
fn default_no_namespace_schema_location() -> String {
    format!(
        "CMSIS-SVD_Schema_{}.xsd",
        default_schema_version().replace('.', "_")
    )
}
fn default_schema_version() -> String {
    "1.1".into()
}

/// Builder for [`Device`]
#[derive(Clone, Debug, Default)]
pub struct DeviceBuilder {
    vendor: Option<String>,
    vendor_id: Option<String>,
    name: Option<String>,
    series: Option<String>,
    version: Option<String>,
    description: Option<String>,
    license_text: Option<String>,
    cpu: Option<Cpu>,
    header_system_filename: Option<String>,
    header_definitions_prefix: Option<String>,
    address_unit_bits: Option<u32>,
    width: Option<u32>,
    default_register_properties: RegisterProperties,
    peripherals: Option<Vec<Peripheral>>,
    xmlns_xs: Option<String>,
    no_namespace_schema_location: Option<String>,
    schema_version: Option<String>,
    comments: Vec<String>,
}

impl From<Device> for DeviceBuilder {
    fn from(d: Device) -> Self {
        Self {
            vendor: d.vendor,
            vendor_id: d.vendor_id,
            name: Some(d.name),
            series: d.series,
            version: Some(d.version),
            description: Some(d.description),
            license_text: d.license_text,
            cpu: d.cpu,
            header_system_filename: d.header_system_filename,
            header_definitions_prefix: d.header_definitions_prefix,
            address_unit_bits: Some(d.address_unit_bits),
            width: Some(d.width),
            default_register_properties: d.default_register_properties,
            peripherals: Some(d.peripherals),
            xmlns_xs: Some(d.xmlns_xs),
            no_namespace_schema_location: Some(d.no_namespace_schema_location),
            schema_version: Some(d.schema_version),
            comments: d.comments,
        }
    }
}

impl DeviceBuilder {
    /// Set the vendor of the device.
    pub fn vendor(mut self, value: Option<String>) -> Self {
        self.vendor = value;
        self
    }
    /// Set the vendor_id of the device.
    pub fn vendor_id(mut self, value: Option<String>) -> Self {
        self.vendor_id = value;
        self
    }
    /// Set the name of the device.
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    /// Set the series of the device.
    pub fn series(mut self, value: Option<String>) -> Self {
        self.series = value;
        self
    }
    /// Set the version of the device.
    pub fn version(mut self, value: String) -> Self {
        self.version = Some(value);
        self
    }
    /// Set the description of the device.
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }
    /// Set the license_text of the device.
    pub fn license_text(mut self, value: Option<String>) -> Self {
        self.license_text = value;
        self
    }
    /// Set the cpu of the device.
    pub fn cpu(mut self, value: Option<Cpu>) -> Self {
        self.cpu = value;
        self
    }
    /// Set the header_system_filename of the device.
    pub fn header_system_filename(mut self, value: Option<String>) -> Self {
        self.header_system_filename = value;
        self
    }
    /// Set the header_definitions_prefix of the device.
    pub fn header_definitions_prefix(mut self, value: Option<String>) -> Self {
        self.header_definitions_prefix = value;
        self
    }
    /// Set the address unit bits of the device.
    pub fn address_unit_bits(mut self, value: u32) -> Self {
        self.address_unit_bits = Some(value);
        self
    }
    /// Set the width of the device.
    pub fn width(mut self, value: u32) -> Self {
        self.width = Some(value);
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
    /// Set the xmlns_xs version of the device.
    pub fn xmlns_xs(mut self, value: String) -> Self {
        self.xmlns_xs = Some(value);
        self
    }
    /// Set the no_namespace_schema_location version of the device.
    pub fn no_namespace_schema_location(mut self, value: String) -> Self {
        self.no_namespace_schema_location = Some(value);
        self
    }
    /// Set the schema version of the device.
    pub fn schema_version(mut self, value: String) -> Self {
        self.schema_version = Some(value);
        self
    }

    /// XML global comments (usually license)
    pub fn comments(mut self, value: Vec<String>) -> Self {
        self.comments = value;
        self
    }

    /// Validate and build a [`Device`].
    pub fn build(self, lvl: ValidateLevel) -> Result<Device, SvdError> {
        let schema_version = self.schema_version.unwrap_or_else(default_schema_version);
        let device = Device {
            vendor: self.vendor,
            vendor_id: self.vendor_id,
            name: self
                .name
                .ok_or_else(|| BuildError::Uninitialized("name".to_string()))?,
            series: self.series,
            version: self
                .version
                .or_else(|| {
                    if !lvl.is_strict() {
                        Some("1.0".into())
                    } else {
                        None
                    }
                })
                .ok_or_else(|| BuildError::Uninitialized("version".to_string()))?,
            description: self
                .description
                .or_else(|| {
                    if !lvl.is_strict() {
                        Some("".into())
                    } else {
                        None
                    }
                })
                .ok_or_else(|| BuildError::Uninitialized("description".to_string()))?,
            license_text: self.license_text,
            cpu: self.cpu,
            header_system_filename: self.header_system_filename,
            header_definitions_prefix: self.header_definitions_prefix,
            address_unit_bits: self
                .address_unit_bits
                .or_else(|| if !lvl.is_strict() { Some(8) } else { None })
                .ok_or_else(|| BuildError::Uninitialized("addressUnitBits".to_string()))?,
            width: self
                .width
                .or_else(|| if !lvl.is_strict() { Some(32) } else { None })
                .ok_or_else(|| BuildError::Uninitialized("width".to_string()))?,
            default_register_properties: self.default_register_properties.build(lvl)?,
            peripherals: self
                .peripherals
                .ok_or_else(|| BuildError::Uninitialized("peripherals".to_string()))?,
            xmlns_xs: self.xmlns_xs.unwrap_or_else(default_xmlns_xs),
            no_namespace_schema_location: self
                .no_namespace_schema_location
                .unwrap_or_else(default_no_namespace_schema_location),
            schema_version,
            comments: self.comments,
        };
        device.validate(lvl)?;
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
        if builder.vendor.is_some() {
            self.vendor = builder.vendor.empty_to_none();
        }
        if builder.vendor_id.is_some() {
            self.vendor_id = builder.vendor_id.empty_to_none();
        }
        if let Some(name) = builder.name {
            self.name = name;
        }
        if builder.series.is_some() {
            self.series = builder.series.empty_to_none();
        }
        if let Some(version) = builder.version {
            self.version = version;
        }
        if let Some(description) = builder.description {
            self.description = description;
        }
        if builder.license_text.is_some() {
            self.license_text = builder.license_text.empty_to_none();
        }
        if builder.cpu.is_some() {
            self.cpu = builder.cpu;
        }
        if builder.header_system_filename.is_some() {
            self.header_system_filename = builder.header_system_filename.empty_to_none();
        }
        if builder.header_definitions_prefix.is_some() {
            self.header_definitions_prefix = builder.header_definitions_prefix.empty_to_none();
        }
        if let Some(address_unit_bits) = builder.address_unit_bits {
            self.address_unit_bits = address_unit_bits;
        }
        if let Some(width) = builder.width {
            self.width = width;
        }
        self.default_register_properties
            .modify_from(builder.default_register_properties, lvl)?;
        if let Some(peripherals) = builder.peripherals {
            self.peripherals = peripherals;
        }
        if let Some(xmlns_xs) = builder.xmlns_xs {
            self.xmlns_xs = xmlns_xs;
        }
        if let Some(no_namespace_schema_location) = builder.no_namespace_schema_location {
            self.no_namespace_schema_location = no_namespace_schema_location;
        }
        if let Some(schema_version) = builder.schema_version {
            self.schema_version = schema_version;
        }
        self.validate(lvl)
    }

    /// Validate the [`Device`]
    pub fn validate(&self, lvl: ValidateLevel) -> Result<(), SvdError> {
        if !lvl.is_disabled() {
            // TODO
            if self.peripherals.is_empty() {
                return Err(Error::EmptyDevice.into());
            }
        }
        Ok(())
    }
    /// Validate the [`Device`] recursively
    pub fn validate_all(&self, lvl: ValidateLevel) -> Result<(), SvdError> {
        if let Some(cpu) = self.cpu.as_ref() {
            cpu.validate(lvl)?;
        }
        self.default_register_properties.validate(lvl)?;
        for p in &self.peripherals {
            p.validate_all(lvl)?;
        }
        self.validate(lvl)
    }

    /// Get peripheral by name
    pub fn get_peripheral(&self, name: &str) -> Option<&Peripheral> {
        self.peripherals.iter().find(|f| f.name == name)
    }

    /// Get mutable peripheral by name
    pub fn get_mut_peripheral(&mut self, name: &str) -> Option<&mut Peripheral> {
        self.peripherals.iter_mut().find(|f| f.name == name)
    }
}

impl Name for Device {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Description for Device {
    fn description(&self) -> Option<&str> {
        Some(&self.description)
    }
}
