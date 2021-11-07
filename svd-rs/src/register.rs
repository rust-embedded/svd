use core::ops::{Deref, DerefMut};

use super::{DimElement, RegisterCluster, RegisterInfo};

/// A single register or array of registers
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Register {
    #[cfg_attr(feature = "serde", serde(flatten))]
    /// A description of a register
    pub info: RegisterInfo,
    #[cfg_attr(
        feature = "serde",
        serde(flatten, default, skip_serializing_if = "Option::is_none")
    )]
    /// If `None` it is a single register, if `Some` specifies array attributes
    pub dim: Option<DimElement>,
}

impl Deref for Register {
    type Target = RegisterInfo;

    fn deref(&self) -> &RegisterInfo {
        &self.info
    }
}

impl DerefMut for Register {
    fn deref_mut(&mut self) -> &mut RegisterInfo {
        &mut self.info
    }
}

impl Register {
    /// Construct single [`Register`]
    pub const fn single(info: RegisterInfo) -> Self {
        Self { info, dim: None }
    }
    /// Construct [`Register`] array
    pub const fn array(info: RegisterInfo, dim: DimElement) -> Self {
        Self {
            info,
            dim: Some(dim),
        }
    }
    /// Return `true` if register instance is single
    pub const fn is_single(&self) -> bool {
        matches!(&self.dim, None)
    }
    /// Return `true` if it is register array
    pub const fn is_array(&self) -> bool {
        matches!(&self.dim, Some(_))
    }
}

/// Register iterator
pub struct RegIter<'a> {
    pub(crate) rem: Vec<&'a RegisterCluster>,
}

impl<'a> std::iter::Iterator for RegIter<'a> {
    type Item = &'a Register;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(b) = self.rem.pop() {
            match b {
                RegisterCluster::Register(reg) => {
                    return Some(reg);
                }
                RegisterCluster::Cluster(cluster) => {
                    for c in cluster.children.iter().rev() {
                        self.rem.push(c);
                    }
                }
            }
        }
        None
    }
}

/// Mutable register iterator
pub struct RegIterMut<'a> {
    pub(crate) rem: Vec<&'a mut RegisterCluster>,
}

impl<'a> std::iter::Iterator for RegIterMut<'a> {
    type Item = &'a mut Register;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(b) = self.rem.pop() {
            match b {
                RegisterCluster::Register(reg) => {
                    return Some(reg);
                }
                RegisterCluster::Cluster(cluster) => {
                    for c in cluster.children.iter_mut().rev() {
                        self.rem.push(c);
                    }
                }
            }
        }
        None
    }
}
