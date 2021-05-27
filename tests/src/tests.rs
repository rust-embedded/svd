use crate::encode::{Encode, EncodeError};
use crate::parse::Parse;
use core::str;
use xmltree::Element;

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

mod access;
mod addressblock;
mod bitrange;
mod cpu;
mod dimelement;
mod endian;
mod enumeratedvalue;
mod enumeratedvalues;
mod field;
mod fieldinfo;
mod interrupt;
mod modifiedwritevalues;
mod register;
mod registerinfo;
mod registerproperties;
mod usage;
mod writeconstraint;
