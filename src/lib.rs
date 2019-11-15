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

#![deny(warnings)]

#[cfg(feature = "unproven")]
use std::collections::HashMap;

use xmltree::Element;

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
#[cfg(feature = "unproven")]
pub mod encode;
#[cfg(feature = "unproven")]
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
    let tree = Element::parse(xml.as_bytes())?;
    Device::parse(&tree)
}

/// Encodes a device object to an SVD (XML) string
#[cfg(feature = "unproven")]
pub fn encode(d: &Device) -> Result<String> {
    let root = d.encode()?;
    let mut wr = Vec::new();
    root.write(&mut wr).unwrap();
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
#[cfg(feature = "unproven")]
pub(crate) fn new_element(name: &str, text: Option<String>) -> Element {
    Element {
        prefix: None,
        namespace: None,
        namespaces: None,
        name: String::from(name),
        attributes: HashMap::new(),
        children: Vec::new(),
        text,
    }
}

pub trait Build {
    type Builder;
}

/// Generic test helper function
/// Takes an array of (item, xml) pairs where the item implements
/// Parse and Encode and tests object encoding and decoding
#[cfg(test)]
#[cfg(feature = "unproven")]
pub fn run_test<
    T: Parse<Error = anyhow::Error, Object = T>
        + Encode<Error = anyhow::Error>
        + core::fmt::Debug
        + PartialEq,
>(
    tests: &[(T, &str)],
) {
    for t in tests {
        let mut tree1 = Element::parse(t.1.as_bytes()).unwrap();
        let elem = T::parse(&tree1).unwrap();
        // Hack to make assert be order agnostic
        tree1.children.sort_by(|e1, e2| e1.name.cmp(&e2.name));
        tree1.children.iter_mut().for_each(|e| {
            e.children.sort_by(|e1, e2| e1.name.cmp(&e2.name));
        });
        assert_eq!(
            elem, t.0,
            "Error parsing xml` (mismatch between parsed and expected)"
        );
        let mut tree2 = elem.encode().unwrap();
        // Hack to make assert be order agnostic
        tree2.children.sort_by(|e1, e2| e1.name.cmp(&e2.name));
        tree2.children.iter_mut().for_each(|e| {
            e.children.sort_by(|e1, e2| e1.name.cmp(&e2.name));
        });
        assert_eq!(
            tree1, tree2,
            "Error encoding xml (mismatch between encoded and original)"
        );
    }
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
