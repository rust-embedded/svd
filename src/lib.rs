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
    fn debug(&self);
}

impl ElementExt for Element {
    fn get_child_text_opt<K>(&self, k: K) -> Result<Option<String>, SVDError>
    where
        String: PartialEq<K>,
    {
        if let Some(child) = self.get_child(k) {
            Ok(Some(parse::get_text(child).map(|s| s.to_owned())?))
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

    fn debug(&self) {
        println!("<{}>", self.name);
        for c in &self.children {
            println!("{}: {:?}", c.name, c.text)
        }
        println!("</{}>", self.name);
    }
}







