use super::run_test;
use crate::svd::{
    Access, BitRange, BitRangeType, FieldInfo, ModifiedWriteValues, RegisterInfo, ValidateLevel,
};

#[test]
fn decode_encode() {
    let tests = vec![(
        RegisterInfo::builder()
            .name("WRITECTRL".to_string())
            .alternate_group(Some("alternate_group".to_string()))
            .alternate_register(Some("alternate_register".to_string()))
            .derived_from(Some("derived_from".to_string()))
            .description(Some("Write Control Register".to_string()))
            .address_offset(8)
            .size(Some(32))
            .access(Some(Access::ReadWrite))
            .reset_value(Some(0x00000000))
            .reset_mask(Some(0x00000023))
            .fields(Some(vec![FieldInfo::builder()
                .name("WREN".to_string())
                .description(Some("Enable Write/Erase Controller".to_string()))
                .bit_range(BitRange {
                    offset: 0,
                    width: 1,
                    range_type: BitRangeType::OffsetWidth,
                })
                .access(Some(Access::ReadWrite))
                .build(ValidateLevel::Strict)
                .unwrap()
                .single()]))
            .modified_write_values(Some(ModifiedWriteValues::OneToToggle))
            .build(ValidateLevel::Strict)
            .unwrap(),
        "
        <register derivedFrom=\"derived_from\">
            <name>WRITECTRL</name>
            <description>Write Control Register</description>
            <alternateGroup>alternate_group</alternateGroup>
            <alternateRegister>alternate_register</alternateRegister>
            <addressOffset>0x8</addressOffset>
            <size>0x20</size>
            <access>read-write</access>
            <resetValue>0x00000000</resetValue>
            <resetMask>0x00000023</resetMask>
            <modifiedWriteValues>oneToToggle</modifiedWriteValues>
            <fields>
                <field>
                    <name>WREN</name>
                    <description>Enable Write/Erase Controller</description>
                    <bitOffset>0</bitOffset>
                    <bitWidth>1</bitWidth>
                    <access>read-write</access>
                </field>
            </fields>
        </register>
        ",
    )];

    run_test::<RegisterInfo>(&tests[..]);
}
