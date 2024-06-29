use std::vec;

use super::run_test;
use crate::svd::{
    riscv::{Hart, Priority, Riscv},
    Interrupt, ValidateLevel,
};

#[test]
fn decode_encode() {
    let interrupts = vec![
        Interrupt::builder()
            .name("MachineSoft".to_string())
            .description(Some("Machine Software Interrupt".to_string()))
            .value(3)
            .build(ValidateLevel::Strict)
            .unwrap(),
        Interrupt::builder()
            .name("MachineTimer".to_string())
            .description(Some("Machine Timer Interrupt".to_string()))
            .value(7)
            .build(ValidateLevel::Strict)
            .unwrap(),
        Interrupt::builder()
            .name("MachineExternal".to_string())
            .description(Some("Machine External Interrupt".to_string()))
            .value(11)
            .build(ValidateLevel::Strict)
            .unwrap(),
    ];

    let priorities = vec![
        Priority::builder()
            .name("P0".to_string())
            .description(Some("Priority level 0".to_string()))
            .value(0)
            .build(ValidateLevel::Strict)
            .unwrap(),
        Priority::builder()
            .name("P1".to_string())
            .description(Some("Priority level 1".to_string()))
            .value(1)
            .build(ValidateLevel::Strict)
            .unwrap(),
        Priority::builder()
            .name("P2".to_string())
            .description(Some("Priority level 2".to_string()))
            .value(2)
            .build(ValidateLevel::Strict)
            .unwrap(),
        Priority::builder()
            .name("P3".to_string())
            .description(Some("Priority level 3".to_string()))
            .value(3)
            .build(ValidateLevel::Strict)
            .unwrap(),
        Priority::builder()
            .name("P4".to_string())
            .description(Some("Priority level 4".to_string()))
            .value(4)
            .build(ValidateLevel::Strict)
            .unwrap(),
        Priority::builder()
            .name("P5".to_string())
            .description(Some("Priority level 5".to_string()))
            .value(5)
            .build(ValidateLevel::Strict)
            .unwrap(),
        Priority::builder()
            .name("P6".to_string())
            .description(Some("Priority level 6".to_string()))
            .value(6)
            .build(ValidateLevel::Strict)
            .unwrap(),
        Priority::builder()
            .name("P7".to_string())
            .description(Some("Priority level 7".to_string()))
            .value(7)
            .build(ValidateLevel::Strict)
            .unwrap(),
    ];

    let harts = vec![Hart::builder()
        .name("H0".to_string())
        .description(Some("Hart 0".to_string()))
        .value(0)
        .build(ValidateLevel::Strict)
        .unwrap()];

    let tests = vec![(
        Riscv::builder()
            .clint(Some("CLINT".to_string()))
            .plic(Some("PLIC".to_string()))
            .core_interrupts(interrupts)
            .priorities(priorities)
            .harts(harts)
            .build(ValidateLevel::Strict)
            .unwrap(),
        "
                <riscv>
                    <clint>CLINT</clint>  
                    <plic>PLIC</plic>
                    <coreInterrupts>
                        <interrupt>
                            <name>MachineSoft</name>
                            <description>Machine Software Interrupt</description>
                            <value>3</value>
                        </interrupt>
                        <interrupt>
                            <name>MachineTimer</name>
                            <description>Machine Timer Interrupt</description>
                            <value>7</value>
                        </interrupt>
                        <interrupt>
                            <name>MachineExternal</name>
                            <description>Machine External Interrupt</description>
                            <value>11</value>
                        </interrupt>
                    </coreInterrupts>
                    <priorities>
                        <priority>
                            <name>P0</name>
                            <description>Priority level 0</description>
                            <value>0</value>
                        </priority>
                        <priority>
                            <name>P1</name>
                            <description>Priority level 1</description>
                            <value>1</value>
                        </priority>
                        <priority>
                            <name>P2</name>
                            <description>Priority level 2</description>
                            <value>2</value>
                        </priority>
                        <priority>
                            <name>P3</name>
                            <description>Priority level 3</description>
                            <value>3</value>
                        </priority>
                        <priority>
                            <name>P4</name>
                            <description>Priority level 4</description>
                            <value>4</value>
                        </priority>
                        <priority>
                            <name>P5</name>
                            <description>Priority level 5</description>
                            <value>5</value>
                        </priority>
                        <priority>
                            <name>P6</name>
                            <description>Priority level 6</description>
                            <value>6</value>
                        </priority>
                        <priority>
                            <name>P7</name>
                            <description>Priority level 7</description>
                            <value>7</value>
                        </priority>
                    </priorities>
                    <harts>
                        <hart>
                            <name>H0</name>
                            <description>Hart 0</description>
                            <value>0</value>
                        </hart>
                    </harts>
                </riscv>
            ",
        "
                <riscv>
                    <clint>CLINT</clint>  
                    <plic>PLIC</plic>
                    <coreInterrupts>
                        <interrupt>
                            <name>MachineSoft</name>
                            <description>Machine Software Interrupt</description>
                            <value>3</value>
                        </interrupt>
                        <interrupt>
                            <name>MachineTimer</name>
                            <description>Machine Timer Interrupt</description>
                            <value>7</value>
                        </interrupt>
                        <interrupt>
                            <name>MachineExternal</name>
                            <description>Machine External Interrupt</description>
                            <value>11</value>
                        </interrupt>
                    </coreInterrupts>
                    <priorities>
                        <priority>
                            <name>P0</name>
                            <description>Priority level 0</description>
                            <value>0</value>
                        </priority>
                        <priority>
                            <name>P1</name>
                            <description>Priority level 1</description>
                            <value>1</value>
                        </priority>
                        <priority>
                            <name>P2</name>
                            <description>Priority level 2</description>
                            <value>2</value>
                        </priority>
                        <priority>
                            <name>P3</name>
                            <description>Priority level 3</description>
                            <value>3</value>
                        </priority>
                        <priority>
                            <name>P4</name>
                            <description>Priority level 4</description>
                            <value>4</value>
                        </priority>
                        <priority>
                            <name>P5</name>
                            <description>Priority level 5</description>
                            <value>5</value>
                        </priority>
                        <priority>
                            <name>P6</name>
                            <description>Priority level 6</description>
                            <value>6</value>
                        </priority>
                        <priority>
                            <name>P7</name>
                            <description>Priority level 7</description>
                            <value>7</value>
                        </priority>
                    </priorities>
                    <harts>
                        <hart>
                            <name>H0</name>
                            <description>Hart 0</description>
                            <value>0</value>
                        </hart>
                    </harts>
                </riscv>
            ",
    )];

    run_test::<Riscv>(&tests[..], None, None);
}
