use crate::elementext::ElementExt;
use xmltree::Element;

use crate::types::Parse;

#[cfg(feature = "unproven")]
use crate::encode::{Encode, EncodeChildren};
#[cfg(feature = "unproven")]
use crate::new_element;

use crate::error::*;
use crate::svd::{registercluster::RegisterCluster, registerproperties::RegisterProperties};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct ClusterInfo {
    /// String to identify the cluster.
    /// Cluster names are required to be unique within the scope of a peripheral
    pub name: String,

    /// Cluster address relative to the `baseAddress` of the peripheral
    pub address_offset: u32,

    /// Specify the cluster name from which to inherit data.
    /// Elements specified subsequently override inherited values
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub derived_from: Option<String>,

    /// String describing the details of the register cluster
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<String>,

    /// Specify the struct type name created in the device header file
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub header_struct_name: Option<String>,

    pub default_register_properties: RegisterProperties,

    pub children: Vec<RegisterCluster>,

    // Reserve the right to add more fields to this struct
    #[cfg_attr(feature = "serde", serde(skip))]
    _extensible: (),
}

impl Parse for ClusterInfo {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        let name = tree.get_child_text("name")?;
        Self::_parse(tree, name.clone()).with_context(|| format!("In cluster `{}`", name))
    }
}

impl ClusterInfo {
    fn _parse(tree: &Element, name: String) -> Result<Self> {
        Ok(Self {
            name, // TODO: Handle naming of cluster
            derived_from: tree.attributes.get("derivedFrom").map(|s| s.to_owned()),
            description: tree.get_child_text_opt("description")?,
            header_struct_name: tree.get_child_text_opt("headerStructName")?,
            address_offset: tree.get_child_u32("addressOffset")?,
            default_register_properties: RegisterProperties::parse(tree)?,
            children: {
                let children: Result<Vec<_>, _> = tree
                    .children
                    .iter()
                    .filter(|t| t.name == "register" || t.name == "cluster")
                    .map(RegisterCluster::parse)
                    .collect();
                children?
            },
            _extensible: (),
        })
    }
}

#[cfg(feature = "unproven")]
impl Encode for ClusterInfo {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        let mut e = new_element("cluster", None);

        if let Some(v) = &self.derived_from {
            e.attributes
                .insert(String::from("derivedFrom"), format!("{}", v));
        }

        e.children
            .push(new_element("description", self.description.clone()));

        if let Some(v) = &self.header_struct_name {
            e.children
                .push(new_element("headerStructName", Some(v.clone())));
        }

        e.children.push(new_element(
            "addressOffset",
            Some(format!("{}", self.address_offset)),
        ));

        e.children
            .extend(self.default_register_properties.encode()?);

        for c in &self.children {
            e.children.push(c.encode()?);
        }

        Ok(e)
    }
}

// TODO: test ClusterInfo encode and decode
