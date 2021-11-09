use super::run_test;
use crate::svd::{AddressBlock, AddressBlockUsage, ValidateLevel};

#[test]
fn decode_encode() {
    let tests = vec![(
        AddressBlock::builder()
            .offset(0)
            .size(0x00000800)
            .usage(AddressBlockUsage::Registers)
            .protection(None)
            .build(ValidateLevel::Strict)
            .unwrap(),
        "<addressBlock>
            <offset>0x0</offset>
            <size>0x800</size>
            <usage>registers</usage>
        </addressBlock>",
    )];

    run_test::<AddressBlock>(&tests[..]);
}
