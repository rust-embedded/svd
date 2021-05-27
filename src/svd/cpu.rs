use super::{BuildError, Endian, SvdError};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
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
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CpuBuilder {
    name: Option<String>,
    revision: Option<String>,
    endian: Option<Endian>,
    mpu_present: Option<bool>,
    fpu_present: Option<bool>,
    nvic_priority_bits: Option<u32>,
    has_vendor_systick: Option<bool>,
}

impl From<Cpu> for CpuBuilder {
    fn from(c: Cpu) -> Self {
        Self {
            name: Some(c.name),
            revision: Some(c.revision),
            endian: Some(c.endian),
            mpu_present: Some(c.mpu_present),
            fpu_present: Some(c.fpu_present),
            nvic_priority_bits: Some(c.nvic_priority_bits),
            has_vendor_systick: Some(c.has_vendor_systick),
        }
    }
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
    pub fn build(self) -> Result<Cpu, SvdError> {
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
        })
        .validate()
    }
}

impl Cpu {
    pub fn builder() -> CpuBuilder {
        CpuBuilder::default()
    }
    #[allow(clippy::unnecessary_wraps)]
    fn validate(self) -> Result<Self, SvdError> {
        // TODO
        Ok(self)
    }
    pub fn is_cortex_m(&self) -> bool {
        self.name.starts_with("CM")
    }
}
