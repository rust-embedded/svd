use super::{new_element, Element, EncodeError};
use crate::svd::{BitRange, BitRangeType};

// TODO: Encode method differs from Encode trait as it acts on a set of possible children, create an interface or decide how to better do this
pub fn encode_bitrange(br: &BitRange) -> Result<Vec<Element>, EncodeError> {
    match br.range_type {
        BitRangeType::BitRange => Ok(vec![new_element(
            "bitRange",
            Some(format!("[{}:{}]", br.msb(), br.lsb())),
        )]),
        BitRangeType::MsbLsb => Ok(vec![
            new_element("lsb", Some(format!("{}", br.lsb()))),
            new_element("msb", Some(format!("{}", br.msb()))),
        ]),
        BitRangeType::OffsetWidth => Ok(vec![
            new_element("bitOffset", Some(format!("{}", br.offset))),
            new_element("bitWidth", Some(format!("{}", br.width))),
        ]),
    }
}
