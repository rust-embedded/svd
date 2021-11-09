use super::{BuildError, Protection, SvdError, ValidateLevel};

///  An uniquely mapped address block to a peripheral
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct AddressBlock {
    /// Specifies the start address of an address block relative to the peripheral [`baseAddress`](crate::Peripheral::base_address).
    pub offset: u32,
    /// Specifies the number of [`addressUnitBits`](crate::Device::address_unit_bits) being covered by this address block.
    pub size: u32,
    /// Usage of the address block.
    pub usage: AddressBlockUsage,
    /// Specify the security privilege to access an address region
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub protection: Option<Protection>,
}

/// Usage of the address block.
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "kebab-case")
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AddressBlockUsage {
    /// Registers
    Registers,
    /// Buffer / Memory
    Buffer,
    /// Reserved
    Reserved,
}

impl Default for AddressBlockUsage {
    fn default() -> Self {
        Self::Registers
    }
}

impl AddressBlockUsage {
    /// Parse a string into an [`AddressBlockUsage`] value, returning [`Option::None`] if the string is not valid.
    pub fn parse_str(s: &str) -> Option<Self> {
        match s {
            "registers" => Some(Self::Registers),
            "buffer" => Some(Self::Buffer),
            "reserved" => Some(Self::Reserved),
            _ => None,
        }
    }

    /// Convert this [`AddressBlockUsage`] into a static string.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Registers => "registers",
            Self::Buffer => "buffer",
            Self::Reserved => "reserved",
        }
    }
}

/// Builder for [`AddressBlock`]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AddressBlockBuilder {
    offset: Option<u32>,
    size: Option<u32>,
    usage: Option<AddressBlockUsage>,
    protection: Option<Protection>,
}

impl From<AddressBlock> for AddressBlockBuilder {
    fn from(d: AddressBlock) -> Self {
        Self {
            offset: Some(d.offset),
            size: Some(d.size),
            usage: Some(d.usage),
            protection: d.protection,
        }
    }
}

impl AddressBlockBuilder {
    /// Set the offset of the block
    pub fn offset(mut self, value: u32) -> Self {
        self.offset = Some(value);
        self
    }
    /// Set the size of the block
    pub fn size(mut self, value: u32) -> Self {
        self.size = Some(value);
        self
    }
    /// Set the usage of the block
    pub fn usage(mut self, value: AddressBlockUsage) -> Self {
        self.usage = Some(value);
        self
    }
    /// Set the protection of the block
    pub fn protection(mut self, value: Option<Protection>) -> Self {
        self.protection = value;
        self
    }
    /// Validate and build a [`AddressBlock`].
    pub fn build(self, lvl: ValidateLevel) -> Result<AddressBlock, SvdError> {
        let mut de = AddressBlock {
            offset: self
                .offset
                .ok_or_else(|| BuildError::Uninitialized("offset".to_string()))?,
            size: self
                .size
                .ok_or_else(|| BuildError::Uninitialized("size".to_string()))?,
            usage: self
                .usage
                .ok_or_else(|| BuildError::Uninitialized("usage".to_string()))?,
            protection: self.protection,
        };
        if !lvl.is_disabled() {
            de.validate(lvl)?;
        }
        Ok(de)
    }
}

impl AddressBlock {
    /// Make a builder for [`AddressBlock`]
    pub fn builder() -> AddressBlockBuilder {
        AddressBlockBuilder::default()
    }
    /// Modify an existing [`AddressBlock`] based on a [builder](AddressBlockBuilder).
    pub fn modify_from(
        &mut self,
        builder: AddressBlockBuilder,
        lvl: ValidateLevel,
    ) -> Result<(), SvdError> {
        if let Some(offset) = builder.offset {
            self.offset = offset;
        }
        if let Some(size) = builder.size {
            self.size = size;
        }
        if let Some(usage) = builder.usage {
            self.usage = usage;
        }
        if builder.protection.is_some() {
            self.protection = builder.protection;
        }
        if !lvl.is_disabled() {
            self.validate(lvl)
        } else {
            Ok(())
        }
    }
    /// Validate the [`AddressBlock`].
    ///
    /// # Notes
    ///
    /// This doesn't do anything.
    pub fn validate(&mut self, _lvl: ValidateLevel) -> Result<(), SvdError> {
        Ok(())
    }
}
