//! Shared primitive types for use in SVD objects.

use failure::ResultExt;
use xmltree::Element;

#[cfg(feature = "unproven")]
pub use encode::Encode;
pub use parse::optional as parse_optional;
pub use parse::Parse;

use elementext::ElementExt;
use error::{SVDError, SVDErrorKind};

macro_rules! try {
    ($e:expr) => {
        $e.expect(concat!(
            file!(),
            ":",
            line!(),
            " ",
            stringify!($e)
        ))
    };
}

impl Parse for u32 {
    type Object = u32;
    type Error = SVDError;

    fn parse(tree: &Element) -> Result<u32, SVDError> {
        let text = tree.get_text()?;

        if text.starts_with("0x") || text.starts_with("0X") {
            u32::from_str_radix(&text["0x".len()..], 16)
                .context(SVDErrorKind::Other(format!("{} invalid", text)))
                .map_err(|e| e.into())
        } else if text.starts_with('#') {
            // Handle strings in the binary form of:
            // #01101x1
            // along with don't care character x (replaced with 0)
            u32::from_str_radix(
                &str::replace(&text.to_lowercase()["#".len()..], "x", "0"),
                2,
            ).context(SVDErrorKind::Other(format!("{} invalid", text)))
                .map_err(|e| e.into())
        } else if text.starts_with("0b") {
            // Handle strings in the binary form of:
            // 0b01101x1
            // along with don't care character x (replaced with 0)
            u32::from_str_radix(&str::replace(&text["0b".len()..], "x", "0"), 2)
                .context(SVDErrorKind::Other(format!("{} invalid", text)))
                .map_err(|e| e.into())
        } else {
            text.parse::<u32>()
                .context(SVDErrorKind::Other(format!("{} invalid", text)))
                .map_err(|e| e.into())
        }
    }
}

pub struct BoolParse;

impl Parse for BoolParse {
    type Object = bool;
    type Error = SVDError;
    fn parse(tree: &Element) -> Result<bool, SVDError> {
        let text = try!(tree.text.as_ref());
        Ok(match text.as_ref() {
            "0" => false,
            "1" => true,
            _ => match text.parse() {
                Ok(b) => b,
                Err(e) => {
                    return Err(SVDErrorKind::InvalidBooleanValue(
                        tree.clone(),
                        text.clone(),
                        e,
                    ).into())
                }
            },
        })
    }
}

pub struct DimIndex;

impl Parse for DimIndex {
    type Object = Vec<String>;
    type Error = SVDError;

    fn parse(tree: &Element) -> Result<Vec<String>, SVDError> {
        let text = tree.get_text()?;
        if text.contains('-') {
            let mut parts = text.splitn(2, '-');
            let start = try!(try!(parts.next()).parse::<u32>());
            let end = try!(try!(parts.next()).parse::<u32>()) + 1;

            Ok((start..end).map(|i| i.to_string()).collect())
        } else if text.contains(',') {
            Ok(text.split(',').map(|s| s.to_string()).collect())
        } else if text.parse::<u32>().is_ok() {
            Ok(vec![text])
        } else {
            unreachable!()
        }
    }
}

//TODO: encode for DimIndex
