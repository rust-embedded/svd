use super::{Cluster, Register};

/// A [cluster](crate::Cluster) or a [register](crate::Register)
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[derive(Clone, Debug, PartialEq)]
pub enum RegisterCluster {
    /// Register
    Register(Register),
    /// Cluster
    Cluster(Cluster),
}

impl From<Register> for RegisterCluster {
    fn from(reg: Register) -> Self {
        RegisterCluster::Register(reg)
    }
}

impl From<Cluster> for RegisterCluster {
    fn from(cluser: Cluster) -> Self {
        RegisterCluster::Cluster(cluser)
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
