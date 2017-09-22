#[macro_use]
extern crate error_chain;
extern crate xmltree;

use xmltree::Element;

pub use enumerated_value::EnumeratedValue;

use error::*;

pub mod error;
mod enumerated_value;
mod parse;

pub trait Parse {
    fn parse(element: &Element) -> Result<Self>
    where
        Self: Sized;
}

trait ElementExt {
    fn text(&self) -> Result<String>;
}

impl ElementExt for Element {
    fn text(&self) -> Result<String> {
        ensure!(
            self.children.is_empty() && self.prefix.is_none() &&
                self.namespace.is_none() &&
                self.namespaces.is_none() &&
                self.attributes.is_empty(),
            "expected an element with only text"
        );

        Ok(self.text.clone().ok_or("no inner text")?)
    }
}
