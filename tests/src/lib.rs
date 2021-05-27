#![cfg(test)]

use core::str;
use roxmltree::Document;
use std::collections::HashMap;
use svd_encoder::{self as encode, Encode, EncodeChildren, EncodeError};
use svd_parser::{self as parse, Parse};
use svd_rs as svd;
use xmltree::Element;

mod bad_svd;

/// Generic test helper function
/// Takes an array of (item, xml) pairs where the item implements
/// Parse and Encode and tests object encoding and decoding
pub fn run_test<
    T: Parse<Error = anyhow::Error, Object = T>
        + Encode<Error = EncodeError>
        + core::fmt::Debug
        + PartialEq,
>(
    tests: &[(T, &str)],
) {
    for t in tests {
        let rotree = Document::parse(t.1).unwrap();
        let elem = T::parse(&rotree.root().first_element_child().unwrap()).unwrap();
        assert_eq!(
            elem, t.0,
            "Error parsing xml` (mismatch between parsed and expected)"
        );

        let mut tree1 = Element::parse(t.1.as_bytes()).unwrap();
        // Hack to make assert be order agnostic
        tree1.children.sort_by(|e1, e2| e1.name.cmp(&e2.name));
        tree1.children.iter_mut().for_each(|e| {
            e.children.sort_by(|e1, e2| e1.name.cmp(&e2.name));
        });
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

/// Helper to create new base xml elements
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

mod access;
mod addressblock;
//mod bitrange;
mod cpu;
mod dimelement;
mod endian;
mod enumeratedvalue;
//mod enumeratedvalues;
mod field;
mod fieldinfo;
mod interrupt;
mod modifiedwritevalues;
mod register;
mod registerinfo;
//mod registerproperties;
mod usage;
mod writeconstraint;
