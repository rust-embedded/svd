//! Parse traits.
//! These support parsing of SVD types from XML

use xmltree::Element;

/// Parse trait allows SVD objects to be parsed from XML elements.
pub trait Parse {
    /// Object returned by parse method
    type Object;
    /// Parsing error
    type Error;
    /// Parse an XML/SVD element into it's corresponding `Object`.
    fn parse(elem: &Element) -> Result<Self::Object, Self::Error>;
}

/// Parses an optional child element with the provided name and Parse function
/// Returns an none if the child doesn't exist, Ok(Some(e)) if parsing succeeds,
/// and Err() if parsing fails.
pub fn optional<T>(n: &str, e: &Element) -> anyhow::Result<Option<T::Object>>
where
    T: Parse<Error = anyhow::Error>,
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

use crate::svd::Device;
/// Parses the contents of an SVD (XML) string
pub fn parse(xml: &str) -> anyhow::Result<Device> {
    let xml = trim_utf8_bom(xml);
    let tree = Element::parse(xml.as_bytes())?;
    Device::parse(&tree)
}

/// Return the &str trimmed UTF-8 BOM if the input &str contains the BOM.
fn trim_utf8_bom(s: &str) -> &str {
    if s.len() > 2 && s.as_bytes().starts_with(b"\xef\xbb\xbf") {
        &s[3..]
    } else {
        s
    }
}

mod access;
mod addressblock;
mod bitrange;
mod cluster;
mod clusterinfo;
mod cpu;
mod device;
mod dimelement;
mod endian;
mod enumeratedvalue;
mod enumeratedvalues;
mod field;
mod fieldinfo;
mod interrupt;
mod modifiedwritevalues;
mod peripheral;
mod register;
mod registercluster;
mod registerinfo;
mod registerproperties;
mod usage;
mod writeconstraint;

#[test]
fn test_trim_utf8_bom_from_str() {
    // UTF-8 BOM + "xyz"
    let bom_str = std::str::from_utf8(b"\xef\xbb\xbfxyz").unwrap();
    assert_eq!("xyz", trim_utf8_bom(bom_str));
    assert_eq!("xyz", trim_utf8_bom("xyz"));
}
