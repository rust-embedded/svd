use std::collections::HashMap;

use xmltree::Element;

use crate::elementext::ElementExt;
use crate::parse;

use crate::encode::{Encode, EncodeChildren};

use crate::new_element;
use crate::types::Parse;

use crate::error::*;
use crate::svd::{
    addressblock::AddressBlock, interrupt::Interrupt, registercluster::RegisterCluster,
    registerproperties::RegisterProperties,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Peripheral {
    /// The string identifies the peripheral. Peripheral names are required to be unique for a device
    pub name: String,

    /// Lowest address reserved or used by the peripheral
    pub base_address: u64,

    /// The string specifies the version of this peripheral description
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub version: Option<String>,

    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub display_name: Option<String>,

    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub group_name: Option<String>,

    /// The string provides an overview of the purpose and functionality of the peripheral
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<String>,

    /// Specify an address range uniquely mapped to this peripheral
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub address_block: Option<AddressBlock>,

    /// A peripheral can have multiple associated interrupts
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub interrupt: Vec<Interrupt>,

    pub default_register_properties: RegisterProperties,

    /// Group to enclose register definitions.
    /// `None` indicates that the `<registers>` node is not present
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub registers: Option<Vec<RegisterCluster>>,

    /// Specify the peripheral name from which to inherit data. Elements specified subsequently override inherited values
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub derived_from: Option<String>,

    // Reserve the right to add more fields to this struct
    #[cfg_attr(feature = "serde", serde(skip))]
    _extensible: (),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PeripheralBuilder {
    name: Option<String>,
    base_address: Option<u64>,
    version: Option<String>,
    display_name: Option<String>,
    group_name: Option<String>,
    description: Option<String>,
    address_block: Option<AddressBlock>,
    interrupt: Vec<Interrupt>,
    default_register_properties: RegisterProperties,
    registers: Option<Vec<RegisterCluster>>,
    derived_from: Option<String>,
}

impl PeripheralBuilder {
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    pub fn base_address(mut self, value: u64) -> Self {
        self.base_address = Some(value);
        self
    }
    pub fn version(mut self, value: Option<String>) -> Self {
        self.version = value;
        self
    }
    pub fn display_name(mut self, value: Option<String>) -> Self {
        self.display_name = value;
        self
    }
    pub fn group_name(mut self, value: Option<String>) -> Self {
        self.group_name = value;
        self
    }
    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }
    pub fn address_block(mut self, value: Option<AddressBlock>) -> Self {
        self.address_block = value;
        self
    }
    pub fn interrupt(mut self, value: Vec<Interrupt>) -> Self {
        self.interrupt = value;
        self
    }
    pub fn default_register_properties(mut self, value: RegisterProperties) -> Self {
        self.default_register_properties = value;
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
    pub fn build(self) -> Result<Peripheral> {
        (Peripheral {
            name: self
                .name
                .ok_or_else(|| BuildError::Uninitialized("name".to_string()))?,
            base_address: self
                .base_address
                .ok_or_else(|| BuildError::Uninitialized("base_address".to_string()))?,
            version: self.version,
            display_name: self.display_name,
            group_name: self.group_name,
            description: self.description,
            address_block: self.address_block,
            interrupt: self.interrupt,
            default_register_properties: self.default_register_properties,
            registers: self.registers,
            derived_from: self.derived_from,
            _extensible: (),
        })
        .validate()
    }
}

impl Peripheral {
    fn validate(self) -> Result<Self> {
        // TODO
        check_dimable_name(&self.name, "name")?;
        if let Some(name) = self.derived_from.as_ref() {
            check_dimable_name(name, "derivedFrom")?;
        } else if let Some(registers) = self.registers.as_ref() {
            if registers.is_empty() {
                #[cfg(feature = "strict")]
                return Err(SVDError::EmptyRegisters)?;
            }
        }
        Ok(self)
    }
}

impl Parse for Peripheral {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        if tree.name != "peripheral" {
            return Err(SVDError::NotExpectedTag(tree.clone(), "peripheral".to_string()).into());
        }
        let name = tree.get_child_text("name")?;
        Self::_parse(tree, name.clone()).with_context(|| format!("In peripheral `{}`", name))
    }
}

impl Peripheral {
    fn _parse(tree: &Element, name: String) -> Result<Self> {
        PeripheralBuilder::default()
            .name(name)
            .version(tree.get_child_text_opt("version")?)
            .display_name(tree.get_child_text_opt("displayName")?)
            .group_name(tree.get_child_text_opt("groupName")?)
            .description(tree.get_child_text_opt("description")?)
            .base_address(tree.get_child_u64("baseAddress")?)
            .address_block(parse::optional::<AddressBlock>("addressBlock", tree)?)
            .interrupt({
                let interrupt: Result<Vec<_>, _> = tree
                    .children
                    .iter()
                    .filter(|t| t.name == "interrupt")
                    .enumerate()
                    .map(|(e, i)| {
                        Interrupt::parse(i).with_context(|| format!("Parsing interrupt #{}", e))
                    })
                    .collect();
                interrupt?
            })
            .default_register_properties(RegisterProperties::parse(tree)?)
            .registers(if let Some(registers) = tree.get_child("registers") {
                let rs: Result<Vec<_>, _> = registers
                    .children
                    .iter()
                    .map(RegisterCluster::parse)
                    .collect();
                Some(rs?)
            } else {
                None
            })
            .derived_from(tree.attributes.get("derivedFrom").map(|s| s.to_owned()))
            .build()
    }
}

impl Encode for Peripheral {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        let mut elem = Element {
            prefix: None,
            namespace: None,
            namespaces: None,
            name: String::from("peripheral"),
            attributes: HashMap::new(),
            children: vec![new_element("name", Some(self.name.clone()))],
            text: None,
        };

        if let Some(v) = &self.version {
            elem.children
                .push(new_element("version", Some(v.to_string())));
        };
        if let Some(v) = &self.display_name {
            elem.children
                .push(new_element("displayName", Some(v.to_string())));
        };
        if let Some(v) = &self.group_name {
            elem.children
                .push(new_element("groupName", Some(v.to_string())));
        };
        if let Some(v) = &self.description {
            elem.children
                .push(new_element("description", Some(v.to_string())));
        };
        elem.children.push(new_element(
            "baseAddress",
            Some(format!("0x{:.08x}", self.base_address)),
        ));

        elem.children
            .extend(self.default_register_properties.encode()?);

        if let Some(v) = &self.address_block {
            elem.children.push(v.encode()?);
        };

        let interrupts: Result<Vec<_>, _> = self.interrupt.iter().map(Interrupt::encode).collect();

        elem.children.append(&mut interrupts?);

        if let Some(v) = &self.registers {
            let children: Result<Vec<_>, _> = v.iter().map(|e| e.encode()).collect();

            elem.children.push(Element {
                prefix: None,
                namespace: None,
                namespaces: None,
                name: String::from("registers"),
                attributes: HashMap::new(),
                children: children?,
                text: None,
            });
        };

        if let Some(v) = &self.derived_from {
            elem.attributes
                .insert(String::from("derivedFrom"), v.to_string());
        }

        Ok(elem)
    }
}

// TODO: add Peripheral encode / decode tests
