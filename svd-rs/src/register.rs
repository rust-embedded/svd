#[doc(hidden)]
pub use super::registercluster::{AllRegistersIter as RegIter, AllRegistersIterMut as RegIterMut};
use super::{array::SingleArray, RegisterInfo};

/// A single register or array of registers. A register is a named, programmable resource that belongs to a [peripheral](crate::Peripheral).
pub type Register = SingleArray<RegisterInfo>;
