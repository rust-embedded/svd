/// Define constraints for writing values to a field
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WriteConstraintRange {
    /// Specify the smallest number to be written to the field
    #[cfg_attr(feature = "serde", serde(rename = "minimum"))]
    pub min: u64,
    /// Specify the largest number to be written to the field.
    #[cfg_attr(feature = "serde", serde(rename = "maximum"))]
    pub max: u64,
}
