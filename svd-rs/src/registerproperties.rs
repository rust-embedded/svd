use super::{Access, Protection, SvdError, ValidateLevel};

/// Errors from [`RegisterProperties::validate`]
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// Value is too large
    #[error("Reset value 0x{0:x} doesn't fit in {1} bits")]
    ValueTooLarge(u64, u32),
    /// reset value is conflicting with the mask
    #[error("Reset value 0x{0:x} conflicts with mask '0x{1:x}'")]
    MaskConflict(u64, u64),
    /// Mask doesn't fit
    #[error("Mask value 0x{0:x} doesn't fit in {1} bits")]
    MaskTooLarge(u64, u32),
}

/// Register default properties
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "camelCase")
)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct RegisterProperties {
    /// Bit-width of register
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub size: Option<u32>,

    /// Access rights for register
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub access: Option<Access>,

    /// Specify the security privilege to access an address region
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub protection: Option<Protection>,

    /// Register value at RESET
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub reset_value: Option<u64>,

    /// Define which register bits have a defined reset value
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub reset_mask: Option<u64>,
}

impl RegisterProperties {
    /// Create a new [`RegisterProperties`].
    pub fn new() -> Self {
        Self::default()
    }
    /// Modify an existing [`RegisterProperties`] based on another.
    pub fn modify_from(
        &mut self,
        builder: RegisterProperties,
        lvl: ValidateLevel,
    ) -> Result<(), SvdError> {
        if builder.size.is_some() {
            self.size = builder.size;
        }
        if builder.access.is_some() {
            self.access = builder.access;
        }
        if builder.protection.is_some() {
            self.protection = builder.protection;
        }
        if builder.reset_value.is_some() {
            self.reset_value = builder.reset_value;
        }
        if builder.reset_mask.is_some() {
            self.reset_mask = builder.reset_mask;
        }
        if !lvl.is_disabled() {
            self.validate(lvl)
        } else {
            Ok(())
        }
    }

    /// Validate the [`RegisterProperties`]
    pub fn validate(&mut self, lvl: ValidateLevel) -> Result<(), SvdError> {
        check_reset_value(self.size, self.reset_value, self.reset_mask, lvl)?;
        Ok(())
    }
    /// Set the size of the register properties.
    pub fn size(mut self, value: Option<u32>) -> Self {
        self.size = value;
        self
    }
    /// Set the access of the register properties.
    pub fn access(mut self, value: Option<Access>) -> Self {
        self.access = value;
        self
    }
    /// Set the protection of the register properties.
    pub fn protection(mut self, value: Option<Protection>) -> Self {
        self.protection = value;
        self
    }
    /// Set the reset_value of the register properties.
    pub fn reset_value(mut self, value: Option<u64>) -> Self {
        self.reset_value = value;
        self
    }
    /// Set the reset_mask of the register properties.
    pub fn reset_mask(mut self, value: Option<u64>) -> Self {
        self.reset_mask = value;
        self
    }
    /// Validate and build a [`RegisterProperties`].
    pub fn build(mut self, lvl: ValidateLevel) -> Result<RegisterProperties, SvdError> {
        if !lvl.is_disabled() {
            self.validate(lvl)?;
        }
        Ok(self)
    }
}

pub(crate) fn check_reset_value(
    size: Option<u32>,
    value: Option<u64>,
    mask: Option<u64>,
    lvl: ValidateLevel,
) -> Result<(), Error> {
    const MAX_BITS: u32 = core::u64::MAX.count_ones();

    if let (Some(size), Some(value)) = (size, value) {
        if MAX_BITS - value.leading_zeros() > size {
            return Err(Error::ValueTooLarge(value, size));
        }
    }
    if lvl.is_strict() {
        if let (Some(size), Some(mask)) = (size, mask) {
            if MAX_BITS - mask.leading_zeros() > size {
                return Err(Error::MaskTooLarge(mask, size));
            }
        }
        if let (Some(value), Some(mask)) = (value, mask) {
            if value & mask != value {
                return Err(Error::MaskConflict(value, mask));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{check_reset_value, ValidateLevel};

    #[test]
    fn test_check_reset_value() {
        let lvl = ValidateLevel::Strict;
        check_reset_value(None, None, None, lvl).unwrap();
        check_reset_value(Some(8), None, None, lvl).unwrap();
        check_reset_value(Some(8), None, Some(0xff), lvl).unwrap();
        check_reset_value(Some(32), Some(0xfaceface), None, lvl).unwrap();
        check_reset_value(Some(32), Some(0xfaceface), Some(0xffffffff), lvl).unwrap();

        assert!(
            check_reset_value(Some(8), None, Some(0x100), lvl).is_err(),
            "mask shouldn't fit in size"
        );
        assert!(
            check_reset_value(Some(1), Some(0x02), None, lvl).is_err(),
            "reset value shouldn't fit in field"
        );
        assert!(
            check_reset_value(Some(8), Some(0x80), Some(0x01), lvl).is_err(),
            "value should conflict with mask"
        );
    }
}
