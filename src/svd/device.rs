use crate::elementext::ElementExt;
#[cfg(feature = "unproven")]
use std::collections::HashMap;
use xmltree::Element;

use crate::parse;
use crate::types::Parse;

#[cfg(feature = "unproven")]
use crate::encode::{Encode, EncodeChildren};
use crate::error::SVDError;
#[cfg(feature = "unproven")]
use crate::new_element;
use crate::svd::{cpu::Cpu, peripheral::Peripheral, registerproperties::RegisterProperties};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
#[derive(Clone, Debug)]
pub struct Device {
    pub name: String,
    schema_version: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub address_unit_bits: Option<u32>,
    pub width: Option<u32>,
    pub cpu: Option<Cpu>,
    pub peripherals: Vec<Peripheral>,
    pub default_register_properties: RegisterProperties,
    // Reserve the right to add more fields to this struct
    _extensible: (),
}

impl Parse for Device {
    type Object = Device;
    type Error = SVDError;

    /// Parses a SVD file
    fn parse(tree: &Element) -> Result<Device, SVDError> {
        Ok(Device {
            name: tree.get_child_text("name")?,
            schema_version: tree.attributes.get("schemaVersion").cloned(),
            cpu: parse::optional::<Cpu>("cpu", tree)?,
            version: tree.get_child_text_opt("version")?,
            description: tree.get_child_text_opt("description")?,
            address_unit_bits: parse::optional::<u32>("addressUnitBits", tree)?,
            width: None,
            peripherals: {
                let ps: Result<Vec<_>, _> = tree
                    .get_child_elem("peripherals")?
                    .children
                    .iter()
                    .map(Peripheral::parse)
                    .collect();
                ps?
            },
            default_register_properties: RegisterProperties::parse(tree)?,
            _extensible: (),
        })
    }
}

#[cfg(feature = "unproven")]
impl Encode for Device {
    type Error = SVDError;

    fn encode(&self) -> Result<Element, SVDError> {
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
