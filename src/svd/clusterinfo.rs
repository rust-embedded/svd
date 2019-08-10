use elementext::ElementExt;
use xmltree::Element;

#[cfg(feature = "unproven")]
use encode::Encode;
#[cfg(feature = "unproven")]
use new_element;
use parse::ParseDefaults;

use error::SVDError;
use svd::defaults::Defaults;
use svd::registercluster::RegisterCluster;

#[derive(Clone, Debug, PartialEq)]
pub struct ClusterInfo {
    pub name: String,
    pub description: String,
    pub header_struct_name: Option<String>,
    pub address_offset: u32,
    pub defaults: Defaults,
    pub children: Vec<RegisterCluster>,
    // Reserve the right to add more fields to this struct
    _extensible: (),
}

impl ParseDefaults for ClusterInfo {
    type Object = ClusterInfo;
    type Error = SVDError;

    fn parse(tree: &Element, defaults: Defaults) -> Result<ClusterInfo, SVDError> {
        let defaults = Defaults::parse(tree, defaults)?;
        Ok(ClusterInfo {
            name: tree.get_child_text("name")?, // TODO: Handle naming of cluster
            description: tree.get_child_text("description")?,
            header_struct_name: tree.get_child_text_opt("headerStructName")?,
            address_offset: tree.get_child_u32("addressOffset")?,
            defaults,
            children: {
                let children: Result<Vec<_>, _> = tree.children
                    .iter()
                    .filter(|t| t.name == "register" || t.name == "cluster")
                    .map(|rc| RegisterCluster::parse(rc, defaults))
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
