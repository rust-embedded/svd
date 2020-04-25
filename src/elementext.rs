//! SVD Element Extensions.
//! This module is extends xmltree::Element objects with convenience methods

use xmltree::Element;

use crate::types::{BoolParse, Parse};

use crate::error::*;

/// Defines extensions for implementation over xmltree::Element
pub trait ElementExt {
    fn get_child_text_opt<K>(&self, k: K) -> Result<Option<String>>
    where
        String: PartialEq<K>;
    fn get_child_text<K>(&self, k: K) -> Result<String>
    where
        String: PartialEq<K>,
        K: core::fmt::Display + Clone;

    fn get_text(&self) -> Result<String>;

    fn get_child_elem<'a>(&'a self, n: &str) -> Result<&'a Element>;
    fn get_child_u32(&self, n: &str) -> Result<u32>;
    fn get_child_bool(&self, n: &str) -> Result<bool>;

    fn merge(&self, n: &Self) -> Self;

    fn debug(&self);
}

/// Implements extensions for xmltree::Element
impl ElementExt for Element {
    fn get_child_text_opt<K>(&self, k: K) -> Result<Option<String>>
    where
        String: PartialEq<K>,
    {
        if let Some(child) = self.get_child(k) {
            match child.get_text() {
                Err(e) => match e.downcast_ref() {
                    // if tag is empty just ignore it
                    Some(ParseError::EmptyTag(_, _)) => Ok(None),
                    _ => return Err(e),
                },
                Ok(s) => Ok(Some(s.to_owned())),
            }
        } else {
            Ok(None)
        }
    }
    fn get_child_text<K>(&self, k: K) -> Result<String>
    where
        String: PartialEq<K>,
        K: core::fmt::Display + Clone,
    {
        self.get_child_text_opt(k.clone())?
            .ok_or_else(|| ParseError::MissingTag(self.clone(), format!("{}", k)).into())
    }

    /// Get text contained by an XML Element
    fn get_text(&self) -> Result<String> {
        match self.text.as_ref() {
            Some(s) => Ok(s.clone()),
            // FIXME: Doesn't look good because SVDError doesn't format by itself. We already
            // capture the element and this information can be used for getting the name
            // This would fix ParseError
            None => Err(ParseError::EmptyTag(self.clone(), self.name.clone()).into()),
        }
    }

    /// Get a named child element from an XML Element
    fn get_child_elem<'a>(&'a self, n: &str) -> Result<&'a Element> {
        match self.get_child(n) {
            Some(s) => Ok(s),
            None => Err(ParseError::MissingTag(self.clone(), n.to_string()).into()),
        }
    }

    /// Get a u32 value from a named child element
    fn get_child_u32(&self, n: &str) -> Result<u32> {
        let s = self.get_child_elem(n)?;
        u32::parse(&s).context(ParseError::Other(self.clone()))
    }

    /// Get a bool value from a named child element
    fn get_child_bool(&self, n: &str) -> Result<bool> {
        let s = self.get_child_elem(n)?;
        BoolParse::parse(s)
    }

    // Merges the children of two elements, maintaining the name and description of the first
    fn merge(&self, r: &Self) -> Self {
        let mut n = self.clone();
        for c in &r.children {
            n.children.push(c.clone());
        }
        n
    }

    fn debug(&self) {
        println!("<{}>", self.name);
        for c in &self.children {
            println!("{}: {:?}", c.name, c.text)
        }
        println!("</{}>", self.name);
    }
}
