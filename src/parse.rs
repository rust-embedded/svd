//! Parse traits.
//! These support parsing of SVD types from XML

use xmltree::Element;

use error::*;

/// Parse trait allows SVD objects to be parsed from XML elements.
pub trait Parse {
    /// Object returned by parse method
    type Object;
    /// Parsing error
    type Error;
    /// Parse an XML/SVD element into it's corresponding `Object`.
    fn parse(&Element) -> Result<Self::Object, Self::Error>;
}


/// Parses an optional child element with the provided name and Parse function
/// Returns an none if the child doesn't exist, Ok(Some(e)) if parsing succeeds,
/// and Err() if parsing fails.
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

