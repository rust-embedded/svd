

use xmltree::Element;
use ElementExt;

use types::{Parse, Encode, new_element};
use parse;

use ::error::{SVDError, SVDErrorKind};
use ::svd::access::Access;
use ::svd::registercluster::RegisterCluster;

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
            address_offset: 
                parse::get_child_u32("addressOffset", tree)?,
            size: parse::optional("size", tree, parse::u32)?,
            //access: tree.get_child("access").map(|t| Access::parse(t).ok() ),
            access: parse::optional("access", tree, Access::parse)?,
            reset_value:
                parse::optional("resetValue", tree, parse::u32)?,
            reset_mask:
                parse::optional("resetMask", tree, parse::u32)?,
            children: {
                let children: Result<Vec<_>,_> = tree.children
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

impl Encode for ClusterInfo {
    type Error = SVDError;
    fn encode(&self) -> Result<Element, SVDError> {
        // TODO: encoding here
        let _ = new_element("fake", None);

        Err(SVDError::from(SVDErrorKind::EncodeNotImplemented(String::from("RegisterClusterArrayInfo"))))
    }
}

