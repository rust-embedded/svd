use crate::elementext::ElementExt;
#[cfg(feature = "unproven")]
use std::collections::HashMap;
use xmltree::Element;

use crate::types::Parse;

#[cfg(feature = "unproven")]
use crate::encode::Encode;
use crate::error::*;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ModifiedWriteValues {
    OneToClear,
    OneToSet,
    OneToToggle,
    ZeroToClear,
    ZeroToSet,
    ZeroToToggle,
    Clear,
    Set,
    Modify,
}

impl Parse for ModifiedWriteValues {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        use self::ModifiedWriteValues::*;
        let text = tree.get_text()?;

        Ok(match text.as_ref() {
            "oneToClear" => OneToClear,
            "oneToSet" => OneToSet,
            "oneToToggle" => OneToToggle,
            "zeroToClear" => ZeroToClear,
            "zeroToSet" => ZeroToSet,
            "zeroToToggle" => ZeroToToggle,
            "clear" => Clear,
            "set" => Set,
            "modify" => Modify,
            s => return Err(ModifiedWriteValuesError::Invalid(tree.clone(), s.into()).into()),
        })
    }
}

#[cfg(feature = "unproven")]
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

#[cfg(test)]
#[cfg(feature = "unproven")]
mod tests {
    use super::*;
    use crate::run_test;

    #[test]
    fn decode_encode() {
        // FIXME: Do we need a more extensive test?
        let tests = vec![(
            ModifiedWriteValues::OneToToggle,
            "<modifiedWriteValues>oneToToggle</modifiedWriteValues>",
        )];

        run_test::<ModifiedWriteValues>(&tests[..]);
    }
}
