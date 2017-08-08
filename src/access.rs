extern crate xmltree;

use std::collections::HashMap;

use xmltree::Element;

use helpers::*;

macro_rules! try {
    ($e:expr) => {
        $e.expect(concat!(file!(), ":", line!(), " ", stringify!($e)))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Access {
    ReadOnly,
    ReadWrite,
    ReadWriteOnce,
    WriteOnce,
    WriteOnly,
}

impl Parse for Access {
    fn parse(tree: &Element) -> Access {
        let text = try!(tree.text.as_ref());

        match &text[..] {
            "read-only" => Access::ReadOnly,
            "read-write" => Access::ReadWrite,
            "read-writeOnce" => Access::ReadWriteOnce,
            "write-only" => Access::WriteOnly,
            "writeOnce" => Access::WriteOnce,
            _ => panic!("unknown access variant: {}", text),
        }
    }
}

impl Encode for Access {
    fn encode(&self) -> Element {
        let text = match *self {
            Access::ReadOnly => String::from("read-only"),
            Access::ReadWrite => String::from("read-write"),
            Access::ReadWriteOnce => String::from("read-writeOnce"),
            Access::WriteOnly => String::from("write-only"),
            Access::WriteOnce => String::from("writeOnce"),
        };

        Element{
            name: String::from("access"),
            attributes: HashMap::new(),
            children: Vec::new(),
            text: Some(text),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_encode() {
        let types = vec![
            (Access::ReadOnly,        String::from("<access>read-only</access>")),
            (Access::ReadWrite,       String::from("<access>read-write</access>")),
            (Access::ReadWriteOnce,   String::from("<access>read-writeOnce</access>")),
            (Access::WriteOnly,       String::from("<access>write-only</access>")),
            (Access::WriteOnce,       String::from("<access>writeOnce</access>"))
        ];

        for (a, s) in types {
            let tree1 = &try!(Element::parse(s.as_bytes()));
            let access = Access::parse(tree1);
            assert_eq!(access, a, "Parsing `{}` expected `{:?}`", s, a);
            let tree2 = &access.encode();
            assert_eq!(tree1, tree2, "Encoding {:?} expected {}", a, s);
        }
    }
}