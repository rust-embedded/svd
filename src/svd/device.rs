use crate::elementext::ElementExt;
#[cfg(feature = "unproven")]
use std::collections::HashMap;
use xmltree::Element;

use crate::parse;
use crate::types::Parse;

#[cfg(feature = "unproven")]
use crate::encode::{Encode, EncodeChildren};
use crate::error::*;
#[cfg(feature = "unproven")]
use crate::new_element;
use crate::svd::{cpu::Cpu, peripheral::Peripheral, registerproperties::RegisterProperties};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, derive_builder::Builder)]
pub struct Device {
    /// The string identifies the device or device series. Device names are required to be unique
    pub name: String,

    /// Specify the compliant CMSIS-SVD schema version
    #[builder(default)]
    schema_version: Option<String>,

    /// Define the version of the SVD file
    #[builder(default)]
    pub version: Option<String>,

    /// Describe the main features of the device (for example CPU, clock frequency, peripheral overview)
    #[builder(default)]
    pub description: Option<String>,

    /// Define the number of data bits uniquely selected by each address
    #[builder(default)]
    pub address_unit_bits: Option<u32>,

    /// Define the number of data bit-width of the maximum single data transfer supported by the bus infrastructure
    #[builder(default)]
    pub width: Option<u32>,

    /// Describe the processor included in the device
    #[builder(default)]
    pub cpu: Option<Cpu>,

    /// Group to define peripherals
    pub peripherals: Vec<Peripheral>,

    #[builder(default)]
    pub default_register_properties: RegisterProperties,

    // Reserve the right to add more fields to this struct
    #[builder(default)]
    _extensible: (),
}

impl Parse for Device {
    type Object = Device;
    type Error = anyhow::Error;

    /// Parses a SVD file
    fn parse(tree: &Element) -> Result<Device> {
        DeviceBuilder::default()
            .name(tree.get_child_text("name")?)
            .schema_version(tree.attributes.get("schemaVersion").cloned())
            .cpu(parse::optional::<Cpu>("cpu", tree)?)
            .version(tree.get_child_text_opt("version")?)
            .description(tree.get_child_text_opt("description")?)
            .address_unit_bits(parse::optional::<u32>("addressUnitBits", tree)?)
            .width(None)
            .peripherals({
                let ps: Result<Vec<_>, _> = tree
                    .get_child_elem("peripherals")?
                    .children
                    .iter()
                    .map(Peripheral::parse)
                    .collect();
                ps?
            })
            .default_register_properties(RegisterProperties::parse(tree)?)
            .build()
            .map_err(|e| anyhow::anyhow!(e))
    }
}

#[cfg(feature = "unproven")]
impl Encode for Device {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        let mut elem = Element {
            prefix: None,
            namespace: None,
            namespaces: None,
            name: String::from("device"),
            attributes: HashMap::new(),
            children: vec![new_element("name", Some(self.name.clone()))],
            text: None,
        };

        elem.attributes.insert(
            String::from("xmlns:xs"),
            String::from("http://www.w3.org/2001/XMLSchema-instance"),
        );
        if let Some(schema_version) = &self.schema_version {
            elem.attributes
                .insert(String::from("schemaVersion"), format!("{}", schema_version));
        }
        if let Some(schema_version) = &self.schema_version {
            elem.attributes.insert(
                String::from("xs:noNamespaceSchemaLocation"),
                format!("CMSIS-SVD_Schema_{}.xsd", schema_version),
            );
        }

        if let Some(v) = &self.version {
            elem.children.push(new_element("version", Some(v.clone())));
        }

        if let Some(v) = &self.description {
            elem.children
                .push(new_element("description", Some(v.clone())));
        }

        if let Some(v) = &self.description {
            elem.children
                .push(new_element("addressUnitBits", Some(format!("{}", v))));
        }

        if let Some(v) = &self.width {
            elem.children
                .push(new_element("width", Some(format!("{}", v))));
        }

        elem.children
            .extend(self.default_register_properties.encode()?);

        if let Some(v) = &self.cpu {
            elem.children.push(v.encode()?);
        }

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

        Ok(elem)
    }
}

// TODO: test device encoding and decoding
