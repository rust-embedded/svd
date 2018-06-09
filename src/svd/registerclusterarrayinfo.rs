
use xmltree::Element;

use parse;
use types::{Parse, Encode, new_element};
use ElementExt;

use ::error::{SVDError, SVDErrorKind};

#[derive(Clone, Debug, PartialEq)]
pub struct RegisterClusterArrayInfo {
    pub dim: u32,
    pub dim_increment: u32,
    pub dim_index: Option<Vec<String>>,
}

impl Parse for RegisterClusterArrayInfo {
    type Object = RegisterClusterArrayInfo;
    type Error = SVDError;

    fn parse(tree: &Element) -> Result<RegisterClusterArrayInfo, SVDError> {
        Ok(RegisterClusterArrayInfo {
            dim: tree.get_child_u32("dim")?,
            dim_increment: tree.get_child_u32("dimIncrement")?,
            dim_index: parse::optional::<parse::DimIndex>("dimIndex", tree)?,
        })
    }
}

impl Encode for RegisterClusterArrayInfo {
    type Error = SVDError;
    fn encode(&self) -> Result<Element, SVDError> {
        // TODO: support RegisterClusterArrayInfo encoding
        let _ = new_element("fake", None);

        Err(SVDError::from(SVDErrorKind::EncodeNotImplemented(String::from("RegisterClusterArrayInfo"))))
    }
}

//TODO: test RegisterClusterArrayInfo encode and decode
