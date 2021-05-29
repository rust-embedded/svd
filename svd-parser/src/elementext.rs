//! SVD Element Extensions.
//! This module is extends roxmltree::Element objects with convenience methods

use roxmltree::Node;

use super::types::BoolParse;
use super::{Context, Parse, Result, SVDError, SVDErrorAt};

/// Defines extensions for implementation over roxmltree::Node
pub trait ElementExt {
    fn get_child<K>(&self, k: K) -> Option<Node>
    where
        K: AsRef<str>;
    fn get_child_text_opt<K>(&self, k: K) -> Result<Option<String>>
    where
        K: AsRef<str>;
    fn get_child_text<K>(&self, k: K) -> Result<String>
    where
        K: AsRef<str>;

    fn get_text(&self) -> Result<&str>;

    fn get_child_elem(&self, n: &str) -> Result<Node>;
    fn get_child_u32(&self, n: &str) -> Result<u32>;
    fn get_child_u64(&self, n: &str) -> Result<u64>;
    fn get_child_bool(&self, n: &str) -> Result<bool>;

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
    fn get_child_text_opt<K>(&self, k: K) -> Result<Option<String>>
    where
        K: AsRef<str>,
    {
        if let Some(child) = self.get_child(k) {
            match child.get_text() {
                Err(e) => {
                    // if tag is empty just ignore it
                    if let Some(SVDError::EmptyTag(_)) = e.downcast_ref() {
                        Ok(None)
                    } else if let Some(SVDErrorAt {
                        error: SVDError::EmptyTag(_),
                        ..
                    }) = e.downcast_ref()
                    {
                        Ok(None)
                    } else {
                        Err(e)
                    }
                }
                Ok(s) => Ok(Some(s.to_string())),
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
            .ok_or_else(|| SVDError::MissingTag(format!("{}", k)).at(self.id()).into())
    }

    /// Get text contained by an XML Element
    fn get_text(&self) -> Result<&str> {
        match self.text() {
            Some(s) => Ok(s),
            // FIXME: Doesn't look good because SVDError doesn't format by itself. We already
            // capture the element and this information can be used for getting the name
            // This would fix ParseError
            None => Err(SVDError::EmptyTag(self.tag_name().name().to_string())
                .at(self.id())
                .into()),
        }
    }

    /// Get a named child element from an XML Element
    fn get_child_elem(&self, n: &str) -> Result<Node> {
        self.get_child(n)
            .ok_or_else(|| SVDError::MissingTag(n.to_string()).at(self.id()).into())
    }

    /// Get a u32 value from a named child element
    fn get_child_u32(&self, n: &str) -> Result<u32> {
        let s = self.get_child_elem(n)?;
        u32::parse(&s, &()).context(SVDError::ParseError.at(self.id()))
    }

    /// Get a u64 value from a named child element
    fn get_child_u64(&self, n: &str) -> Result<u64> {
        let s = self.get_child_elem(n)?;
        u64::parse(&s, &()).context(SVDError::ParseError.at(self.id()))
    }

    /// Get a bool value from a named child element
    fn get_child_bool(&self, n: &str) -> Result<bool> {
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
