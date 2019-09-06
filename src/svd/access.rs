use xmltree::Element;

use crate::elementext::ElementExt;
#[cfg(feature = "unproven")]
use crate::encode::Encode;
use crate::error::*;
#[cfg(feature = "unproven")]
use crate::new_element;
use crate::types::Parse;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Access {
    ReadOnly,
    ReadWrite,
    ReadWriteOnce,
    WriteOnce,
    WriteOnly,
}

impl Parse for Access {
    type Object = Access;
    type Error = SVDError;

    fn parse(tree: &Element) -> Result<Access, SVDError> {
        let text = tree.get_text()?;

        match &text[..] {
            "read-only" => Ok(Access::ReadOnly),
            "read-write" => Ok(Access::ReadWrite),
            "read-writeOnce" => Ok(Access::ReadWriteOnce),
            "write-only" => Ok(Access::WriteOnly),
            "writeOnce" => Ok(Access::WriteOnce),
            _ => Err(SVDErrorKind::UnknownAccessType(tree.clone(), text).into()),
        }
    }
}

#[cfg(feature = "unproven")]
impl Encode for Access {
    type Error = SVDError;

    fn encode(&self) -> Result<Element, SVDError> {
        let text = match *self {
            Access::ReadOnly => String::from("read-only"),
            Access::ReadWrite => String::from("read-write"),
            Access::ReadWriteOnce => String::from("read-writeOnce"),
            Access::WriteOnly => String::from("write-only"),
            Access::WriteOnce => String::from("writeOnce"),
        };

        Ok(new_element("access", Some(text)))
    }
}

#[cfg(test)]
#[cfg(feature = "unproven")]
mod tests {
    use super::*;
    use crate::run_test;

    #[test]
    fn decode_encode() {
        let tests = vec![
            (Access::ReadOnly, "<access>read-only</access>"),
            (Access::ReadWrite, "<access>read-write</access>"),
            (Access::ReadWriteOnce, "<access>read-writeOnce</access>"),
            (Access::WriteOnly, "<access>write-only</access>"),
            (Access::WriteOnce, "<access>writeOnce</access>"),
        ];

        run_test::<Access>(&tests[..]);
    }
}
