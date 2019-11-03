use crate::elementext::ElementExt;
use xmltree::Element;

use crate::types::Parse;

#[cfg(feature = "unproven")]
use crate::encode::{Encode, EncodeChildren};
#[cfg(feature = "unproven")]
use crate::new_element;

use crate::error::SVDError;
use crate::svd::{registercluster::RegisterCluster, registerproperties::RegisterProperties};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
#[cfg_attr(feature = "builder", builder(build_fn(validate = "Self::validate")))]
#[derive(Clone, Debug, PartialEq)]
pub struct ClusterInfo {
    pub name: String,
    pub derived_from: Option<String>,
    pub description: String,
    pub header_struct_name: Option<String>,
    pub address_offset: u32,
    pub default_register_properties: RegisterProperties,
    pub children: Vec<RegisterCluster>,
    // Reserve the right to add more fields to this struct
    _extensible: (),
}

#[cfg(feature = "builder")]
impl ClusterInfoBuilder {
    fn validate(&self) -> Result<(), String> {
        if let Some(ref name) = self.name {
            if crate::is_valid_name(&name) {
                Ok(())
            } else {
                Err("Name is invalid".to_string())
            }
        } else {
            Err("ClusterInfo must have name".to_string())
        }
    }
}

impl Parse for ClusterInfo {
    type Object = ClusterInfo;
    type Error = SVDError;

    fn parse(tree: &Element) -> Result<ClusterInfo, SVDError> {
        Ok(ClusterInfo {
            name: tree.get_child_text("name")?, // TODO: Handle naming of cluster
            derived_from: tree.attributes.get("derivedFrom").map(|s| s.to_owned()),
            description: tree.get_child_text("description")?,
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
    type Error = SVDError;
    fn encode(&self) -> Result<Element, SVDError> {
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
