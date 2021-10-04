use super::{new_node, EncodeError, XMLNode};
use crate::svd::{BitRange, BitRangeType};

// TODO: Encode method differs from Encode trait as it acts on a set of possible children, create an interface or decide how to better do this
pub fn encode_bitrange(br: &BitRange) -> Result<Vec<XMLNode>, EncodeError> {
    match br.range_type {
        BitRangeType::BitRange => Ok(vec![new_node(
            "bitRange",
            format!("[{}:{}]", br.msb(), br.lsb()),
        )]),
        BitRangeType::MsbLsb => Ok(vec![
            new_node("lsb", format!("{}", br.lsb())),
            new_node("msb", format!("{}", br.msb())),
        ]),
        BitRangeType::OffsetWidth => Ok(vec![
            new_node("bitOffset", format!("{}", br.offset)),
            new_node("bitWidth", format!("{}", br.width)),
        ]),
    }
}
