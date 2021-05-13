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
//! let xml = &mut String::new();
//! File::open("STM32F30x.svd").unwrap().read_to_string(xml);
//!
//! println!("{:?}", svd::parse(xml));
//! ```
//!
//! # References
//!
//! - [SVD Schema file](https://www.keil.com/pack/doc/CMSIS/SVD/html/schema_1_2_gr.html)
//! - [SVD file database](https://github.com/posborne/cmsis-svd/tree/master/data)
//! - [Sample SVD file](https://www.keil.com/pack/doc/CMSIS/SVD/html/svd_Example_pg.html)

#![deny(warnings)]

// SVD contains svd primitives
pub mod svd;
pub use svd::*;
// Parse defines parsing interfaces
pub mod parse;
pub use parse::parse;
// Encode defines encoding interfaces
pub mod encode;
pub use encode::encode;

#[cfg(feature = "derive-from")]
pub mod derive_from;
#[cfg(feature = "derive-from")]
pub use derive_from::DeriveFrom;

#[cfg(test)]
mod tests;
