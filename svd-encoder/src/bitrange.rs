use super::{new_node, Config, EncodeError, XMLNode};
use crate::{
    config::FieldBitRangeFormat,
    svd::{BitRange, BitRangeType},
};

// TODO: Encode method differs from Encode trait as it acts on a set of possible children, create an interface or decide how to better do this
pub fn encode_bitrange(br: &BitRange, config: &Config) -> Result<Vec<XMLNode>, EncodeError> {
    match (config.field_bit_range, br.range_type) {
        (Some(FieldBitRangeFormat(BitRangeType::BitRange)), _) | (None, BitRangeType::BitRange) => {
            Ok(vec![new_node("bitRange", br.bit_range())])
        }
        (Some(FieldBitRangeFormat(BitRangeType::MsbLsb)), _) | (None, BitRangeType::MsbLsb) => {
            Ok(vec![
                new_node("lsb", format!("{}", br.lsb())),
                new_node("msb", format!("{}", br.msb())),
            ])
        }
        (Some(FieldBitRangeFormat(BitRangeType::OffsetWidth)), _)
        | (None, BitRangeType::OffsetWidth) => Ok(vec![
            new_node("bitOffset", format!("{}", br.offset)),
            new_node("bitWidth", format!("{}", br.width)),
        ]),
    }
}
