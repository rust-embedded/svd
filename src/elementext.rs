//! SVD Element Extensions.
//! This module is extends xmltree::Element objects with convenience methods

use xmltree::Element;

use crate::types::{BoolParse, Parse};
use failure::ResultExt;

use crate::error::*;

/// Defines extensions for implementation over xmltree::Element
pub trait ElementExt {
    fn get_child_text_opt<K>(&self, k: K) -> Result<Option<String>, SVDError>
    where
        String: PartialEq<K>;
    fn get_child_text<K>(&self, k: K) -> Result<String, SVDError>
    where
        String: PartialEq<K>,
        K: core::fmt::Display + Clone;

    fn get_text(&self) -> Result<String, SVDError>;

    fn get_child_elem<'a>(&'a self, n: &str) -> Result<&'a Element, SVDError>;
    fn get_child_u32(&self, n: &str) -> Result<u32, SVDError>;
    fn get_child_bool(&self, n: &str) -> Result<bool, SVDError>;

    fn merge(&self, n: &Self) -> Self;

    fn debug(&self);
}

/// Implements extensions for xmltree::Element
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
        K: core::fmt::Display + Clone,
    {
        self.get_child_text_opt(k.clone())?
            .ok_or(SVDErrorKind::MissingTag(self.clone(), format!("{}", k)).into())
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
        u32::parse(&s)
            .context(SVDErrorKind::ParseError(self.clone()))
            .map_err(|e| e.into())
    }

    /// Get a bool value from a named child element
    fn get_child_bool(&self, n: &str) -> Result<bool, SVDError> {
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
