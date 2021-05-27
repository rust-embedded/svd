//! SVD Element Extensions.
//! This module is extends xmltree::Element objects with convenience methods

use roxmltree::Node as Element;

use super::types::BoolParse;
use super::{Context, Parse, Result, SVDError};

/// Defines extensions for implementation over xmltree::Element
pub trait ElementExt {
    fn get_child<K>(&self, k: K) -> Option<Element>
    where
        K: AsRef<str>;
    fn get_child_text_opt<K>(&self, k: K) -> Result<Option<String>>
    where
        K: AsRef<str>;
    fn get_child_text<K>(&self, k: K) -> Result<String>
    where
        K: AsRef<str>;

    fn get_text(&self) -> Result<String>;

    fn get_child_elem(&self, n: &str) -> Result<Element>;
    fn get_child_u32(&self, n: &str) -> Result<u32>;
    fn get_child_u64(&self, n: &str) -> Result<u64>;
    fn get_child_bool(&self, n: &str) -> Result<bool>;

    fn debug(&self);
}

/// Implements extensions for xmltree::Element
impl<'a, 'input> ElementExt for Element<'a, 'input> {
    fn get_child<K>(&self, k: K) -> Option<Element>
    where
        K: AsRef<str>,
    {
        for c in self.children() {
            if c.has_tag_name(k.as_ref()) {
                return Some(c);
            }
        }
        None
    }
    fn get_child_text_opt<K>(&self, k: K) -> Result<Option<String>>
    where
        K: AsRef<str>,
    {
        if let Some(child) = self.get_child(k) {
            match child.get_text() {
                Err(e) => match e.downcast_ref() {
                    // if tag is empty just ignore it
                    Some(SVDError::EmptyTag(_, _)) => Ok(None),
                    _ => Err(e),
                },
                Ok(s) => Ok(Some(s)),
            }
        } else {
            Ok(None)
        }
    }
    fn get_child_text<K>(&self, k: K) -> Result<String>
    where
        K: AsRef<str>,
    {
        let k = k.as_ref();
        self.get_child_text_opt(k)?
            .ok_or_else(|| SVDError::MissingTag(self.id(), format!("{}", k)).into())
    }

    /// Get text contained by an XML Element
    fn get_text(&self) -> Result<String> {
        match self.text().as_ref() {
            Some(s) => Ok(s.to_string()),
            // FIXME: Doesn't look good because SVDError doesn't format by itself. We already
            // capture the element and this information can be used for getting the name
            // This would fix ParseError
            None => Err(SVDError::EmptyTag(self.id(), self.tag_name().name().to_string()).into()),
        }
    }

    /// Get a named child element from an XML Element
    fn get_child_elem(&self, n: &str) -> Result<Element> {
        match self.get_child(n) {
            Some(s) => Ok(s),
            None => Err(SVDError::MissingTag(self.id(), n.to_string()).into()),
        }
    }

    /// Get a u32 value from a named child element
    fn get_child_u32(&self, n: &str) -> Result<u32> {
        let s = self.get_child_elem(n)?;
        u32::parse(&s).context(SVDError::ParseError(self.id()))
    }

    /// Get a u64 value from a named child element
    fn get_child_u64(&self, n: &str) -> Result<u64> {
        let s = self.get_child_elem(n)?;
        u64::parse(&s).context(SVDError::ParseError(self.id()))
    }

    /// Get a bool value from a named child element
    fn get_child_bool(&self, n: &str) -> Result<bool> {
        let s = self.get_child_elem(n)?;
        BoolParse::parse(&s)
    }

    fn debug(&self) {
        let name = self.tag_name().name();
        println!("<{}>", name);
        for c in self.children() {
            println!("{}: {:?}", c.tag_name().name(), c.text())
        }
        println!("</{}>", name);
    }
}
