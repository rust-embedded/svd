//! Encode traits.
//! These support encoding of SVD types to XML

use svd_rs as svd;

use crate::svd::Device;
use xmltree::{Element, EmitterConfig, XMLNode};

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum EncodeError {}

/// Encode trait allows SVD objects to be encoded into XML elements.
pub trait Encode {
    /// Encoding error
    type Error;
    /// Encode into an XML/SVD element
    fn encode(&self) -> Result<Element, Self::Error>;
    fn encode_node(&self) -> Result<XMLNode, Self::Error> {
        self.encode().map(XMLNode::Element)
    }
}

/// EncodeChildren allows SVD objects to be encoded as a list of XML nodes
/// This is typically used to merge with an existing element.
pub trait EncodeChildren {
    /// Encoding error
    type Error;
    /// Encode into XML/SVD children to merge with existing object
    fn encode(&self) -> Result<Vec<XMLNode>, Self::Error>;
}

/// Encodes a device object to an SVD (XML) string
pub fn encode(d: &Device) -> Result<String, EncodeError> {
    let root = d.encode()?;
    let mut wr = Vec::new();
    let mut cfg = EmitterConfig::new();
    cfg.perform_indent = true;
    cfg.pad_self_closing = false;
    root.write_with_config(&mut wr, cfg).unwrap();
    Ok(String::from_utf8(wr).unwrap())
}

/// Defines extensions for implementation over xmltree::Element
trait ElementMerge {
    fn merge(&mut self, n: &Self);
}
/// Implements extensions for xmltree::Element
impl ElementMerge for Element {
    // Merges the children of two elements, maintaining the name and description of the first
    fn merge(&mut self, r: &Self) {
        self.children.extend(r.children.iter().cloned());
        for (key, val) in &r.attributes {
            self.attributes.insert(key.clone(), val.clone());
        }
    }
}

/// Helper to create new base xml nodes
pub(crate) fn new_node(name: &str, text: String) -> XMLNode {
    let mut e = Element::new(name);
    e.children.push(XMLNode::Text(text));
    XMLNode::Element(e)
}

mod access;
mod addressblock;
mod bitrange;
mod cluster;
mod clusterinfo;
mod cpu;
mod device;
mod dimelement;
mod endian;
mod enumeratedvalue;
mod enumeratedvalues;
mod field;
mod fieldinfo;
mod interrupt;
mod modifiedwritevalues;
mod peripheral;
mod register;
mod registercluster;
mod registerinfo;
mod registerproperties;
mod usage;
mod writeconstraint;
