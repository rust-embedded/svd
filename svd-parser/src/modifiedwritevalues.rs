use super::{elementext::ElementExt, Config, Node, Parse, Result, SVDError};

use crate::svd::ModifiedWriteValues;
impl Parse for ModifiedWriteValues {
    type Object = Self;
    type Error = anyhow::Error;
    type Config = Config;

    fn parse(tree: &Node, _config: &Self::Config) -> Result<Self> {
        use self::ModifiedWriteValues::*;
        let text = tree.get_text()?;

        Ok(match text {
            "oneToClear" => OneToClear,
            "oneToSet" => OneToSet,
            "oneToToggle" => OneToToggle,
            "zeroToClear" => ZeroToClear,
            "zeroToSet" => ZeroToSet,
            "zeroToToggle" => ZeroToToggle,
            "clear" => Clear,
            "set" => Set,
            "modify" => Modify,
            s => {
                return Err(SVDError::InvalidModifiedWriteValues(s.into())
                    .at(tree.id())
                    .into())
            }
        })
    }
}
