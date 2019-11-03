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
#[derive(Clone, Debug, PartialEq, derive_builder::Builder)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct ClusterInfo {
    /// String to identify the cluster.
    /// Cluster names are required to be unique within the scope of a peripheral
    pub name: String,

    /// Specify the cluster name from which to inherit data.
    /// Elements specified subsequently override inherited values
    #[builder(default)]
    pub derived_from: Option<String>,

    /// String describing the details of the register cluster
    pub description: String,

    /// Specify the struct type name created in the device header file
    #[builder(default)]
    pub header_struct_name: Option<String>,

    /// Cluster address relative to the `baseAddress` of the peripheral
    pub address_offset: u32,

    #[builder(default)]
    pub default_register_properties: RegisterProperties,

    pub children: Vec<RegisterCluster>,

    // Reserve the right to add more fields to this struct
    #[builder(default)]
    _extensible: (),
}

impl ClusterInfoBuilder {
    fn validate(&self) -> Result<(), String> {
        match &self.name {
            Some(name) if crate::is_valid_name(name) => Ok(()),
            Some(name) => Err(format!("Cluster name `{}` is invalid", name)),
            None => Err("ClusterInfo must have name".to_string()),
        }
    }
}

impl Parse for ClusterInfo {
    type Object = ClusterInfo;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<ClusterInfo> {
        ClusterInfoBuilder::default()
            .name(tree.get_child_text("name")?)
            .derived_from(tree.attributes.get("derivedFrom").map(|s| s.to_owned()))
            .description(tree.get_child_text("description")?)
            .header_struct_name(tree.get_child_text_opt("headerStructName")?)
            .address_offset(tree.get_child_u32("addressOffset")?)
            .default_register_properties(RegisterProperties::parse(tree)?)
            .children({
                let children: Result<Vec<_>, _> = tree
                    .children
                    .iter()
                    .filter(|t| t.name == "register" || t.name == "cluster")
                    .map(RegisterCluster::parse)
                    .collect();
                children?
            })
            .build()
            .map_err(|e| anyhow::anyhow!(e))
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
            .push(new_element("description", Some(self.description.clone())));

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
