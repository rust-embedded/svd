use crate::elementext::ElementExt;
use crate::NS;
use minidom::Element;

use crate::parse;
use crate::types::Parse;

use crate::encode::{Encode, EncodeChildren};
use crate::error::*;

use crate::new_element;
use crate::svd::{cpu::Cpu, peripheral::Peripheral, registerproperties::RegisterProperties};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Device {
    /// The string identifies the device or device series. Device names are required to be unique
    pub name: String,

    /// Specify the compliant CMSIS-SVD schema version
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    schema_version: Option<String>,

    /// Define the version of the SVD file
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub version: Option<String>,

    /// Describe the main features of the device (for example CPU, clock frequency, peripheral overview)
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<String>,

    /// Define the number of data bits uniquely selected by each address
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub address_unit_bits: Option<u32>,

    /// Define the number of data bit-width of the maximum single data transfer supported by the bus infrastructure
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub width: Option<u32>,

    /// Describe the processor included in the device
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub cpu: Option<Cpu>,

    /// Group to define peripherals
    pub peripherals: Vec<Peripheral>,

    pub default_register_properties: RegisterProperties,

    // Reserve the right to add more fields to this struct
    #[cfg_attr(feature = "serde", serde(skip))]
    _extensible: (),
}

#[derive(Clone, Debug, Default)]
pub struct DeviceBuilder {
    name: Option<String>,
    schema_version: Option<String>,
    version: Option<String>,
    description: Option<String>,
    address_unit_bits: Option<u32>,
    width: Option<u32>,
    cpu: Option<Cpu>,
    peripherals: Option<Vec<Peripheral>>,
    default_register_properties: RegisterProperties,
}

impl DeviceBuilder {
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    pub fn schema_version(mut self, value: Option<String>) -> Self {
        self.schema_version = value;
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
    pub fn address_unit_bits(mut self, value: Option<u32>) -> Self {
        self.address_unit_bits = value;
        self
    }
    pub fn width(mut self, value: Option<u32>) -> Self {
        self.width = value;
        self
    }
    pub fn cpu(mut self, value: Option<Cpu>) -> Self {
        self.cpu = value;
        self
    }
    pub fn peripherals(mut self, value: Vec<Peripheral>) -> Self {
        self.peripherals = Some(value);
        self
    }
    pub fn default_register_properties(mut self, value: RegisterProperties) -> Self {
        self.default_register_properties = value;
        self
    }
    pub fn build(self) -> Result<Device> {
        (Device {
            name: self
                .name
                .ok_or_else(|| BuildError::Uninitialized("name".to_string()))?,
            schema_version: self.schema_version,
            version: self.version,
            description: self.description,
            address_unit_bits: self.address_unit_bits,
            width: self.width,
            cpu: self.cpu,
            peripherals: self
                .peripherals
                .ok_or_else(|| BuildError::Uninitialized("peripherals".to_string()))?,
            default_register_properties: self.default_register_properties,
            _extensible: (),
        })
        .validate()
    }
}

impl Device {
    fn validate(self) -> Result<Self> {
        // TODO
        if self.peripherals.is_empty() {
            return Err(SVDError::EmptyDevice)?;
        }
        Ok(self)
    }
}

impl Parse for Device {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        if tree.name() != "device" {
            return Err(SVDError::NotExpectedTag(tree.clone(), "device".to_string()).into());
        }
        let name = tree.get_child_text("name")?;
        Self::_parse(tree, name.clone()).with_context(|| format!("In device `{}`", name))
    }
}

impl Device {
    /// Parses a SVD file
    fn _parse(tree: &Element, name: String) -> Result<Self> {
        DeviceBuilder::default()
            .name(name)
            .schema_version(tree.attr("schemaVersion").map(|s| s.to_string()))
            .cpu(parse::optional::<Cpu>("cpu", tree)?)
            .version(tree.get_child_text_opt("version")?)
            .description(tree.get_child_text_opt("description")?)
            .address_unit_bits(parse::optional::<u32>("addressUnitBits", tree)?)
            .width(None)
            .peripherals({
                let ps: Result<Vec<_>, _> = tree
                    .get_child_elem("peripherals")?
                    .children()
                    .map(Peripheral::parse)
                    .collect();
                ps?
            })
            .default_register_properties(RegisterProperties::parse(tree)?)
            .build()
    }
}

impl Encode for Device {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        let mut e = Element::builder("device", NS)
            .attr("xmlns:xs", "http://www.w3.org/2001/XMLSchema-instance")
            .append(new_element("name", Some(self.name.clone())));

        if let Some(schema_version) = &self.schema_version {
            e = e.attr("schemaVersion", schema_version);
        }
        if let Some(schema_version) = &self.schema_version {
            e = e.attr(
                "xs:noNamespaceSchemaLocation",
                format!("CMSIS-SVD_Schema_{}.xsd", schema_version),
            );
        }

        if let Some(v) = &self.version {
            e = e.append(new_element("version", Some(v.clone())));
        }

        if let Some(v) = &self.description {
            e = e.append(new_element("description", Some(v.clone())));
        }

        if let Some(v) = &self.address_unit_bits {
            e = e.append(new_element("addressUnitBits", Some(format!("{}", v))));
        }

        if let Some(v) = &self.width {
            e = e.append(new_element("width", Some(format!("{}", v))));
        }

        e = e.append_all(self.default_register_properties.encode()?);

        if let Some(v) = &self.cpu {
            e = e.append(v.encode()?);
        }

        let peripherals: Result<Vec<_>, _> =
            self.peripherals.iter().map(Peripheral::encode).collect();
        e = e.append(Element::builder("peripherals", NS).append_all(peripherals?));

        Ok(e.build())
    }
}

// TODO: test device encoding and decoding
