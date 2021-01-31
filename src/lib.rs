//! CMSIS-SVD file parser
//!
//! # Usage
//!
//! ``` no_run
//! use svd_parser as svd;
//!
//! use std::fs::File;
//! use std::io::Read;
//!
//! fn main() {
//!     let xml = &mut String::new();
//!     File::open("STM32F30x.svd").unwrap().read_to_string(xml);
//!
//!     println!("{:?}", svd::parse(xml));
//! }
//! ```
//!
//! # References
//!
//! - [SVD Schema file](https://www.keil.com/pack/doc/CMSIS/SVD/html/schema_1_2_gr.html)
//! - [SVD file database](https://github.com/posborne/cmsis-svd/tree/master/data)
//! - [Sample SVD file](https://www.keil.com/pack/doc/CMSIS/SVD/html/svd_Example_pg.html)

//#![deny(warnings)]

use minidom::{Element, ElementBuilder};

// ElementExt extends XML elements with useful methods
pub mod elementext;
// SVD contains svd primitives
pub mod svd;
pub use svd::*;
// Error defines SVD error types
pub mod error;
use anyhow::Result;
// Parse defines parsing interfaces
pub mod parse;
use parse::Parse;
// Encode defines encoding interfaces
pub mod encode;
use encode::Encode;
// Types defines simple types and parse/encode implementations
pub mod types;

#[cfg(feature = "derive-from")]
pub mod derive_from;
#[cfg(feature = "derive-from")]
pub use derive_from::DeriveFrom;

/// Parses the contents of an SVD (XML) string
pub fn parse(xml: &str) -> Result<Device> {
    let xml = trim_utf8_bom(xml);
    let tree: Element = xml.parse()?;
    Device::parse(&tree)
}

/// Encodes a device object to an SVD (XML) string
pub fn encode(d: &Device) -> Result<String> {
    let root = d.encode()?;
    let mut wr = Vec::new();
    root.write_to(&mut wr).unwrap();
    Ok(String::from_utf8(wr).unwrap())
}

/// Return the &str trimmed UTF-8 BOM if the input &str contains the BOM.
fn trim_utf8_bom(s: &str) -> &str {
    if s.len() > 2 && s.as_bytes().starts_with(b"\xef\xbb\xbf") {
        &s[3..]
    } else {
        s
    }
}

/// Helper to create new base xml elements
pub(crate) fn new_element(name: &str, text: Option<String>) -> ElementBuilder {
    if let Some(text) = text {
        Element::builder(name, "").append(text)
    } else {
        Element::builder(name, "")
    }
}

/// Generic test helper function
/// Takes an array of (item, xml) pairs where the item implements
/// Parse and Encode and tests object encoding and decoding
#[cfg(test)]
pub fn run_test<
    T: Parse<Error = anyhow::Error, Object = T>
        + Encode<Error = anyhow::Error>
        + core::fmt::Debug
        + PartialEq,
>(
    tests: &[(T, &str)],
) {
    /*for t in tests {
        let mut tree1: Element = t.1.parse().unwrap();
        let elem = T::parse(&tree1).unwrap();
        // Hack to make assert be order agnostic
        let mut children1 = tree1.children().collect::<Vec<_>>();
        children1.sort_by(|e1, e2| e1.name().cmp(&e2.name()));
        children1.iter_mut().for_each(|e| {
            e.children.sort_by(|e1, e2| e1.name().cmp(&e2.name()));
        });
        assert_eq!(
            elem, t.0,
            "Error parsing xml` (mismatch between parsed and expected)"
        );
        let mut tree2 = elem.encode().unwrap();
        // Hack to make assert be order agnostic
        let mut children2 = tree2.children().collect::<Vec<_>>();
        children2.sort_by(|e1, e2| e1.name().cmp(&e2.name()));
        children2.iter_mut().for_each(|e| {
            e.children.sort_by(|e1, e2| e1.name().cmp(&e2.name()));
        });
        assert_eq!(
            children1, children1,
            "Error encoding xml (mismatch between encoded and original)"
        );
    }*/
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::str;

    #[test]
    fn test_trim_utf8_bom_from_str() {
        // UTF-8 BOM + "xyz"
        let bom_str = str::from_utf8(b"\xef\xbb\xbfxyz").unwrap();
        assert_eq!("xyz", trim_utf8_bom(bom_str));
        assert_eq!("xyz", trim_utf8_bom("xyz"));
    }
}
