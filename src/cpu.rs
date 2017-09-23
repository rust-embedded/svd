extern crate xmltree;

use std::collections::HashMap;

use xmltree::Element;

#[macro_use]
use elementext::*;
use parse;
use helpers::*;
use endian::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Cpu {
    pub name: String,
    pub revision: String,
    pub endian: Endian,
    pub mpu_present: bool,
    pub fpu_present: bool,
    pub nvic_priority_bits: u32,
    pub has_vendor_systick: bool,

    // Reserve the right to add more fields to this struct
    pub(crate) _extensible: (),
}

impl Cpu {
    pub fn is_cortex_m(&self) -> bool {
        self.name.starts_with("CM")
    }
}

impl ParseElem for Cpu {
    fn parse(tree: &Element) -> Cpu {
        // TODO: not sure where CPU comes from here?
        // EFM32 SVDs appear to have "device" key with similar stuff under it.
        assert_eq!(tree.name, "cpu");

        Cpu {
            name: try_get_child!(tree.get_child_text("name")),
            revision: try_get_child!(tree.get_child_text("revision")),
            endian: Endian::parse(try_get_child!(tree.get_child("endian"))),
            mpu_present: try_get_child!(parse::bool(try_get_child!(tree.get_child("mpuPresent")))),
            fpu_present: try_get_child!(parse::bool(try_get_child!(tree.get_child("fpuPresent")))),
            nvic_priority_bits: try_get_child!(parse::u32(try_get_child!(tree.get_child("nvicPrioBits")))),
            has_vendor_systick: try_get_child!(parse::bool(try_get_child!(tree.get_child("vendorSystickConfig")))),

            _extensible: (),
        }
    }
}

impl EncodeElem for Cpu {
    fn encode(&self) -> Element {
        Element {
            name: String::from("cpu"),
            attributes: HashMap::new(),
            children: vec![
                new_element("name", Some(self.name.clone())),
                new_element("revision", Some(self.revision.clone())),
                self.endian.encode(),
                new_element("mpuPresent", Some(format!("{}", self.mpu_present))),
                new_element("fpuPresent", Some(format!("{}", self.fpu_present))),
                new_element("nvicPrioBits", Some(format!("{}", self.nvic_priority_bits))),
                new_element(
                    "vendorSystickConfig",
                    Some(format!("{}", self.has_vendor_systick))
                ),
            ],
            text: None,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_encode() {
        let types = vec![
            (
                Cpu {
                    name: String::from("EFM32JG12B500F512GM48"),
                    revision: String::from("5.1.1"),
                    endian: Endian::Little,
                    mpu_present: true,
                    fpu_present: true,
                    nvic_priority_bits: 8,
                    has_vendor_systick: false,
                    _extensible: (),
                },
                String::from(
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
                )
            ),
        ];

        for (a, s) in types {
            let tree1 = &try_get_child!(Element::parse(s.as_bytes()));
            let value = Cpu::parse(tree1);
            assert_eq!(value, a, "Parsing `{}` expected `{:?}`", s, a);
            let tree2 = value.encode();
            assert_eq!(tree1, &tree2, "Encoding {:?} expected {}", a, s);
        }
    }
}
