use super::{elementext::ElementExt, Context, Node, Parse, Result};
use crate::svd::{ClusterInfo, RegisterCluster, RegisterProperties};

impl Parse for ClusterInfo {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Node) -> Result<Self> {
        let name = tree.get_child_text("name")?;
        parse_cluster(tree, name.clone()).with_context(|| format!("In cluster `{}`", name))
    }
}

fn parse_cluster(tree: &Node, name: String) -> Result<ClusterInfo> {
    Ok(ClusterInfo::builder()
        .name(name)
        .description(tree.get_child_text_opt("description")?)
        .header_struct_name(tree.get_child_text_opt("headerStructName")?)
        .address_offset(tree.get_child_u32("addressOffset")?)
        .default_register_properties(RegisterProperties::parse(tree)?)
        .children({
            let children: Result<Vec<_>, _> = tree
                .children()
                .filter(|t| {
                    t.is_element() && (t.has_tag_name("register") || t.has_tag_name("cluster"))
                })
                .map(|t| RegisterCluster::parse(&t))
                .collect();
            children?
        })
        .derived_from(tree.attribute("derivedFrom").map(|s| s.to_owned()))
        .build()?)
}
