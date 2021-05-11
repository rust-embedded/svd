use super::{new_element, Element, Encode, EncodeError};

use crate::svd::ModifiedWriteValues;

impl Encode for ModifiedWriteValues {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
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

        Ok(new_element("modifiedWriteValues", Some(v.into())))
    }
}
