use xmltree::Element;

use error::*;
use types::Parse;
use ElementExt;

macro_rules! try {
    ($e:expr) => {
        $e.expect(concat!(file!(), ":", line!(), " ", stringify!($e)))
    }
}

pub struct BoolParse;

impl Parse for BoolParse {
    type Object = Option<bool>;
    type Error = SVDError;
    fn parse(tree: &Element) -> Result<Option<bool>, SVDError> {
        let text = try!(tree.text.as_ref());
        Ok(match text.as_ref() {
            "0" => Some(false),
            "1" => Some(true),
            _ => text.parse::<bool>().ok()
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
        } else {
            unreachable!()
        }
     }
 }
/// Parses an optional child element with the provided name and Parse function
/// Returns an none if the child doesn't exist, Ok(Some(e)) if parsing succeeds,
/// and Err() if parsing fails.
/// TODO: suspect we should be able to use the Parse trait here
pub fn optional<'a, T>(n: &str, e: &'a Element) -> Result<Option<T::Object>, SVDError>
    where T: Parse<Error = SVDError>
{
     let child = match e.get_child(n) {
        Some(c) => c,
        None => return Ok(None),
    };

    match T::parse(child) {
        Ok(r) => Ok(Some(r)),
        Err(e) => Err(e),
    }
}

