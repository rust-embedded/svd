use super::{Cluster, Register};

/// A [cluster](crate::Cluster) or a [register](crate::Register)
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[derive(Clone, Debug, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum RegisterCluster {
    /// Register
    Register(Box<Register>),
    /// Cluster
    Cluster(Box<Cluster>),
}

impl From<Register> for RegisterCluster {
    fn from(reg: Register) -> Self {
        Self::Register(Box::new(reg))
    }
}

impl From<Cluster> for RegisterCluster {
    fn from(cluser: Cluster) -> Self {
        Self::Cluster(Box::new(cluser))
    }
}

impl From<Box<Register>> for RegisterCluster {
    fn from(reg: Box<Register>) -> Self {
        Self::Register(reg)
    }
}

impl From<Box<Cluster>> for RegisterCluster {
    fn from(cluser: Box<Cluster>) -> Self {
        Self::Cluster(cluser)
    }
}

impl RegisterCluster {
    /// Name of register or cluster
    pub fn name(&self) -> &String {
        match self {
            Self::Register(r) => &r.name,
            Self::Cluster(c) => &c.name,
        }
    }
    /// Description of register or cluster
    pub fn description(&self) -> &Option<String> {
        match self {
            Self::Register(r) => &r.description,
            Self::Cluster(c) => &c.description,
        }
    }
    /// Specify the name from which to inherit data
    pub fn derived_from(&self) -> &Option<String> {
        match self {
            Self::Register(r) => &r.derived_from,
            Self::Cluster(c) => &c.derived_from,
        }
    }
    /// Address offset of register or cluster
    pub fn address_offset(&self) -> u32 {
        match self {
            Self::Register(r) => r.address_offset,
            Self::Cluster(c) => c.address_offset,
        }
    }
}

/// Register iterator
pub struct RegisterIter<'a> {
    pub(crate) all: std::slice::Iter<'a, RegisterCluster>,
}

impl<'a> std::iter::Iterator for RegisterIter<'a> {
    type Item = &'a Register;
    fn next(&mut self) -> Option<Self::Item> {
        match self.all.next() {
            None => None,
            Some(RegisterCluster::Register(reg)) => Some(reg),
            _ => self.next(),
        }
    }
}

/// Mutable register iterator
pub struct RegisterIterMut<'a> {
    pub(crate) all: std::slice::IterMut<'a, RegisterCluster>,
}

impl<'a> std::iter::Iterator for RegisterIterMut<'a> {
    type Item = &'a mut Register;
    fn next(&mut self) -> Option<Self::Item> {
        match self.all.next() {
            None => None,
            Some(RegisterCluster::Register(reg)) => Some(reg),
            _ => self.next(),
        }
    }
}

/// Cluster iterator
pub struct ClusterIter<'a> {
    pub(crate) all: std::slice::Iter<'a, RegisterCluster>,
}

impl<'a> std::iter::Iterator for ClusterIter<'a> {
    type Item = &'a Cluster;
    fn next(&mut self) -> Option<Self::Item> {
        match self.all.next() {
            None => None,
            Some(RegisterCluster::Cluster(c)) => Some(c),
            _ => self.next(),
        }
    }
}

/// Mutable cluster iterator
pub struct ClusterIterMut<'a> {
    pub(crate) all: std::slice::IterMut<'a, RegisterCluster>,
}

impl<'a> std::iter::Iterator for ClusterIterMut<'a> {
    type Item = &'a mut Cluster;
    fn next(&mut self) -> Option<Self::Item> {
        match self.all.next() {
            None => None,
            Some(RegisterCluster::Cluster(c)) => Some(c),
            _ => self.next(),
        }
    }
}

/// Iterator over all registers
pub struct AllRegistersIter<'a> {
    pub(crate) rem: Vec<&'a RegisterCluster>,
}

impl<'a> std::iter::Iterator for AllRegistersIter<'a> {
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

/// Mutable iterator over all registers
pub struct AllRegistersIterMut<'a> {
    pub(crate) rem: Vec<&'a mut RegisterCluster>,
}

impl<'a> std::iter::Iterator for AllRegistersIterMut<'a> {
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
