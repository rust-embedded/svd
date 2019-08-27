//! SVD objects.
//! This module defines components of an SVD along with parse and encode implementations
#[cfg(feature = "serde_svd")]
extern crate serde;

pub mod endian;
pub use self::endian::Endian;

pub mod cpu;
pub use self::cpu::Cpu;

pub mod interrupt;
pub use self::interrupt::Interrupt;

pub mod access;
pub use self::access::Access;

pub mod bitrange;
pub use self::bitrange::BitRange;

pub mod writeconstraint;
pub use self::writeconstraint::WriteConstraint;

pub mod usage;
pub use self::usage::Usage;

pub mod enumeratedvalue;
pub use self::enumeratedvalue::EnumeratedValue;

pub mod enumeratedvalues;
pub use self::enumeratedvalues::EnumeratedValues;

pub mod field;
pub use self::field::Field;

pub mod registerinfo;
pub use self::registerinfo::RegisterInfo;

pub mod registerproperties;
pub use self::registerproperties::RegisterProperties;

pub mod addressblock;
pub use self::addressblock::AddressBlock;

pub mod cluster;
pub use self::cluster::Cluster;

pub mod clusterinfo;
pub use self::clusterinfo::ClusterInfo;

pub mod register;
pub use self::register::Register;

pub mod registercluster;
pub use self::registercluster::RegisterCluster;

pub mod registerclusterarrayinfo;
pub use self::registerclusterarrayinfo::RegisterClusterArrayInfo;

pub mod peripheral;
pub use self::peripheral::Peripheral;

pub mod device;
pub use self::device::Device;

pub mod modifiedwritevalues;
pub use self::modifiedwritevalues::ModifiedWriteValues;
