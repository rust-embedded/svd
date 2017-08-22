extern crate xmltree;

use xmltree::Element;
use ElementExt;

use helpers::*;
use access::*;
use writeconstraint::*;
use bitrange::*;
use enumeratedvalues::*;

macro_rules! try {
    ($e:expr) => {
        $e.expect(concat!(file!(), ":", line!(), " ", stringify!($e)))
    }
}


#[derive(Clone, Debug)]
pub struct Field {
    pub name: String,
    pub description: Option<String>,
    pub bit_range: BitRange,
    pub access: Option<Access>,
    pub enumerated_values: Vec<EnumeratedValues>,
    pub write_constraint: Option<WriteConstraint>,
    // Reserve the right to add more fields to this struct
    _extensible: (),
}

impl ParseElem for Field {
    fn parse(tree: &Element) -> Field {
        assert_eq!(tree.name, "field");

        Field {
            name: try!(tree.get_child_text("name")),
            description: tree.get_child_text("description"),
            bit_range: BitRange::parse(tree),
            access: tree.get_child("access").map(Access::parse),
            enumerated_values: tree.children
                .iter()
                .filter(|t| t.name == "enumeratedValues")
                .map(EnumeratedValues::parse)
                .collect::<Vec<_>>(),
            write_constraint: tree.get_child("writeConstraint")
                .map(WriteConstraint::parse),
            _extensible: (),
        }
    }
}



