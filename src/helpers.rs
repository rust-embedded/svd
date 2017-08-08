extern crate xmltree;

use xmltree::Element;

pub trait Parse {
    fn parse(tree: &Element) -> Self;
}

pub trait Encode {
    fn encode(&self) -> Element;
}

