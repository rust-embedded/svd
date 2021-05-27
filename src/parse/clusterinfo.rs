use super::{elementext::ElementExt, Context, Element, Parse, Result};
use crate::svd::{ClusterInfo, RegisterCluster, RegisterProperties};

impl Parse for ClusterInfo {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        let name = tree.get_child_text("name")?;
        parse_cluster(tree, name.clone()).with_context(|| format!("In cluster `{}`", name))
    }
}

fn parse_cluster(tree: &Element, name: String) -> Result<ClusterInfo> {
    Ok(ClusterInfo::builder()
        .name(name)
        .description(tree.get_child_text_opt("description")?)
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
        .derived_from(tree.attributes.get("derivedFrom").map(|s| s.to_owned()))
        .build()?)
}
