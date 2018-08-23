use xmltree::Element;

use parse;
use types::{Parse, Encode};
use new_element;
use error::*;

use svd::access::Access;

/// Register default properties
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Defaults {
    pub size: Option<u32>,
    pub reset_value: Option<u32>,
    pub reset_mask: Option<u32>,
    pub access: Option<Access>,
    // Reserve the right to add more fields to this struct
    _extensible: (),
}

impl Parse for Defaults {
    type Object = Defaults;
    type Error = SVDError;

    fn parse(tree: &Element) -> Result<Defaults, SVDError> {
        Ok(Defaults {
            size: parse::optional::<u32>("size", tree)?,
            reset_value: parse::optional::<u32>("resetValue", tree)?,
            reset_mask: parse::optional::<u32>("resetMask", tree)?,
            access: parse::optional::<Access>("access", tree)?,
            _extensible: (),
        })
    }
}

impl Defaults {
    pub fn encode_children(&self) -> Result<Vec<Element>, SVDError>  {
        let mut children = Vec::new();

        match self.size {
            Some(ref v) => {
                children.push(new_element("size", Some(format!("0x{:08.x}", v))));
            }
            None => (),
        };

        match self.reset_value {
            Some(ref v) => {
                children.push(new_element("resetValue", Some(format!("0x{:08.x}", v))));
            }
            None => (),
        };

        match self.reset_mask {
            Some(ref v) => {
                children.push(new_element("resetMask", Some(format!("0x{:08.x}", v))));
            }
            None => (),
        };

        match self.access {
            Some(ref v) => {
                children.push(v.encode()?);
            }
            None => (),
        };

        Ok(children)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_encode() {
        let example = String::from(
            "
            <mock>
                <size>0xaabbccdd</size>
                <resetValue>0x11223344</resetValue>
                <resetMask>0x00000000</resetMask>
                <access>read-only</access>
            </mock>
        ",
        );
        
        let expected = Defaults {
            size: Some(0xaabbccdd),
            reset_value: Some(0x11223344),
            reset_mask: Some(0x00000000),
            access: Some(Access::ReadOnly),
            _extensible: (),
        };

        let tree1 = Element::parse(example.as_bytes()).unwrap();

        let parsed = Defaults::parse(&tree1).unwrap();
        assert_eq!(parsed, expected, "Parsing tree failed");

        let mut tree2 = new_element("mock", None);
        tree2.children = parsed.encode_children().unwrap();
        assert_eq!(tree1, tree2, "Encoding value failed");
    }
}
