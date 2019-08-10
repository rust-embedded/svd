#[cfg(feature = "unproven")]
use std::collections::HashMap;

use xmltree::Element;

use elementext::ElementExt;
use failure::ResultExt;
use parse::{self, Parse, ParseDefaults};

#[cfg(feature = "unproven")]
use encode::Encode;
#[cfg(feature = "unproven")]
use new_element;

use error::{SVDError, SVDErrorKind};
use svd::defaults::Defaults;
use svd::addressblock::AddressBlock;
use svd::interrupt::Interrupt;
use svd::registercluster::RegisterCluster;

#[derive(Clone, Debug)]
pub struct Peripheral {
    pub name: String,
    pub version: Option<String>,
    pub display_name: Option<String>,
    pub group_name: Option<String>,
    pub description: Option<String>,
    pub base_address: u32,
    pub address_block: Option<AddressBlock>,
    pub interrupt: Vec<Interrupt>,
    /// `None` indicates that the `<registers>` node is not present
    pub registers: Option<Vec<RegisterCluster>>,
    pub derived_from: Option<String>,
    pub defaults: Defaults,
    // Reserve the right to add more fields to this struct
    _extensible: (),
}

impl ParseDefaults for Peripheral {
    type Object = Peripheral;
    type Error = SVDError;

    fn parse(tree: &Element, defaults: Defaults) -> Result<Peripheral, SVDError> {
        if tree.name != "peripheral" {
            return Err(SVDErrorKind::NotExpectedTag(
                tree.clone(),
                format!("peripheral"),
            ).into());
        }
        let name = tree.get_child_text("name")?;
        Peripheral::_parse(tree, name.clone(), defaults)
            .context(SVDErrorKind::Other(format!(
                "In peripheral `{}`",
                name
            )))
            .map_err(|e| e.into())
    }
}

impl Peripheral {
    pub fn derive_from(&self, other: &Self) -> Self {
        let mut derived = self.clone();
        derived.group_name = derived
            .group_name
            .or_else(|| other.group_name.clone());
        derived.description = derived
            .description
            .or_else(|| other.description.clone());
        derived.registers = derived
            .registers
            .or_else(|| other.registers.clone());
        if derived.interrupt.is_empty() {
            derived.interrupt = other.interrupt.clone();
        }
        derived
    }

    fn _parse(tree: &Element, name: String, defaults: Defaults) -> Result<Self, SVDError> {
        let defaults = Defaults::parse(tree, defaults)?;
        Ok(Peripheral {
            name,
            version: tree.get_child_text_opt("version")?,
            display_name: tree.get_child_text_opt("displayName")?,
            group_name: tree.get_child_text_opt("groupName")?,
            description: tree.get_child_text_opt("description")?,
            base_address: tree.get_child_u32("baseAddress")?,
            defaults,
            address_block: parse::optional::<AddressBlock>(
                "addressBlock",
                tree,
            )?,
            interrupt: {
                let interrupt: Result<Vec<_>, _> = tree.children
                    .iter()
                    .filter(|t| t.name == "interrupt")
                    .enumerate()
                    .map(|(e, i)| {
                        Interrupt::parse(i).context(SVDErrorKind::Other(
                            format!("Parsing interrupt #{}", e).into(),
                        ))
                    })
                    .collect();
                interrupt?
            },
            registers: if let Some(registers) = tree.get_child("registers") {
                let rs: Result<Vec<_>, _> = registers
                    .children
                    .iter()
                    .map(|rc| RegisterCluster::parse(rc, defaults))
                    .collect();
                Some(rs?)
            } else {
                None
            },
            derived_from: tree.attributes
                .get("derivedFrom")
                .map(|s| s.to_owned()),
            _extensible: (),
        })
    }
}

#[cfg(feature = "unproven")]
impl Encode for Peripheral {
    type Error = SVDError;

    fn encode(&self) -> Result<Element, SVDError> {
        let mut elem = Element {
            name: String::from("peripheral"),
            attributes: HashMap::new(),
            children: vec![new_element("name", Some(self.name.clone()))],
            text: None,
        };

        match self.version {
            Some(ref v) => {
                elem.children
                    .push(new_element("version", Some(format!("{}", v))));
            }
            None => (),
        };
        match self.display_name {
            Some(ref v) => {
                elem.children.push(new_element(
                    "displayName",
                    Some(format!("{}", v)),
                ));
            }
            None => (),
        };
        match self.group_name {
            Some(ref v) => {
                elem.children
                    .push(new_element("groupName", Some(format!("{}", v))));
            }
            None => (),
        };
        match self.description {
            Some(ref v) => {
                elem.children.push(new_element(
                    "description",
                    Some(format!("{}", v)),
                ));
            }
            None => (),
        };
        elem.children.push(new_element(
            "baseAddress",
            Some(format!("0x{:.08x}", self.base_address)),
        ));
        match self.address_block {
            Some(ref v) => {
                elem.children.push(v.encode()?);
            }
            None => (),
        };

        let interrupts: Result<Vec<_>, _> = self.interrupt
            .iter()
            .map(Interrupt::encode)
            .collect();

        elem.children.append(&mut interrupts?);

        match self.registers {
            Some(ref v) => {
                let children: Result<Vec<_>, _> =
                    v.iter().map(|&ref e| e.encode()).collect();

                elem.children.push(Element {
                    name: String::from("registers"),
                    attributes: HashMap::new(),
                    children: children?,
                    text: None,
                });
            }
            None => (),
        };

        match self.derived_from {
            Some(ref v) => {
                elem.attributes
                    .insert(String::from("derivedFrom"), format!("{}", v));
            }
            None => (),
        }

        Ok(elem)
    }
}

// TODO: add Peripheral encode / decode tests
