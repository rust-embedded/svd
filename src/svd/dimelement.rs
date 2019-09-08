use xmltree::Element;

use crate::types::{parse_optional, DimIndex, Parse};

use crate::elementext::ElementExt;
#[cfg(feature = "unproven")]
use crate::encode::Encode;
#[cfg(feature = "unproven")]
use crate::new_element;

use crate::error::SVDError;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct DimElement {
    pub dim: u32,
    pub dim_increment: u32,
    pub dim_index: Option<Vec<String>>,
}

impl Parse for DimElement {
    type Object = DimElement;
    type Error = SVDError;

    fn parse(tree: &Element) -> Result<DimElement, SVDError> {
        Ok(DimElement {
            dim: tree.get_child_u32("dim")?,
            dim_increment: tree.get_child_u32("dimIncrement")?,
            dim_index: parse_optional::<DimIndex>("dimIndex", tree)?,
        })
    }
}

#[cfg(feature = "unproven")]
impl Encode for DimElement {
    type Error = SVDError;

    fn encode(&self) -> Result<Element, SVDError> {
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
            DimElement {
                dim: 100,
                dim_increment: 4,
                dim_index: Some(vec!["10".to_owned(), "20".to_owned()]),
            },
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
