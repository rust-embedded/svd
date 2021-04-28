use crate::elementext::ElementExt;
use std::collections::HashMap;
use xmltree::Element;

use rayon::prelude::*;

use crate::parse;
use crate::types::Parse;

use crate::encode::{Encode, EncodeChildren};
use crate::error::*;

use crate::new_element;
use crate::svd::{cpu::Cpu, peripheral::Peripheral, registerproperties::RegisterProperties};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Device {
    // vendor

    // vendorID
    /// The string identifies the device or device series. Device names are required to be unique
    pub name: String,

    // series
    /// Define the version of the SVD file
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub version: Option<String>,

    /// Describe the main features of the device (for example CPU, clock frequency, peripheral overview)
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<String>,

    // licenseText
    /// Describe the processor included in the device
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub cpu: Option<Cpu>,

    /// Define the number of data bits uniquely selected by each address
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub address_unit_bits: Option<u32>,

    /// Define the number of data bit-width of the maximum single data transfer supported by the bus infrastructure
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub width: Option<u32>,

    /// Default properties for all registers
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub default_register_properties: RegisterProperties,

    /// Group to define peripherals
    pub peripherals: Vec<Peripheral>,

    /// Specify the compliant CMSIS-SVD schema version
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    schema_version: Option<String>,
}

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
            cpu: d.cpu,
            address_unit_bits: d.address_unit_bits,
            width: d.width,
            default_register_properties: d.default_register_properties,
            peripherals: Some(d.peripherals),
            schema_version: d.schema_version,
        }
    }
}

impl DeviceBuilder {
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
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
    pub fn cpu(mut self, value: Option<Cpu>) -> Self {
        self.cpu = value;
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
    pub fn default_register_properties(mut self, value: RegisterProperties) -> Self {
        self.default_register_properties = value;
        self
    }
    pub fn peripherals(mut self, value: Vec<Peripheral>) -> Self {
        self.peripherals = Some(value);
        self
    }
    pub fn schema_version(mut self, value: Option<String>) -> Self {
        self.schema_version = value;
        self
    }
    pub fn build(self) -> Result<Device> {
        (Device {
            name: self
                .name
                .ok_or_else(|| BuildError::Uninitialized("name".to_string()))?,
            version: self.version,
            description: self.description,
            cpu: self.cpu,
            address_unit_bits: self.address_unit_bits,
            width: self.width,
            default_register_properties: self.default_register_properties,
            peripherals: self
                .peripherals
                .ok_or_else(|| BuildError::Uninitialized("peripherals".to_string()))?,
            schema_version: self.schema_version,
        })
        .validate()
    }
}

impl Device {
    fn validate(self) -> Result<Self> {
        // TODO
        if self.peripherals.is_empty() {
            return Err(SVDError::EmptyDevice.into());
        }
        Ok(self)
    }
}

impl Parse for Device {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        if tree.name != "device" {
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
            .version(tree.get_child_text_opt("version")?)
            .description(tree.get_child_text_opt("description")?)
            .cpu(parse::optional::<Cpu>("cpu", tree)?)
            .address_unit_bits(parse::optional::<u32>("addressUnitBits", tree)?)
            .width(parse::optional::<u32>("width", tree)?)
            .default_register_properties(RegisterProperties::parse(tree)?)
            .peripherals({
                let ps: Result<Vec<_>, _> = tree
                    .get_child_elem("peripherals")?
                    .children
                    .par_iter()
                    .map(Peripheral::parse)
                    .collect();
                ps?
            })
            .schema_version(tree.attributes.get("schemaVersion").cloned())
            .build()
    }
}

impl Encode for Device {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        let mut elem = new_element("device", None);
        elem.children
            .push(new_element("name", Some(self.name.clone())));

        if let Some(v) = &self.version {
            elem.children.push(new_element("version", Some(v.clone())));
        }

        if let Some(v) = &self.description {
            elem.children
                .push(new_element("description", Some(v.clone())));
        }

        if let Some(v) = &self.cpu {
            elem.children.push(v.encode()?);
        }

        if let Some(v) = &self.address_unit_bits {
            elem.children
                .push(new_element("addressUnitBits", Some(format!("{}", v))));
        }

        if let Some(v) = &self.width {
            elem.children
                .push(new_element("width", Some(format!("{}", v))));
        }

        elem.children
            .extend(self.default_register_properties.encode()?);

        let peripherals: Result<Vec<_>, _> =
            self.peripherals.iter().map(Peripheral::encode).collect();
        elem.children.push(Element {
            prefix: None,
            namespace: None,
            namespaces: None,
            name: String::from("peripherals"),
            attributes: HashMap::new(),
            children: peripherals?,
            text: None,
        });

        elem.attributes.insert(
            String::from("xmlns:xs"),
            String::from("http://www.w3.org/2001/XMLSchema-instance"),
        );
        if let Some(schema_version) = &self.schema_version {
            elem.attributes
                .insert(String::from("schemaVersion"), schema_version.to_string());
        }
        if let Some(schema_version) = &self.schema_version {
            elem.attributes.insert(
                String::from("xs:noNamespaceSchemaLocation"),
                format!("CMSIS-SVD_Schema_{}.xsd", schema_version),
            );
        }

        Ok(elem)
    }
}

// TODO: test device encoding and decoding
