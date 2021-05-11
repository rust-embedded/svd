use std::collections::HashMap;
use xmltree::Element;

use crate::encode::Encode;
use crate::error::*;

use crate::svd::ModifiedWriteValues;
impl Encode for ModifiedWriteValues {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        use self::ModifiedWriteValues::*;
        let v = match *self {
            OneToClear => "oneToClear",
            OneToSet => "oneToSet",
            OneToToggle => "oneToToggle",
            ZeroToClear => "zeroToClear",
            ZeroToSet => "zeroToSet",
            ZeroToToggle => "zeroToToggle",
            Clear => "clear",
            Set => "set",
            Modify => "modify",
        };

        Ok(Element {
            prefix: None,
            namespace: None,
            namespaces: None,
            name: String::from("modifiedWriteValues"),
            attributes: HashMap::new(),
            children: vec![],
            text: Some(v.into()),
        })
    }
}
