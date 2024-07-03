pub use super::Interrupt;
use super::{BuildError, SvdError, ValidateLevel};

/// Description of HARTs in the device.
pub mod hart;
pub use hart::Hart;

/// Description of interrupt priority levels in the device.
pub mod priority;
pub use priority::Priority;

/// RISC-V specific descriptions.
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "camelCase")
)]
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Riscv {
    /// Core interrupt enumeration values
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub core_interrupts: Vec<Interrupt>,

    /// Priority level enumeration values
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub priorities: Vec<Priority>,

    /// HART enumeration values
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub harts: Vec<Hart>,
}

/// Builder for [`Riscv`]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RiscvBuilder {
    core_interrupts: Option<Vec<Interrupt>>,
    priorities: Option<Vec<Priority>>,
    harts: Option<Vec<Hart>>,
}

impl From<Riscv> for RiscvBuilder {
    fn from(riscv: Riscv) -> Self {
        Self {
            core_interrupts: Some(riscv.core_interrupts),
            priorities: Some(riscv.priorities),
            harts: Some(riscv.harts),
        }
    }
}

impl RiscvBuilder {
    /// Set the core interrupt enumeration values
    pub fn core_interrupts(mut self, core_interrupts: Vec<Interrupt>) -> Self {
        self.core_interrupts = Some(core_interrupts);
        self
    }

    /// Set the priority level enumeration values
    pub fn priorities(mut self, priorities: Vec<Priority>) -> Self {
        self.priorities = Some(priorities);
        self
    }

    /// Set the HART enumeration values
    pub fn harts(mut self, harts: Vec<Hart>) -> Self {
        self.harts = Some(harts);
        self
    }

    /// Validate and build a [`Riscv`].
    pub fn build(self, lvl: ValidateLevel) -> Result<Riscv, SvdError> {
        let riscv = Riscv {
            core_interrupts: self
                .core_interrupts
                .ok_or_else(|| BuildError::Uninitialized("core_interrupts".to_string()))?,
            priorities: self
                .priorities
                .ok_or_else(|| BuildError::Uninitialized("priorities".to_string()))?,
            harts: self
                .harts
                .ok_or_else(|| BuildError::Uninitialized("harts".to_string()))?,
        };
        riscv.validate(lvl)?;
        Ok(riscv)
    }
}

impl Riscv {
    /// Make a builder for [`Riscv`]
    pub fn builder() -> RiscvBuilder {
        RiscvBuilder::default()
    }

    /// Modify an existing [`Riscv`] based on a [builder](RiscvBuilder).
    pub fn modify_from(
        &mut self,
        builder: RiscvBuilder,
        lvl: ValidateLevel,
    ) -> Result<(), SvdError> {
        if let Some(core_interrupts) = builder.core_interrupts {
            self.core_interrupts = core_interrupts;
        }
        if let Some(priorities) = builder.priorities {
            self.priorities = priorities;
        }
        if let Some(harts) = builder.harts {
            self.harts = harts;
        }
        self.validate(lvl)
    }

    /// Validate the [`Riscv`].
    ///
    /// # Errors
    ///
    /// - If any of the core interrupt enumeration values are invalid
    /// - If any of the priority level enumeration values are invalid
    /// - If any of the HART enumeration values are invalid
    pub fn validate(&self, lvl: ValidateLevel) -> Result<(), SvdError> {
        for ci in &self.core_interrupts {
            ci.validate(lvl)?;
        }
        for p in &self.priorities {
            p.validate(lvl)?;
        }
        for h in &self.harts {
            h.validate(lvl)?;
        }
        Ok(())
    }
}
