use super::*;
use crate::svd::{ClusterInfo, RegisterCluster, RegisterProperties};

impl Parse for ClusterInfo {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, config: &Self::Config) -> Result<Self, Self::Error> {
        ClusterInfo::builder()
            .name(tree.get_child_text("name")?)
            .description(tree.get_child_text_opt("description")?)
            .alternate_cluster(tree.get_child_text_opt("alternateCluster")?)
            .header_struct_name(tree.get_child_text_opt("headerStructName")?)
            .address_offset(tree.get_child_u32("addressOffset")?)
            .default_register_properties(RegisterProperties::parse(tree, config)?)
            .children({
                let children: Result<Vec<_>, _> = tree
                    .children()
                    .filter(|t| {
                        t.is_element() && (t.has_tag_name("register") || t.has_tag_name("cluster"))
                    })
                    .map(|t| RegisterCluster::parse(&t, config))
                    .collect();
                children?
            })
            .derived_from(tree.attribute("derivedFrom").map(|s| s.to_owned()))
            .build(config.validate_level)
            .map_err(|e| SVDError::from(e).at(tree.id()))
    }
}
