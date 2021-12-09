use super::{array::SingleArray, FieldInfo};

/// Describes a field or fields of a [register](crate::RegisterInfo).
pub type Field = SingleArray<FieldInfo>;
