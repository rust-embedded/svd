
use xmltree::Element;

use ::parse;
use ::types::{Parse, Encode, new_element};
use ::error::SVDError;


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Access {
    ReadOnly,
    ReadWrite,
    ReadWriteOnce,
    WriteOnce,
    WriteOnly,
}

impl Parse for Access {
    type Object = Access;
    type Error = SVDError;

    fn parse(tree: &Element) -> Result<Access, SVDError> {
        let text = parse::get_text(tree)?;

        match &text[..] {
            "read-only" => Ok(Access::ReadOnly),
            "read-write" => Ok(Access::ReadWrite),
            "read-writeOnce" => Ok(Access::ReadWriteOnce),
            "write-only" => Ok(Access::WriteOnly),
            "writeOnce" => Ok(Access::WriteOnce),
            _ => Err(SVDError::UnknownAccessType(tree.clone())),
        }
    }
}

impl Encode for Access {
    type Error = SVDError;

    fn encode(&self) -> Result<Element, SVDError> {
        let text = match *self {
            Access::ReadOnly => String::from("read-only"),
            Access::ReadWrite => String::from("read-write"),
            Access::ReadWriteOnce => String::from("read-writeOnce"),
            Access::WriteOnly => String::from("write-only"),
            Access::WriteOnce => String::from("writeOnce"),
        };

        Ok(new_element("access", Some(text)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_encode() {
        let types = vec![
            (Access::ReadOnly, String::from("<access>read-only</access>")),
            (
                Access::ReadWrite,
                String::from("<access>read-write</access>")
            ),
            (
                Access::ReadWriteOnce,
                String::from("<access>read-writeOnce</access>")
            ),
            (
                Access::WriteOnly,
                String::from("<access>write-only</access>")
            ),
            (
                Access::WriteOnce,
                String::from("<access>writeOnce</access>")
            ),
        ];

        for (a, s) in types {
            let tree1 = Element::parse(s.as_bytes()).unwrap();
            let access = Access::parse(&tree1).unwrap();
            assert_eq!(access, a, "Parsing `{}` expected `{:?}`", s, a);
            let tree2 = access.encode().unwrap();
            assert_eq!(tree1, tree2, "Encoding {:?} expected {}", a, s);
        }
    }
}
