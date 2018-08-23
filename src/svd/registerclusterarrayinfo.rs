
use xmltree::Element;

use types::DimIndex;
use ::new_element;
use types::{Parse, Encode, parse_optional};
use elementext::ElementExt;

use ::error::{SVDError};

#[derive(Clone, Debug, PartialEq)]
pub struct RegisterClusterArrayInfo {
    pub dim: u32,
    pub dim_increment: u32,
    pub dim_index: Option<Vec<String>>,
    // Reserve the right to add more fields to this struct
    _extensible: (),
}

impl Parse for RegisterClusterArrayInfo {
    type Object = RegisterClusterArrayInfo;
    type Error = SVDError;

    fn parse(tree: &Element) -> Result<RegisterClusterArrayInfo, SVDError> {
        Ok(RegisterClusterArrayInfo {
            dim: tree.get_child_u32("dim")?,
            dim_increment: tree.get_child_u32("dimIncrement")?,
            dim_index: parse_optional::<DimIndex>("dimIndex", tree)?,
            _extensible: (),
        })
    }
}

impl Encode for RegisterClusterArrayInfo {
    type Error = SVDError;

    fn encode(&self) -> Result<Element, SVDError> {
        let mut e = new_element("registerClusterArrayInfo", None);

        e.children.push(new_element("dim", Some(format!("{}", self.dim))));
        e.children.push(new_element("dimIncrement", Some(format!("{}", self.dim_increment))));

        if let Some(ref di) = self.dim_index {
            e.children.push(new_element("dimIndex", Some(di.join(","))));
        }

        Ok(e)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use run_test;

    #[test]
    fn decode_encode() {          
        let tests = vec![
            (RegisterClusterArrayInfo {
                dim: 100,
                dim_increment: 4,
                dim_index: Some(vec!["10".to_owned(), "20".to_owned()]),
                _extensible: (),
            },
            "<registerClusterArrayInfo>
                <dim>100</dim>
                <dimIncrement>4</dimIncrement>
                <dimIndex>10,20</dimIndex>
            </registerClusterArrayInfo>
            ")
        ];

        run_test::<RegisterClusterArrayInfo>(&tests[..]);
    }
}
