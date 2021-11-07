use core::ops::{Deref, DerefMut};

use super::{DimElement, FieldInfo};

/// A single field or array of fields
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    #[cfg_attr(feature = "serde", serde(flatten))]
    /// A description of a field
    pub info: FieldInfo,
    #[cfg_attr(
        feature = "serde",
        serde(flatten, default, skip_serializing_if = "Option::is_none")
    )]
    /// If `None` it is a single field, if `Some` specifies array attributes
    pub dim: Option<DimElement>,
}

impl Deref for Field {
    type Target = FieldInfo;

    fn deref(&self) -> &FieldInfo {
        &self.info
    }
}

impl DerefMut for Field {
    fn deref_mut(&mut self) -> &mut FieldInfo {
        &mut self.info
    }
}

impl Field {
    /// Construct single [`Field`]
    pub const fn single(info: FieldInfo) -> Self {
        Self { info, dim: None }
    }
    /// Construct [`Field`] array
    pub const fn array(info: FieldInfo, dim: DimElement) -> Self {
        Self {
            info,
            dim: Some(dim),
        }
    }
    /// Return `true` if field instance is single
    pub const fn is_single(&self) -> bool {
        matches!(&self.dim, None)
    }
    /// Return `true` if it is field array
    pub const fn is_array(&self) -> bool {
        matches!(&self.dim, Some(_))
    }
}
