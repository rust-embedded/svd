use super::{elementext::ElementExt, Element, Parse};

use crate::error::*;

use crate::svd::ModifiedWriteValues;
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
            s => return Err(SVDError::InvalidModifiedWriteValues(tree.clone(), s.into()).into()),
        })
    }
}
