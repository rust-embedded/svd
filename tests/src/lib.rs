#![cfg(test)]

use core::str;
use roxmltree::Document;
use svd_encoder::{Encode, EncodeError};
use svd_parser::{Config, Parse, SVDErrorAt};
use svd_rs as svd;
use xmltree::Element;

mod bad_svd;

/// Generic test helper function
/// Takes an array of (item, xml) pairs where the item implements
/// Parse and Encode and tests object encoding and decoding
pub fn run_test<
    T: Parse<Error = SVDErrorAt, Object = T, Config = Config>
        + Encode<Error = EncodeError>
        + core::fmt::Debug
        + PartialEq,
>(
    tests: &[(T, &str)],
) {
    for t in tests {
        let rotree = Document::parse(t.1).unwrap();
        let elem = T::parse(
            &rotree.root().first_element_child().unwrap(),
            &Config::default(),
        )
        .unwrap();
        assert_eq!(
            elem, t.0,
            "Error parsing xml` (mismatch between parsed and expected)"
        );

        let tree1 = Element::parse(t.1.as_bytes()).unwrap();
        let tree2 = elem.encode().unwrap();
        assert_eq!(
            tree1, tree2,
            "Error encoding xml (mismatch between encoded and original)"
        );
    }
}

mod access;
mod addressblock;
//mod bitrange;
mod cpu;
mod dimelement;
mod endian;
mod enumeratedvalue;
//mod enumeratedvalues;
mod field;
mod interrupt;
mod modifiedwritevalues;
mod register;
//mod registerproperties;
mod usage;
mod writeconstraint;
