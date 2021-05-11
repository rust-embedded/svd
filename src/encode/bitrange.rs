use super::{new_element, Element, EncodeError};
use crate::svd::{BitRange, BitRangeType};

impl BitRange {
    // TODO: Encode method differs from Encode trait as it acts on a set of possible children, create an interface or decide how to better do this
    pub fn encode(&self) -> Result<Vec<Element>, EncodeError> {
        match self.range_type {
            BitRangeType::BitRange => Ok(vec![new_element(
                "bitRange",
                Some(format!("[{}:{}]", self.msb(), self.lsb())),
            )]),
            BitRangeType::MsbLsb => Ok(vec![
                new_element("lsb", Some(format!("{}", self.lsb()))),
                new_element("msb", Some(format!("{}", self.msb()))),
            ]),
            BitRangeType::OffsetWidth => Ok(vec![
                new_element("bitOffset", Some(format!("{}", self.offset))),
                new_element("bitWidth", Some(format!("{}", self.width))),
            ]),
        }
    }
}
