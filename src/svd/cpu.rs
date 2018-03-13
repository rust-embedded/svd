
use xmltree::Element;

use ::parse;
use ::types::Parse;
use ::error::SVDError;
use ::svd::endian::Endian;


#[derive(Clone, Debug)]
pub struct Cpu {
    pub name: String,
    pub revision: String,
    pub endian: Endian,
    pub mpu_present: bool,
    pub fpu_present: bool,
    pub nvic_priority_bits: u32,
    pub has_vendor_systick: bool,

    // Reserve the right to add more fields to this struct
    _extensible: (),
}

impl Parse for Cpu {
    type Object = Cpu;
    type Error = SVDError;

    fn parse2(tree: &Element) -> Result<Cpu, SVDError> {
        if tree.name != "cpu" {
            return Err(SVDError::NameMismatch(tree.clone()));
        }

        Ok(Cpu {
            name: parse::get_child_string("name", tree)?,
            revision: parse::get_child_string("revision", tree)?,
            endian: Endian::parse2(parse::get_child_elem("endian", tree)?)?,
            mpu_present: parse::get_child_bool("mpuPresent", tree)?,
            fpu_present: parse::get_child_bool("fpuPresent", tree)?,
            nvic_priority_bits: parse::get_child_u32("nvicPrioBits", tree)?,
            has_vendor_systick: parse::get_child_bool("vendorSystickConfig", tree)?,
            _extensible: (),
        })
    }
}

impl Cpu {
    pub fn is_cortex_m(&self) -> bool {
        self.name.starts_with("CM")
    }
}