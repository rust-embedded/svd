use std::str::FromStr;

use convert_case::{Boundary, Case, Casing};

use crate::svd::BitRangeType;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdentifierFormat {
    /// `Camel` case
    ///
    /// `aCamelCaseExample`
    Camel,
    /// `Pascal` case
    ///
    /// `APascalCaseExample`
    Pascal,
    /// `Snake` case
    ///
    /// `a_snake_case_example`
    Snake,
    /// `Constant` case
    ///
    /// `A_CONSTANT_CASE_EXAMPLE`
    Constant,
}

impl FromStr for IdentifierFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Camel" => Ok(IdentifierFormat::Camel),
            "Pascal" => Ok(IdentifierFormat::Pascal),
            "Snake" => Ok(IdentifierFormat::Snake),
            "Constant" => Ok(IdentifierFormat::Constant),
            _ => Err(()),
        }
    }
}

pub fn change_case(s: &str, case: Option<IdentifierFormat>) -> String {
    match case {
        None => s.to_string(),
        Some(case) => {
            let boundary = [
                Boundary::Underscore,
                Boundary::Hyphen,
                Boundary::Space,
                Boundary::LowerUpper,
                Boundary::UpperLower,
                Boundary::Acronym,
            ];

            s.with_boundaries(&boundary)
                .to_case(match case {
                    IdentifierFormat::Camel => Case::Camel,
                    IdentifierFormat::Pascal => Case::Pascal,
                    IdentifierFormat::Snake => Case::Snake,
                    IdentifierFormat::Constant => Case::UpperSnake,
                })
                .replace("%S", "%s")
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NumberFormat {
    /// `UpperHex` format
    ///
    /// `0xABCD`
    UpperHex,
    /// `UpperHex8` format
    ///
    /// `0x0000ABCD`
    UpperHex8,
    /// `UpperHex16` format
    ///
    /// `0x0000ABCD` `0x0000000123456789`
    UpperHex16,
    /// `LowerHex` format
    ///
    /// `0xabcdef`
    LowerHex,
    /// `LowerHex8` format
    ///
    /// `0x0000abcd`
    LowerHex8,
    /// `LowerHex16` format
    ///
    /// `0x0000abcd` `0x0000000123456789`
    LowerHex16,
    /// `Dec` format
    ///
    /// `12345`
    Dec,
    /// `Bin`
    ///
    /// `0b10101010`
    Bin,
}

impl FromStr for NumberFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UpperHex" => Ok(NumberFormat::UpperHex),
            "UpperHex8" => Ok(NumberFormat::UpperHex8),
            "UpperHex16" => Ok(NumberFormat::UpperHex16),
            "LowerHex" => Ok(NumberFormat::LowerHex),
            "LowerHex8" => Ok(NumberFormat::LowerHex8),
            "LowerHex16" => Ok(NumberFormat::LowerHex16),
            "Dec" => Ok(NumberFormat::Dec),
            "Bin" => Ok(NumberFormat::Bin),
            _ => Err(()),
        }
    }
}

