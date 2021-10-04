//! SVD Element Extensions.
//! This module is extends roxmltree::Element objects with convenience methods

use roxmltree::Node;

use super::types::BoolParse;
use super::{Parse, SVDError, SVDErrorAt};

/// Defines extensions for implementation over roxmltree::Node
pub trait ElementExt {
    fn get_child<K>(&self, k: K) -> Option<Node>
    where
        K: AsRef<str>;
    fn get_child_text_opt<K>(&self, k: K) -> Result<Option<String>, SVDErrorAt>
    where
        K: AsRef<str>;
    fn get_child_text<K>(&self, k: K) -> Result<String, SVDErrorAt>
    where
        K: AsRef<str>;

    fn get_text(&self) -> Result<&str, SVDErrorAt>;

    fn get_child_elem(&self, n: &str) -> Result<Node, SVDErrorAt>;
    fn get_child_u32(&self, n: &str) -> Result<u32, SVDErrorAt>;
    fn get_child_u64(&self, n: &str) -> Result<u64, SVDErrorAt>;
    fn get_child_bool(&self, n: &str) -> Result<bool, SVDErrorAt>;

    fn debug(&self);
}

/// Implements extensions for roxmltree::Node
impl<'a, 'input> ElementExt for Node<'a, 'input> {
    fn get_child<K>(&self, k: K) -> Option<Node>
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
    fn get_child_text_opt<K>(&self, k: K) -> Result<Option<String>, SVDErrorAt>
    where
        K: AsRef<str>,
    {
        if let Some(child) = self.get_child(k) {
            match child.get_text() {
                Err(e) => {
                    // if tag is empty just ignore it
                    match e {
                        SVDErrorAt {
                            error: SVDError::EmptyTag(_),
                            ..
                        } => Ok(None),
                        _ => Err(e),
                    }
                }
                Ok(s) => Ok(Some(s.to_string())),
            }
        } else {
            Ok(None)
        }
    }
    fn get_child_text<K>(&self, k: K) -> Result<String, SVDErrorAt>
    where
        K: AsRef<str>,
    {
        let k = k.as_ref();
        self.get_child_text_opt(k)?
            .ok_or_else(|| SVDError::MissingTag(k.to_string()).at(self.id()))
    }

    /// Get text contained by an XML Element
    fn get_text(&self) -> Result<&str, SVDErrorAt> {
        match self.text() {
            Some(s) => Ok(s),
            // FIXME: Doesn't look good because SVDError doesn't format by itself. We already
            // capture the element and this information can be used for getting the name
            // This would fix ParseError
            None => Err(SVDError::EmptyTag(self.tag_name().name().to_string()).at(self.id())),
        }
    }

    /// Get a named child element from an XML Element
    fn get_child_elem(&self, n: &str) -> Result<Node, SVDErrorAt> {
        self.get_child(n)
            .ok_or_else(|| SVDError::MissingTag(n.to_string()).at(self.id()))
    }

    /// Get a u32 value from a named child element
    fn get_child_u32(&self, n: &str) -> Result<u32, SVDErrorAt> {
        let s = self.get_child_elem(n)?;
        u32::parse(&s, &())
    }

    /// Get a u64 value from a named child element
    fn get_child_u64(&self, n: &str) -> Result<u64, SVDErrorAt> {
        let s = self.get_child_elem(n)?;
        u64::parse(&s, &())
    }

    /// Get a bool value from a named child element
    fn get_child_bool(&self, n: &str) -> Result<bool, SVDErrorAt> {
        let s = self.get_child_elem(n)?;
        BoolParse::parse(&s, &())
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
