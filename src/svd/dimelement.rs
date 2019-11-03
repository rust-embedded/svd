use xmltree::Element;

use crate::types::{parse_optional, DimIndex, Parse};

use crate::elementext::ElementExt;
#[cfg(feature = "unproven")]
use crate::encode::Encode;
#[cfg(feature = "unproven")]
use crate::new_element;

use crate::error::*;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, derive_builder::Builder)]
pub struct DimElement {
    /// Defines the number of elements in an array or list
    pub dim: u32,

    /// Specify the address increment between two neighboring array or list members in the address map
    pub dim_increment: u32,

    /// Specify the strings that substitue the placeholder `%s` within `name` and `displayName`.
    /// By default, <dimIndex> is a value starting with 0
    #[builder(default)]
    pub dim_index: Option<Vec<String>>,

    // Reserve the right to add more fields to this struct
    #[builder(default)]
    _extensible: (),
}

impl Parse for DimElement {
    type Object = DimElement;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<DimElement> {
        DimElementBuilder::default()
            .dim(tree.get_child_u32("dim")?)
            .dim_increment(tree.get_child_u32("dimIncrement")?)
            .dim_index(parse_optional::<DimIndex>("dimIndex", tree)?)
            .build()
            .map_err(|e| anyhow::anyhow!(e))
    }
}

#[cfg(feature = "unproven")]
impl Encode for DimElement {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        let mut e = new_element("dimElement", None);

        e.children
            .push(new_element("dim", Some(format!("{}", self.dim))));
        e.children.push(new_element(
            "dimIncrement",
            Some(format!("{}", self.dim_increment)),
        ));

        if let Some(di) = &self.dim_index {
            e.children.push(new_element("dimIndex", Some(di.join(","))));
        }

        Ok(e)
    }
}

#[cfg(test)]
#[cfg(feature = "unproven")]
mod tests {
    use super::*;
    use crate::run_test;

    #[test]
    fn decode_encode() {
        let tests = vec![(
            DimElementBuilder::default()
                .dim(100)
                .dim_increment(4)
                .dim_index(Some(vec!["10".to_string(), "20".to_string()]))
                .build()
                .unwrap(),
            "<dimElement>
                <dim>100</dim>
                <dimIncrement>4</dimIncrement>
                <dimIndex>10,20</dimIndex>
            </dimElement>
            ",
        )];

        run_test::<DimElement>(&tests[..]);
    }
}
