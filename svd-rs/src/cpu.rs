use super::{BuildError, Endian, SvdError, ValidateLevel};
/// CPU describes the processor included in the microcontroller device.
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "camelCase")
)]
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Cpu {
    /// Processor architecture
    pub name: String,

    /// Define the HW revision of the processor
    pub revision: String,

    /// Define the endianness of the processor
    pub endian: Endian,

    /// Indicate whether the processor is equipped with a memory protection unit (MPU)
    pub mpu_present: bool,

    /// Indicate whether the processor is equipped with a hardware floating point unit (FPU)
    pub fpu_present: bool,

    /// Indicate whether the processor is equipped with a double precision floating point unit.
    /// This element is valid only when `fpu_present` is set to `true`
    #[cfg_attr(feature = "serde", serde(rename = "fpuDP"))]
    pub fpu_double_precision: Option<bool>,

    /// Indicates whether the processor implements the optional SIMD DSP extensions (DSP)
    pub dsp_present: Option<bool>,

    /// Indicate whether the processor has an instruction cache
    pub icache_present: Option<bool>,

    /// Indicate whether the processor has a data cache
    pub dcache_present: Option<bool>,

    /// Indicate whether the processor has an instruction tightly coupled memory
    pub itcm_present: Option<bool>,

    /// Indicate whether the processor has a data tightly coupled memory
    pub dtcm_present: Option<bool>,

    /// Indicate whether the Vector Table Offset Register (VTOR) is implemented.
    /// If not specified, then VTOR is assumed to be present
    pub vtor_present: Option<bool>,

    /// Define the number of bits available in the Nested Vectored Interrupt Controller (NVIC) for configuring priority
    #[cfg_attr(feature = "serde", serde(rename = "nvicPrioBits"))]
    pub nvic_priority_bits: u32,

    /// Indicate whether the processor implements a vendor-specific System Tick Timer
    #[cfg_attr(feature = "serde", serde(rename = "vendorSystickConfig"))]
    pub has_vendor_systick: bool,

    /// Add 1 to the highest interrupt number and specify this number in here
    pub device_num_interrupts: Option<u32>,

    /// Indicate the amount of regions in the Security Attribution Unit (SAU)
    pub sau_num_regions: Option<u32>,
    // sauRegionsConfig
}

/// Builder for [`Cpu`]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CpuBuilder {
    name: Option<String>,
    revision: Option<String>,
    endian: Option<Endian>,
    mpu_present: Option<bool>,
    fpu_present: Option<bool>,
    fpu_double_precision: Option<bool>,
    dsp_present: Option<bool>,
    icache_present: Option<bool>,
    dcache_present: Option<bool>,
    itcm_present: Option<bool>,
    dtcm_present: Option<bool>,
    vtor_present: Option<bool>,
    nvic_priority_bits: Option<u32>,
    has_vendor_systick: Option<bool>,
    device_num_interrupts: Option<u32>,
    sau_num_regions: Option<u32>,
}

impl From<Cpu> for CpuBuilder {
    fn from(c: Cpu) -> Self {
        Self {
            name: Some(c.name),
            revision: Some(c.revision),
            endian: Some(c.endian),
            mpu_present: Some(c.mpu_present),
            fpu_present: Some(c.fpu_present),
            fpu_double_precision: c.fpu_double_precision,
            dsp_present: c.dsp_present,
            icache_present: c.icache_present,
            dcache_present: c.dcache_present,
            itcm_present: c.itcm_present,
            dtcm_present: c.dtcm_present,
            vtor_present: c.vtor_present,
            nvic_priority_bits: Some(c.nvic_priority_bits),
            has_vendor_systick: Some(c.has_vendor_systick),
            device_num_interrupts: c.device_num_interrupts,
            sau_num_regions: c.sau_num_regions,
        }
    }
}

