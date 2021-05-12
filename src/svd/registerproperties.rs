use super::{Access, SvdError};

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("Reset value 0x{0:x} doesn't fit in {1} bits")]
    ValueTooLarge(u64, u32),
    #[error("Reset value 0x{0:x} conflicts with mask '0x{1:x}'")]
    MaskConflict(u64, u64),
    #[error("Mask value 0x{0:x} doesn't fit in {1} bits")]
    MaskTooLarge(u64, u32),
}

/// Register default properties
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct RegisterProperties {
    /// Bit-width of register
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub size: Option<u32>,

    /// Access rights for register
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub access: Option<Access>,

    /// Register value at RESET
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub reset_value: Option<u64>,

    /// Define which register bits have a defined reset value
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub reset_mask: Option<u64>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RegisterPropertiesBuilder {
    pub size: Option<u32>,
    pub access: Option<Access>,
    pub reset_value: Option<u64>,
    pub reset_mask: Option<u64>,
}

impl From<RegisterProperties> for RegisterPropertiesBuilder {
    fn from(p: RegisterProperties) -> Self {
        Self {
            size: p.size,
            access: p.access,
            reset_value: p.reset_value,
            reset_mask: p.reset_mask,
        }
    }
}

impl RegisterPropertiesBuilder {
    pub fn size(mut self, value: Option<u32>) -> Self {
        self.size = value;
        self
    }
    pub fn access(mut self, value: Option<Access>) -> Self {
        self.access = value;
        self
    }
    pub fn reset_value(mut self, value: Option<u64>) -> Self {
        self.reset_value = value;
        self
    }
    pub fn reset_mask(mut self, value: Option<u64>) -> Self {
        self.reset_mask = value;
        self
    }
    pub fn build(self) -> Result<RegisterProperties, SvdError> {
        RegisterProperties {
            size: self.size,
            access: self.access,
            reset_value: self.reset_value,
            reset_mask: self.reset_mask,
        }
        .validate()
    }
}

impl RegisterProperties {
    pub fn builder() -> RegisterPropertiesBuilder {
        RegisterPropertiesBuilder::default()
    }
    fn validate(self) -> Result<Self, SvdError> {
        check_reset_value(self.size, self.reset_value, self.reset_mask)?;
        Ok(self)
    }
}

pub(crate) fn check_reset_value(
    size: Option<u32>,
    value: Option<u64>,
    _mask: Option<u64>,
) -> Result<(), Error> {
    const MAX_BITS: u32 = core::u64::MAX.count_ones();

    if let (Some(size), Some(value)) = (size, value) {
        if MAX_BITS - value.leading_zeros() > size {
            return Err(Error::ValueTooLarge(value, size).into());
        }
    }
    #[cfg(feature = "strict")]
    {
        if let (Some(size), Some(mask)) = (size, _mask) {
            if MAX_BITS - mask.leading_zeros() > size {
                return Err(Error::MaskTooLarge(mask, size).into());
            }
        }
        if let (Some(value), Some(mask)) = (value, _mask) {
            if value & mask != value {
                return Err(Error::MaskConflict(value, mask).into());
            }
        }
    }

    Ok(())
}

#[cfg(feature = "strict")]
#[cfg(test)]
mod tests {
    use crate::error::check_reset_value;

    #[test]
    fn test_check_reset_value() {
        check_reset_value(None, None, None).unwrap();
        check_reset_value(Some(8), None, None).unwrap();
        check_reset_value(Some(8), None, Some(0xff)).unwrap();
        check_reset_value(Some(32), Some(0xfaceface), None).unwrap();
        check_reset_value(Some(32), Some(0xfaceface), Some(0xffffffff)).unwrap();

        assert!(
            check_reset_value(Some(8), None, Some(0x100)).is_err(),
            "mask shouldn't fit in size"
        );
        assert!(
            check_reset_value(Some(1), Some(0x02), None).is_err(),
            "reset value shouldn't fit in field"
        );
        assert!(
            check_reset_value(Some(8), Some(0x80), Some(0x01)).is_err(),
            "value should conflict with mask"
        );
    }
}
