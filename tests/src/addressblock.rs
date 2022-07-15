use super::run_test;
use crate::svd::{AddressBlock, AddressBlockUsage, ValidateLevel};

#[test]
fn decode_encode() {
    let parse_config = svd_parser::Config::default();
    let mut encode_config = svd_encoder::Config::default();
    encode_config.update("address_block_offset", "Dec");
    encode_config.update("address_block_size", "UpperHex8");

    let tests = vec![(
        AddressBlock::builder()
            .offset(0)
            .size(0x00000F00)
            .usage(AddressBlockUsage::Registers)
            .protection(None)
            .build(ValidateLevel::Strict)
            .unwrap(),
        "<addressBlock>
            <offset>0x0</offset>
            <size>0xF00</size>
            <usage>registers</usage>
        </addressBlock>",
        "<addressBlock>
            <offset>0</offset>
            <size>0x00000F00</size>
            <usage>registers</usage>
        </addressBlock>",
    )];

    run_test::<AddressBlock>(&tests[..], Some(parse_config), Some(encode_config));
}
