//! Implementations of DeriveFrom, setting non-explicit fields.
use crate::{
    Cluster, ClusterInfo, EnumeratedValues, Field, FieldInfo, Peripheral, PeripheralInfo, Register,
    RegisterInfo, RegisterProperties,
};

/// Fill empty fields of structure with values of other structure
pub trait DeriveFrom {
    /// Derive contents
    fn derive_from(&self, other: &Self) -> Self;
}

impl DeriveFrom for ClusterInfo {
    fn derive_from(&self, other: &Self) -> Self {
        let mut derived = self.clone();
        derived.default_register_properties = derived
            .default_register_properties
            .derive_from(&other.default_register_properties);
        derived.header_struct_name = derived
            .header_struct_name
            .or_else(|| other.header_struct_name.clone());
        if derived.children.is_empty() {
            derived.children = other.children.clone();
        }
        derived
    }
}

impl DeriveFrom for EnumeratedValues {
    fn derive_from(&self, other: &Self) -> Self {
        let mut derived = self.clone();
        derived.usage = derived.usage.or_else(|| other.usage.clone());
        if derived.values.is_empty() {
            derived.values = other.values.clone();
        }
        derived
    }
}

impl DeriveFrom for PeripheralInfo {
    fn derive_from(&self, other: &Self) -> Self {
        let mut derived = self.clone();
        derived.version = derived.version.or_else(|| other.version.clone());
        derived.description = derived.description.or_else(|| other.description.clone());
        derived.group_name = derived.group_name.or_else(|| other.group_name.clone());
        derived.prepend_to_name = derived
            .prepend_to_name
            .or_else(|| other.prepend_to_name.clone());
        derived.append_to_name = derived
            .append_to_name
            .or_else(|| other.append_to_name.clone());
        derived.header_struct_name = derived
            .header_struct_name
            .or_else(|| other.header_struct_name.clone());
        derived.default_register_properties = derived
            .default_register_properties
            .derive_from(&other.default_register_properties);
        derived.registers = derived.registers.or_else(|| other.registers.clone());
        if derived.interrupt.is_empty() {
            derived.interrupt = other.interrupt.clone();
        }
        derived
    }
}

impl DeriveFrom for RegisterInfo {
    fn derive_from(&self, other: &Self) -> Self {
        let mut derived = self.clone();
        derived.description = derived.description.or_else(|| other.description.clone());
        derived.properties = derived.properties.derive_from(&other.properties);
        derived.fields = derived.fields.or_else(|| other.fields.clone());
        derived.write_constraint = derived.write_constraint.or(other.write_constraint);
        derived.read_action = derived.read_action.or(other.read_action);
        derived.modified_write_values = derived
            .modified_write_values
            .or(other.modified_write_values);
        derived
    }
}

impl DeriveFrom for RegisterProperties {
    fn derive_from(&self, other: &Self) -> Self {
        let mut derived = self.clone();
        derived.size = derived.size.or(other.size);
        derived.access = derived.access.or(other.access);
        derived.protection = derived.protection.or(other.protection);
        derived.reset_value = derived.reset_value.or(other.reset_value);
        derived.reset_mask = derived.reset_mask.or(other.reset_mask);
        derived
    }
}

impl DeriveFrom for FieldInfo {
    fn derive_from(&self, other: &Self) -> Self {
        let mut derived = self.clone();
        derived.description = derived.description.or_else(|| other.description.clone());
        derived.access = derived.access.or(other.access);
        if derived.enumerated_values.is_empty() {
            derived.enumerated_values = other.enumerated_values.clone();
        }
        derived.write_constraint = derived.write_constraint.or(other.write_constraint);
        derived.read_action = derived.read_action.or(other.read_action);
        derived.modified_write_values = derived
            .modified_write_values
            .or(other.modified_write_values);
        derived
    }
}

impl DeriveFrom for Peripheral {
    fn derive_from(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Single(info), Self::Single(other_info)) => info.derive_from(other_info).single(),
            (Self::Single(info), Self::Array(other_info, other_dim)) => {
                let mut dim = other_dim.clone();
                dim.dim_name = None;
                info.derive_from(other_info).array(dim)
            }
            (Self::Array(info, dim), Self::Single(other_info))
            | (Self::Array(info, dim), Self::Array(other_info, _)) => {
                info.derive_from(other_info).array(dim.clone())
            }
        }
    }
}

impl DeriveFrom for Cluster {
    fn derive_from(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Single(info), Self::Single(other_info)) => info.derive_from(other_info).single(),
            (Self::Single(info), Self::Array(other_info, other_dim)) => {
                let mut dim = other_dim.clone();
                dim.dim_name = None;
                info.derive_from(other_info).array(dim)
            }
            (Self::Array(info, dim), Self::Single(other_info))
            | (Self::Array(info, dim), Self::Array(other_info, _)) => {
                info.derive_from(other_info).array(dim.clone())
            }
        }
    }
}

impl DeriveFrom for Register {
    fn derive_from(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Single(info), Self::Single(other_info)) => info.derive_from(other_info).single(),
            (Self::Single(info), Self::Array(other_info, other_dim)) => {
                let mut dim = other_dim.clone();
                dim.dim_name = None;
                info.derive_from(other_info).array(dim)
            }
            (Self::Array(info, dim), Self::Single(other_info))
            | (Self::Array(info, dim), Self::Array(other_info, _)) => {
                info.derive_from(other_info).array(dim.clone())
            }
        }
    }
}

impl DeriveFrom for Field {
    fn derive_from(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Single(info), Self::Single(other_info)) => info.derive_from(other_info).single(),
            (Self::Single(info), Self::Array(other_info, other_dim)) => {
                let mut dim = other_dim.clone();
                dim.dim_name = None;
                info.derive_from(other_info).array(dim)
            }
            (Self::Array(info, dim), Self::Single(other_info))
            | (Self::Array(info, dim), Self::Array(other_info, _)) => {
                info.derive_from(other_info).array(dim.clone())
            }
        }
    }
}
