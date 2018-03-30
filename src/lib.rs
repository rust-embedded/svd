//! CMSIS-SVD file parser
//!
//! # Usage
//!
//! ``` no_run
//! extern crate svd_parser as svd;
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

extern crate either;
extern crate xmltree;
#[macro_use]
extern crate failure;
use failure::ResultExt;

use xmltree::Element;

pub mod svd;
use svd::device::Device;
pub mod error;
use error::{SVDError, SVDErrorKind};

pub mod parse;
pub mod types;
use types::Parse;


/// Parses the contents of a SVD file (XML)
pub fn parse(xml: &str) -> Result<Device, SVDError> {
    let tree = Element::parse(xml.as_bytes())?;
    Device::parse(&tree)
}

trait ElementExt {
    fn get_child_text_opt<K>(&self, k: K) -> Result<Option<String>, SVDError>
    where
        String: PartialEq<K>;
    fn get_child_text<K>(&self, k: K) -> Result<String, SVDError>
    where
        String: PartialEq<K>,
        K: ::std::fmt::Display + Clone;

    fn get_text(&self) -> Result<String, SVDError>;

    fn get_child_elem<'a>(&'a self, n: &str) -> Result<&'a Element, SVDError>;
    fn get_child_u32(&self, n: &str) -> Result<u32, SVDError>;
    fn get_child_bool(&self, n: &str) -> Result<bool, SVDError>;

    fn debug(&self);
}

impl ElementExt for Element {
    fn get_child_text_opt<K>(&self, k: K) -> Result<Option<String>, SVDError>
    where
        String: PartialEq<K>,
    {
        if let Some(child) = self.get_child(k) {
            Ok(Some(child.get_text().map(|s| s.to_owned())?))
        } else {
            Ok(None)
        }
    }
    fn get_child_text<K>(&self, k: K) -> Result<String, SVDError>
    where
        String: PartialEq<K>,
        K: ::std::fmt::Display + Clone,
    {
        self.get_child_text_opt(k.clone())?.ok_or(SVDErrorKind::MissingTag(self.clone(), format!("{}", k)).into())
    }

    /// Get text contained by an XML Element
    fn get_text(&self) -> Result<String, SVDError> {
        match self.text.as_ref() {
            Some(s) => Ok(s.clone()),
            // FIXME: Doesn't look good because SVDErrorKind doesn't format by itself. We already
            // capture the element and this information can be used for getting the name
            // This would fix ParseError
            None => Err(SVDErrorKind::EmptyTag(self.clone(), self.name.clone()).into()),
        }
    }

    /// Get a named child element from an XML Element
    fn get_child_elem<'a>(&'a self, n: &str) -> Result<&'a Element, SVDError> {
        match self.get_child(n) {
            Some(s) => Ok(s),
            None => Err(SVDErrorKind::MissingTag(self.clone(), n.to_string()).into()),
        }
    }

    /// Get a u32 value from a named child element
    fn get_child_u32(&self, n: &str) -> Result<u32, SVDError> {
        let s = self.get_child_elem(n)?;
        parse::u32(&s).context(SVDErrorKind::ParseError(self.clone())).map_err(|e| e.into())
    }

    /// Get a bool value from a named child element
    fn get_child_bool(&self, n: &str) -> Result<bool, SVDError> {
        let s = self.get_child_elem(n)?;
        match parse::bool(s) {
            Some(u) => Ok(u),
            None => Err(SVDErrorKind::ParseError(self.clone()).into())
        }
    }

    fn debug(&self) {
        println!("<{}>", self.name);
        for c in &self.children {
            println!("{}: {:?}", c.name, c.text)
        }
        println!("</{}>", self.name);
    }
}