impl CpuBuilder {
    /// Set the name of the cpu.
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    /// Set the revision of the cpu.
    pub fn revision(mut self, value: String) -> Self {
        self.revision = Some(value);
        self
    }
    /// Set the endian of the cpu.
    pub fn endian(mut self, value: Endian) -> Self {
        self.endian = Some(value);
        self
    }
    /// Set the mpu_present of the cpu.
    pub fn mpu_present(mut self, value: bool) -> Self {
        self.mpu_present = Some(value);
        self
    }
    /// Set the fpu_present of the cpu.
    pub fn fpu_present(mut self, value: bool) -> Self {
        self.fpu_present = Some(value);
        self
    }
    /// Set the fpu_double_precision of the cpu.
    pub fn fpu_double_precision(mut self, value: Option<bool>) -> Self {
        self.fpu_double_precision = value;
        self
    }
    /// Set the dsp_present of the cpu.
    pub fn dsp_present(mut self, value: Option<bool>) -> Self {
        self.dsp_present = value;
        self
    }
    /// Set the icache_present of the cpu.
    pub fn icache_present(mut self, value: Option<bool>) -> Self {
        self.icache_present = value;
        self
    }
    /// Set the dcache_present of the cpu.
    pub fn dcache_present(mut self, value: Option<bool>) -> Self {
        self.dcache_present = value;
        self
    }
    /// Set the itcm_present of the cpu.
    pub fn itcm_present(mut self, value: Option<bool>) -> Self {
        self.itcm_present = value;
        self
    }
    /// Set the dtcm_present of the cpu.
    pub fn dtcm_present(mut self, value: Option<bool>) -> Self {
        self.dtcm_present = value;
        self
    }
    /// Set the vtor_present of the cpu.
    pub fn vtor_present(mut self, value: Option<bool>) -> Self {
        self.vtor_present = value;
        self
    }
    /// Set the nvic_priority_bits of the cpu.
    pub fn nvic_priority_bits(mut self, value: u32) -> Self {
        self.nvic_priority_bits = Some(value);
        self
    }
    /// Set the has_vendor_systick of the cpu.
    pub fn has_vendor_systick(mut self, value: bool) -> Self {
        self.has_vendor_systick = Some(value);
        self
    }
    /// Set the device_num_interrupts of the cpu.
    pub fn device_num_interrupts(mut self, value: Option<u32>) -> Self {
        self.device_num_interrupts = value;
        self
    }
    /// Set the sau_num_regions of the cpu.
    pub fn sau_num_regions(mut self, value: Option<u32>) -> Self {
        self.sau_num_regions = value;
        self
    }
    /// Validate and build a [`Cpu`].
    pub fn build(self, lvl: ValidateLevel) -> Result<Cpu, SvdError> {
        let mut cpu = Cpu {
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
            fpu_double_precision: self.fpu_double_precision,
            dsp_present: self.dsp_present,
            icache_present: self.icache_present,
            dcache_present: self.dcache_present,
            itcm_present: self.itcm_present,
            dtcm_present: self.dtcm_present,
            vtor_present: self.vtor_present,
            nvic_priority_bits: self
                .nvic_priority_bits
                .ok_or_else(|| BuildError::Uninitialized("nvic_priority_bits".to_string()))?,
            has_vendor_systick: self
                .has_vendor_systick
                .ok_or_else(|| BuildError::Uninitialized("has_vendor_systick".to_string()))?,
            device_num_interrupts: self.device_num_interrupts,
            sau_num_regions: self.sau_num_regions,
        };
        if !lvl.is_disabled() {
            cpu.validate(lvl)?;
        }
        Ok(cpu)
    }
}

impl Cpu {
    /// Make a builder for [`Cpu`]
    pub fn builder() -> CpuBuilder {
        CpuBuilder::default()
    }
    /// Modify an existing [`Cpu`] based on a [builder](CpuBuilder).
    pub fn modify_from(&mut self, builder: CpuBuilder, lvl: ValidateLevel) -> Result<(), SvdError> {
        if let Some(name) = builder.name {
            self.name = name;
        }
        if let Some(revision) = builder.revision {
            self.revision = revision;
        }
        if let Some(endian) = builder.endian {
            self.endian = endian;
        }
        if let Some(mpu_present) = builder.mpu_present {
            self.mpu_present = mpu_present;
        }
        if let Some(fpu_present) = builder.fpu_present {
            self.fpu_present = fpu_present;
        }
        if builder.fpu_double_precision.is_some() {
            self.fpu_double_precision = builder.fpu_double_precision;
        }
        if builder.dsp_present.is_some() {
            self.dsp_present = builder.dsp_present;
        }
        if builder.icache_present.is_some() {
            self.icache_present = builder.icache_present;
        }
        if builder.dcache_present.is_some() {
            self.dcache_present = builder.dcache_present;
        }
        if builder.itcm_present.is_some() {
            self.itcm_present = builder.itcm_present;
        }
        if builder.dtcm_present.is_some() {
            self.dtcm_present = builder.dtcm_present;
        }
        if builder.vtor_present.is_some() {
            self.vtor_present = builder.vtor_present;
        }
        if let Some(nvic_priority_bits) = builder.nvic_priority_bits {
            self.nvic_priority_bits = nvic_priority_bits;
        }
        if let Some(has_vendor_systick) = builder.has_vendor_systick {
            self.has_vendor_systick = has_vendor_systick;
        }
        if builder.device_num_interrupts.is_some() {
            self.device_num_interrupts = builder.device_num_interrupts;
        }
        if builder.sau_num_regions.is_some() {
            self.sau_num_regions = builder.sau_num_regions;
        }
        if !lvl.is_disabled() {
            self.validate(lvl)
        } else {
            Ok(())
        }
    }
    /// Validate the [`Cpu`]
    pub fn validate(&mut self, _lvl: ValidateLevel) -> Result<(), SvdError> {
        // TODO
        Ok(())
    }
    /// Check if the [`Cpu`] is a Cortex-M
    pub fn is_cortex_m(&self) -> bool {
        self.name.starts_with("CM")
    }
}
