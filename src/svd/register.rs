use std::ops::Deref;

use xmltree::Element;

use types::Parse;

#[cfg(feature = "unproven")]
use elementext::ElementExt;
#[cfg(feature = "unproven")]
use encode::Encode;
use error::SVDError;
use svd::registerclusterarrayinfo::RegisterClusterArrayInfo;
use svd::registerinfo::RegisterInfo;

#[derive(Clone, Debug, PartialEq)]
pub enum Register {
    Single(RegisterInfo),
    Array(RegisterInfo, RegisterClusterArrayInfo),
}

impl Deref for Register {
    type Target = RegisterInfo;

    fn deref(&self) -> &RegisterInfo {
        match *self {
            Register::Single(ref info) => info,
            Register::Array(ref info, _) => info,
        }
    }
}

impl Parse for Register {
    type Object = Register;
    type Error = SVDError;

    fn parse(tree: &Element) -> Result<Register, SVDError> {
        assert_eq!(tree.name, "register");

        let info = RegisterInfo::parse(tree)?;

        if tree.get_child("dimIncrement").is_some() {
            let array_info = RegisterClusterArrayInfo::parse(tree)?;
            assert!(info.name.contains("%s"));
            if let Some(ref indices) = array_info.dim_index {
                assert_eq!(array_info.dim as usize, indices.len())
            }
            Ok(Register::Array(info, array_info))
        } else {
            Ok(Register::Single(info))
        }
    }
}

#[cfg(feature = "unproven")]
impl Encode for Register {
    type Error = SVDError;

    fn encode(&self) -> Result<Element, SVDError> {
        match *self {
            Register::Single(ref info) => info.encode(),
            Register::Array(ref info, ref array_info) => {
                // TODO: is this correct? probably not, need tests
                let base = info.encode()?;
                base.merge(&array_info.encode()?);
                Ok(base)
            }
        }
    }
}

// TODO: add Register encode and decode tests
