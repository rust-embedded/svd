#[cfg(feature = "unproven")]
use std::collections::HashMap;

use xmltree::Element;

use crate::elementext::ElementExt;
#[cfg(feature = "unproven")]
use crate::encode::Encode;
use crate::error::*;
#[cfg(feature = "unproven")]
use crate::new_element;
use crate::svd::endian::Endian;
use crate::types::Parse;

use crate::Build;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Cpu {
    pub name: String,

    /// Define the HW revision of the processor
    pub revision: String,

    /// Define the endianness of the processor
    pub endian: Endian,

    /// Indicate whether the processor is equipped with a memory protection unit (MPU)
    pub mpu_present: bool,

    /// Indicate whether the processor is equipped with a hardware floating point unit (FPU)
    pub fpu_present: bool,

    /// Define the number of bits available in the Nested Vectored Interrupt Controller (NVIC) for configuring priority
    pub nvic_priority_bits: u32,

    /// Indicate whether the processor implements a vendor-specific System Tick Timer
    pub has_vendor_systick: bool,

    // Reserve the right to add more fields to this struct
    #[cfg_attr(feature = "serde", serde(skip))]
    _extensible: (),
}

impl Build for Cpu {
    type Builder = CpuBuilder;
}

#[derive(Default)]
pub struct CpuBuilder {
    name: Option<String>,
    revision: Option<String>,
    endian: Option<Endian>,
    mpu_present: Option<bool>,
    fpu_present: Option<bool>,
    nvic_priority_bits: Option<u32>,
    has_vendor_systick: Option<bool>,
}

impl CpuBuilder {
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    pub fn revision(mut self, value: String) -> Self {
        self.revision = Some(value);
        self
    }
    pub fn endian(mut self, value: Endian) -> Self {
        self.endian = Some(value);
        self
    }
    pub fn mpu_present(mut self, value: bool) -> Self {
        self.mpu_present = Some(value);
        self
    }
    pub fn fpu_present(mut self, value: bool) -> Self {
        self.fpu_present = Some(value);
        self
    }
    pub fn nvic_priority_bits(mut self, value: u32) -> Self {
        self.nvic_priority_bits = Some(value);
        self
    }
    pub fn has_vendor_systick(mut self, value: bool) -> Self {
        self.has_vendor_systick = Some(value);
        self
    }
    pub fn build(self) -> Result<Cpu> {
        (Cpu {
            name: self
                .name
                .ok_or_else(|| BuildError::Uninitialized("name".to_string()))?,
            revision: self
                .revision
                .ok_or_else(|| BuildError::Uninitialized("revision".to_string()))?,
            endian: self
                .endian
                .ok_or_else(|| BuildError::Uninitialized("endian".to_string()))?,
            mpu_present: self
                .mpu_present
                .ok_or_else(|| BuildError::Uninitialized("mpu_present".to_string()))?,
            fpu_present: self
                .fpu_present
                .ok_or_else(|| BuildError::Uninitialized("fpu_present".to_string()))?,
            nvic_priority_bits: self
                .nvic_priority_bits
                .ok_or_else(|| BuildError::Uninitialized("nvic_priority_bits".to_string()))?,
            has_vendor_systick: self
                .has_vendor_systick
                .ok_or_else(|| BuildError::Uninitialized("has_vendor_systick".to_string()))?,
            _extensible: (),
        })
        .validate()
    }
}

impl Cpu {
    fn validate(self) -> Result<Self> {
        // TODO
        Ok(self)
    }
}

impl Parse for Cpu {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        if tree.name != "cpu" {
            return Err(ParseError::NameMismatch(tree.clone()).into());
        }

        CpuBuilder::default()
            .name(tree.get_child_text("name")?)
            .revision(tree.get_child_text("revision")?)
            .endian(Endian::parse(tree.get_child_elem("endian")?)?)
            .mpu_present(tree.get_child_bool("mpuPresent")?)
            .fpu_present(tree.get_child_bool("fpuPresent")?)
            .nvic_priority_bits(tree.get_child_u32("nvicPrioBits")?)
            .has_vendor_systick(tree.get_child_bool("vendorSystickConfig")?)
            .build()
    }
}

#[cfg(feature = "unproven")]
impl Encode for Cpu {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        Ok(Element {
            prefix: None,
            namespace: None,
            namespaces: None,
            name: String::from("cpu"),
            attributes: HashMap::new(),
            children: vec![
                new_element("name", Some(self.name.clone())),
                new_element("revision", Some(self.revision.clone())),
                self.endian.encode()?,
                new_element("mpuPresent", Some(format!("{}", self.mpu_present))),
                new_element("fpuPresent", Some(format!("{}", self.fpu_present))),
                new_element("nvicPrioBits", Some(format!("{}", self.nvic_priority_bits))),
                new_element(
                    "vendorSystickConfig",
                    Some(format!("{}", self.has_vendor_systick)),
                ),
            ],
            text: None,
        })
    }
}

impl Cpu {
    pub fn is_cortex_m(&self) -> bool {
        self.name.starts_with("CM")
    }
}

#[cfg(test)]
#[cfg(feature = "unproven")]
mod tests {
    use super::*;
    use crate::run_test;

    #[test]
    fn decode_encode() {
        let tests = vec![(
            CpuBuilder::default()
                .name("EFM32JG12B500F512GM48".to_string())
                .revision("5.1.1".to_string())
                .endian(Endian::Little)
                .mpu_present(true)
                .fpu_present(true)
                .nvic_priority_bits(8)
                .has_vendor_systick(false)
                .build()
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
}