pub fn format_number<T>(value: T, format: NumberFormat) -> String
where
    T: std::fmt::UpperHex
        + std::fmt::LowerHex
        + std::fmt::Display
        + std::fmt::Binary
        + Into<u64>
        + Copy,
{
    match format {
        NumberFormat::UpperHex => format!("{:#X}", value),
        NumberFormat::UpperHex8 => format!("{:#010X}", value),
        NumberFormat::UpperHex16 => {
            if value.into() > u32::MAX as u64 {
                format!("{:#018X}", value)
            } else {
                format!("{:#010X}", value)
            }
        }
        NumberFormat::LowerHex => format!("{:#x}", value),
        NumberFormat::LowerHex8 => format!("{:#010x}", value),
        NumberFormat::LowerHex16 => {
            if value.into() > u32::MAX as u64 {
                format!("{:#018x}", value)
            } else {
                format!("{:#010x}", value)
            }
        }
        NumberFormat::Dec => format!("{}", value),
        NumberFormat::Bin => format!("{:#b}", value),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldBitRangeFormat(pub BitRangeType);

impl FromStr for FieldBitRangeFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BitRange" => Ok(FieldBitRangeFormat(BitRangeType::BitRange)),
            "OffsetWidth" => Ok(FieldBitRangeFormat(BitRangeType::OffsetWidth)),
            "MsbLsb" => Ok(FieldBitRangeFormat(BitRangeType::MsbLsb)),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
/// Apply a kind of sorting
pub enum Sorting {
    /// Sort by addresses of offsets
    Offset,
    /// Same as [`Sorting::Offset`], but reversed
    OffsetReversed,
    /// Sort by name
    Name,
}

impl Sorting {
    fn from_parts(parts: &[&str]) -> Option<Self> {
        if parts.contains(&"Offset") {
            Some(Self::Offset)
        } else if parts.contains(&"OffsetReserved") {
            Some(Self::OffsetReversed)
        } else if parts.contains(&"Name") {
            Some(Self::Name)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DerivableSorting {
    Unchanged(Option<Sorting>),
    DeriveLast(Option<Sorting>),
}

impl DerivableSorting {
    fn from_parts(parts: &[&str]) -> Self {
        let sorting = Sorting::from_parts(parts);
        if parts.contains(&"DerivedLast") {
            Self::DeriveLast(sorting)
        } else {
            Self::Unchanged(sorting)
        }
    }
}

impl FromStr for DerivableSorting {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(',').collect::<Vec<_>>();
        Ok(DerivableSorting::from_parts(&parts))
    }
}

impl Default for DerivableSorting {
    fn default() -> Self {
        Self::Unchanged(None)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RcSorting {
    Unchanged(DerivableSorting),
    RegistersFirst(DerivableSorting),
    ClustersFirst(DerivableSorting),
}

impl Default for RcSorting {
    fn default() -> Self {
        Self::Unchanged(Default::default())
    }
}

impl FromStr for RcSorting {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(',').collect::<Vec<_>>();
        let derivable_sorting = DerivableSorting::from_parts(&parts);
        Ok(if parts.contains(&"RegistersFirst") {
            Self::RegistersFirst(derivable_sorting)
        } else if parts.contains(&"ClustersFirst") {
            Self::ClustersFirst(derivable_sorting)
        } else {
            Self::Unchanged(derivable_sorting)
        })
    }
}

#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
/// Advanced encoder options
pub struct Config {
    /// Format of peripheral's name-kind elements
    /// - `derivedFrom`
    /// - `name`
    /// - `alternatePeripheral`
    /// - `prependToName`
    /// - `appendToName`
    /// - `headerStructName`
    pub peripheral_name: Option<IdentifierFormat>,

    /// Format of peripheral's baseAddress element
    ///
    /// format: hex, dec
    pub peripheral_base_address: NumberFormat,

    /// Sort peripherals in specified order
    pub peripheral_sorting: DerivableSorting,

    /// Format of addressBlock's offset element
    ///
    /// format: hex, dec
    pub address_block_offset: NumberFormat,

    /// Format of addressBlock's size element
    ///
    /// format: hex, dec
    pub address_block_size: NumberFormat,

    /// Format of interrupt's name-kind elements
    /// - `name`
    pub interrupt_name: Option<IdentifierFormat>,

    /// Format of cluster's name-kind elements
    /// - `derivedFrom`
    /// - `name`
    /// - `alternateCluster`
    pub cluster_name: Option<IdentifierFormat>,

    /// Format of cluster's addressOffset element
    ///
    /// format: hex, dec
    pub cluster_address_offset: NumberFormat,

    /// Sort registers and clusters in specified order
    pub register_cluster_sorting: RcSorting,

    /// Format of register's name-kind elements
    /// - `derivedFrom`
    /// - `name`
    /// - `alternateRegister`
    pub register_name: Option<IdentifierFormat>,

    /// Format of register's addressOffset element
    ///
    /// format: hex, dec
    pub register_address_offset: NumberFormat,

    /// Format of register's size element
    ///
    /// format: hex, dec
    pub register_size: NumberFormat,

    /// Format of register's resetValue element
    ///
    /// format: hex, dec
    pub register_reset_value: NumberFormat,

    /// Format of register's resetMask element
    ///
    /// format: hex, dec
    pub register_reset_mask: NumberFormat,

    /// Format of field's name-kind elements
    /// - `derivedFrom`
    /// - `name`
    pub field_name: Option<IdentifierFormat>,

    /// Format of field's bitRange
    ///
    /// `None` means keep the original bitRange
    pub field_bit_range: Option<FieldBitRangeFormat>,

    /// Sort fields in specified order
    pub field_sorting: DerivableSorting,

    /// Format of enumeratedValues's name-kind elements
    /// - `derivedFrom`
    /// - `name`
    pub enumerated_values_name: Option<IdentifierFormat>,

    /// Format of enumeratedValue's name-kind elements
    /// - `name`
    pub enumerated_value_name: Option<IdentifierFormat>,

    /// Format of enumeratedValue's value element
    ///
    /// format: hex, dec, bing
    pub enumerated_value_value: NumberFormat,

    /// Format of dim's dim element
    ///
    /// format: hex, dec
    pub dim_dim: NumberFormat,

    /// Format of dim's dimIncrement element
    ///
    /// format: hex, dec
    pub dim_increment: NumberFormat,

    /// Format of dimArrayIndex's headerEnumName element
    pub dim_array_index_header_enum_name: Option<IdentifierFormat>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            peripheral_name: None,
            peripheral_base_address: NumberFormat::UpperHex8,
            peripheral_sorting: Default::default(),
            address_block_offset: NumberFormat::UpperHex,
            address_block_size: NumberFormat::UpperHex,
            interrupt_name: None,
            cluster_name: None,
            cluster_address_offset: NumberFormat::UpperHex,
            register_cluster_sorting: Default::default(),
            register_name: None,
            register_address_offset: NumberFormat::UpperHex,
            register_size: NumberFormat::LowerHex,
            register_reset_value: NumberFormat::UpperHex16,
            register_reset_mask: NumberFormat::UpperHex16,
            field_name: None,
            field_bit_range: None,
            field_sorting: Default::default(),
            enumerated_values_name: None,
            enumerated_value_name: None,
            enumerated_value_value: NumberFormat::Dec,
            dim_dim: NumberFormat::Dec,
            dim_increment: NumberFormat::UpperHex,
            dim_array_index_header_enum_name: None,
        }
    }
}

impl Config {
    /// Update the config with given name and value
    ///
    /// panic if the value is not valid
    pub fn update(&mut self, name: &str, value: &str) {
        match name {
            "peripheral_name" => self.peripheral_name = Some(value.parse().unwrap()),
            "peripheral_base_address" => self.peripheral_base_address = value.parse().unwrap(),
            "peripheral_sorting" => self.peripheral_sorting = value.parse().unwrap(),
            "address_block_offset" => self.address_block_offset = value.parse().unwrap(),
            "address_block_size" => self.address_block_size = value.parse().unwrap(),
            "interrupt_name" => self.interrupt_name = Some(value.parse().unwrap()),
            "cluster_name" => self.cluster_name = Some(value.parse().unwrap()),
            "cluster_address_offset" => self.cluster_address_offset = value.parse().unwrap(),
            "register_cluster_sorting" => self.register_cluster_sorting = value.parse().unwrap(),
            "register_name" => self.register_name = Some(value.parse().unwrap()),
            "register_address_offset" => self.register_address_offset = value.parse().unwrap(),
            "register_size" => self.register_size = value.parse().unwrap(),
            "register_reset_value" => self.register_reset_value = value.parse().unwrap(),
            "register_reset_mask" => self.register_reset_mask = value.parse().unwrap(),
            "field_name" => self.field_name = Some(value.parse().unwrap()),
            "field_bit_range" => self.field_bit_range = Some(value.parse().unwrap()),
            "field_sorting" => self.field_sorting = value.parse().unwrap(),
            "enumerated_values_name" => self.enumerated_values_name = Some(value.parse().unwrap()),
            "enumerated_value_name" => self.enumerated_value_name = Some(value.parse().unwrap()),
            "enumerated_value_value" => self.enumerated_value_value = value.parse().unwrap(),
            "dim_dim" => self.dim_dim = value.parse().unwrap(),
            "dim_increment" => self.dim_increment = value.parse().unwrap(),
            "dim_array_index_header_enum_name" => {
                self.dim_array_index_header_enum_name = Some(value.parse().unwrap())
            }
            _ => {
                eprintln!("Unknown config key: {}", name);
            }
        }
    }

    /// Format of peripheral's name-kind elements
    pub fn peripheral_name(mut self, val: Option<IdentifierFormat>) -> Self {
        self.peripheral_name = val;
        self
    }

    /// Format of peripheral's baseAddress element
    ///
    /// format: hex, dec
    pub fn peripheral_base_address(mut self, val: NumberFormat) -> Self {
        self.peripheral_base_address = val;
        self
    }

    /// Sort peripherals in specified order
    ///
    /// `None` means keep the original order
    pub fn peripheral_sorting(mut self, val: DerivableSorting) -> Self {
        self.peripheral_sorting = val;
        self
    }

    /// Format of addressBlock's offset element
    ///
    /// format: hex, dec
    pub fn address_block_offset(mut self, val: NumberFormat) -> Self {
        self.address_block_offset = val;
        self
    }

    /// Format of addressBlock's size element
    ///
    /// format: hex, dec
    pub fn address_block_size(mut self, val: NumberFormat) -> Self {
        self.address_block_size = val;
        self
    }

    /// Format of interrupt's name-kind elements
    pub fn interrupt_name(mut self, val: Option<IdentifierFormat>) -> Self {
        self.interrupt_name = val;
        self
    }

    /// Format of cluster's name-kind elements
    pub fn cluster_name(mut self, val: Option<IdentifierFormat>) -> Self {
        self.cluster_name = val;
        self
    }

    /// Format of cluster's addressOffset element
    ///
    /// format: hex, dec
    pub fn cluster_address_offset(mut self, val: NumberFormat) -> Self {
        self.cluster_address_offset = val;
        self
    }

    /// Sort registers and clusters in specified order
    ///
    /// `None` means keep the original order
    pub fn register_cluster_sorting(mut self, val: RcSorting) -> Self {
        self.register_cluster_sorting = val;
        self
    }

    /// Format of register's name-kind elements
    pub fn register_name(mut self, val: Option<IdentifierFormat>) -> Self {
        self.register_name = val;
        self
    }

    /// Format of register's addressOffset element
    ///
    /// format: hex, dec
    pub fn register_address_offset(mut self, val: NumberFormat) -> Self {
        self.register_address_offset = val;
        self
    }

    /// Format of register's size element
    ///
    /// format: hex, dec
    pub fn register_size(mut self, val: NumberFormat) -> Self {
        self.register_size = val;
        self
    }

    /// Format of register's resetValue element
    ///
    /// format: hex, dec
    pub fn register_reset_value(mut self, val: NumberFormat) -> Self {
        self.register_reset_value = val;
        self
    }

    /// Format of register's resetMask element
    ///
    /// format: hex, dec
    pub fn register_reset_mask(mut self, val: NumberFormat) -> Self {
        self.register_reset_mask = val;
        self
    }

    /// Format of field's name-kind elements
    pub fn field_name(mut self, val: Option<IdentifierFormat>) -> Self {
        self.field_name = val;
        self
    }

    /// Format of field's bitRange
    ///
    /// `None` means keep the original bitRange
    pub fn field_bit_range(mut self, val: Option<FieldBitRangeFormat>) -> Self {
        self.field_bit_range = val;
        self
    }

    /// Sort fields in specified order
    ///
    /// `None` means keep the original order
    pub fn field_sorting(mut self, val: DerivableSorting) -> Self {
        self.field_sorting = val;
        self
    }

    /// Format of enumeratedValues's name-kind elements
    pub fn enumerated_values_name(mut self, val: Option<IdentifierFormat>) -> Self {
        self.enumerated_values_name = val;
        self
    }

    /// Format of enumeratedValue's name-kind elements
    pub fn enumerated_value_name(mut self, val: Option<IdentifierFormat>) -> Self {
        self.enumerated_value_name = val;
        self
    }

    /// Format of enumeratedValue's value element
    ///
    /// format: hex, dec, bing
    pub fn enumerated_value_value(mut self, val: NumberFormat) -> Self {
        self.enumerated_value_value = val;
        self
    }

    /// Format of dim's dim element
    ///
    /// format: hex, dec
    pub fn dim_dim(mut self, val: NumberFormat) -> Self {
        self.dim_dim = val;
        self
    }

    /// Format of dim's dimIncrement element
    ///
    /// format: hex, dec
    pub fn dim_increment(mut self, val: NumberFormat) -> Self {
        self.dim_increment = val;
        self
    }
}
