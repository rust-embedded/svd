
use xmltree::Element;

use types::{Parse, Encode, new_element};
use parse;

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
            dim: parse::get_child_u32("dim", tree)?,
            dim_increment: parse::get_child_u32("dimIncrement", tree)?,
            dim_index: parse::optional("dimIndex", tree, |c| {
                parse::dim_index(&parse::get_text(c)?)
            })?,
        })
    }
}

impl Encode for RegisterClusterArrayInfo {
    type Error = SVDError;
    fn encode(&self) -> Result<Element, SVDError> {
        // TODO: encoding here
        let _ = new_element("fake", None);

        Err(SVDError::from(SVDErrorKind::EncodeNotImplemented(String::from("RegisterClusterArrayInfo"))))
    }
}
