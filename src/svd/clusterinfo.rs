use elementext::ElementExt;
use xmltree::Element;

use types::Parse;

#[cfg(feature = "unproven")]
use encode::Encode;
#[cfg(feature = "unproven")]
use new_element;
use parse;

use error::SVDError;
use svd::access::Access;
use svd::registercluster::RegisterCluster;

#[cfg(feature = "serde_svd")]
use super::serde::{ Deserialize, Serialize };

#[cfg_attr(feature = "serde_svd", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct ClusterInfo {
    pub name: String,
    pub description: String,
    pub header_struct_name: Option<String>,
    pub address_offset: u32,
    pub size: Option<u32>,
    pub access: Option<Access>,
    pub reset_value: Option<u32>,
    pub reset_mask: Option<u32>,
    pub children: Vec<RegisterCluster>,
    // Reserve the right to add more fields to this struct
    _extensible: (),
}

impl Parse for ClusterInfo {
    type Object = ClusterInfo;
    type Error = SVDError;

    fn parse(tree: &Element) -> Result<ClusterInfo, SVDError> {
        Ok(ClusterInfo {
            name: tree.get_child_text("name")?, // TODO: Handle naming of cluster
            description: tree.get_child_text("description")?,
            header_struct_name: tree.get_child_text_opt("headerStructName")?,
            address_offset: tree.get_child_u32("addressOffset")?,
            size: parse::optional::<u32>("size", tree)?,
            //access: tree.get_child("access").map(|t| Access::parse(t).ok() ),
            access: parse::optional::<Access>("access", tree)?,
            reset_value: parse::optional::<u32>("resetValue", tree)?,
            reset_mask: parse::optional::<u32>("resetMask", tree)?,
            children: {
                let children: Result<Vec<_>, _> = tree.children
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

        e.children.push(new_element(
            "description",
            Some(self.description.clone()),
        ));

        if let Some(ref v) = self.header_struct_name {
            e.children
                .push(new_element("headerStructName", Some(v.clone())));
        }

        e.children.push(new_element(
            "addressOffset",
            Some(format!("{}", self.address_offset)),
        ));

        if let Some(ref v) = self.size {
            e.children
                .push(new_element("size", Some(format!("{}", v))));
        }

        if let Some(ref v) = self.access {
            e.children.push(v.encode()?);
        }

        if let Some(ref v) = self.reset_value {
            e.children.push(new_element(
                "resetValue",
                Some(format!("{}", v)),
            ));
        }

        if let Some(ref v) = self.reset_mask {
            e.children
                .push(new_element("resetMask", Some(format!("{}", v))));
        }

        for c in &self.children {
            e.children.push(c.encode()?);
        }

        Ok(e)
    }
}

// TODO: test ClusterInfo encode and decode
