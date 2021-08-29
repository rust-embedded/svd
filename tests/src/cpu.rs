use super::run_test;
use crate::svd::{Cpu, Endian, ValidateLevel};

#[test]
fn decode_encode() {
    let tests = vec![(
        Cpu::builder()
            .name("EFM32JG12B500F512GM48".to_string())
            .revision("5.1.1".to_string())
            .endian(Endian::Little)
            .mpu_present(true)
            .fpu_present(true)
            .nvic_priority_bits(8)
            .has_vendor_systick(false)
            .build(ValidateLevel::Strict)
            .unwrap(),
        "
                <cpu>
                    <name>EFM32JG12B500F512GM48</name>  
                    <revision>5.1.1</revision>
                    <endian>little</endian>
                    <mpuPresent>true</mpuPresent>
                    <fpuPresent>true</fpuPresent>
                    <nvicPrioBits>8</nvicPrioBits>
                    <vendorSystickConfig>false</vendorSystickConfig>
                </cpu>
            ",
    )];

    run_test::<Cpu>(&tests[..]);
}
