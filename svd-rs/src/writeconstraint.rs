use super::SvdError;

/// Define constraints for writing values to a field
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "camelCase")
)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriteConstraint {
    /// If `true`, only the last read value can be written.
    WriteAsRead(bool),
    /// If `true`, only the values listed in the enumeratedValues list can be written.
    UseEnumeratedValues(bool),
    /// A range of numbers that can be written.
    Range(WriteConstraintRange),
}

/// The smallest and largest number that can be written.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WriteConstraintRange {
    /// Specify the smallest number to be written to the field
    #[cfg_attr(feature = "serde", serde(rename = "minimum"))]
    pub min: u64,
    /// Specify the largest number to be written to the field.
    #[cfg_attr(feature = "serde", serde(rename = "maximum"))]
    pub max: u64,
}

/// Errors for [`WriteConstraintRange::check_range`]
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// The value is not in range.
    #[error("Value {0} out of range {1:?}")]
    OutOfRange(u64, core::ops::Range<u64>),
    /// Minimum is greater than maximum.
    #[error("Range minimum {0} is greater than maximum {1}")]
    ReversedRange(u64, u64),
}

impl WriteConstraintRange {
    pub(crate) fn check_range(&self, range: core::ops::Range<u64>) -> Result<(), SvdError> {
        if self.min > self.max {
            return Err(Error::ReversedRange(self.min, self.max).into());
        }
        for v in [&self.min, &self.max] {
            if !range.contains(v) {
                return Err(Error::OutOfRange(*v, range.clone()).into());
            }
        }
        Ok(())
    }
}
