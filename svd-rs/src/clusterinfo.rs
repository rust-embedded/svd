use super::{
    registercluster::{
        AllRegistersIter, AllRegistersIterMut, ClusterIter, ClusterIterMut, RegisterIter,
        RegisterIterMut,
    },
    BuildError, Cluster, DimElement, EmptyToNone, Register, RegisterCluster, RegisterProperties,
    SvdError, ValidateLevel,
};

/// Errors from [`ClusterInfo::validate`]
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// The cluster can not be empty
    #[error("Cluster must contain at least one Register or Cluster")]
    EmptyCluster,
}

/// Description of a cluster
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "camelCase")
)]
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ClusterInfo {
    /// String to identify the cluster.
    /// Cluster names are required to be unique within the scope of a peripheral
    pub name: String,

    /// String describing the details of the register cluster
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub description: Option<String>,

    /// Specify the name of the original cluster if this cluster provides an alternative description
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub alternate_cluster: Option<String>,

    /// Specify the struct type name created in the device header file
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub header_struct_name: Option<String>,

    /// Cluster address relative to the `baseAddress` of the peripheral
    pub address_offset: u32,

    /// Default properties for all registers
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub default_register_properties: RegisterProperties,

    /// Children/members of the cluster
    pub children: Vec<RegisterCluster>,

    /// Specify the cluster name from which to inherit data.
    /// Elements specified subsequently override inherited values
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub derived_from: Option<String>,
}

/// Builder for [`ClusterInfo`]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClusterInfoBuilder {
    name: Option<String>,
    description: Option<String>,
    alternate_cluster: Option<String>,
    header_struct_name: Option<String>,
    address_offset: Option<u32>,
    default_register_properties: RegisterProperties,
    children: Option<Vec<RegisterCluster>>,
    derived_from: Option<String>,
}

impl From<ClusterInfo> for ClusterInfoBuilder {
    fn from(c: ClusterInfo) -> Self {
        Self {
            name: Some(c.name),
            description: c.description,
            alternate_cluster: c.alternate_cluster,
            header_struct_name: c.header_struct_name,
            address_offset: Some(c.address_offset),
            default_register_properties: c.default_register_properties,
            children: Some(c.children),
            derived_from: c.derived_from,
        }
    }
}

impl ClusterInfoBuilder {
    /// Set the name of the cluster.
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    /// Set the description of the cluster.
    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }
    /// Set the alternate cluster.
    pub fn alternate_cluster(mut self, value: Option<String>) -> Self {
        self.alternate_cluster = value;
        self
    }
    /// Set the struct type name of the cluster. If not specified, the name of the cluster should be used.
    pub fn header_struct_name(mut self, value: Option<String>) -> Self {
        self.header_struct_name = value;
        self
    }
    /// Set the address_offset of the cluster, relative to the [`baseAddress`](crate::Peripheral::base_address) of the peripheral.
    pub fn address_offset(mut self, value: u32) -> Self {
        self.address_offset = Some(value);
        self
    }
    /// Set the default_register_properties of the cluster.
    pub fn default_register_properties(mut self, value: RegisterProperties) -> Self {
        self.default_register_properties = value;
        self
    }
    /// Set the children of the cluster.
    pub fn children(mut self, value: Vec<RegisterCluster>) -> Self {
        self.children = Some(value);
        self
    }
    /// Set the derived_from of the cluster.
    pub fn derived_from(mut self, value: Option<String>) -> Self {
        self.derived_from = value;
        self
    }
    /// Validate and build a [`ClusterInfo`].
    pub fn build(self, lvl: ValidateLevel) -> Result<ClusterInfo, SvdError> {
        let mut cluster = ClusterInfo {
            name: self
                .name
                .ok_or_else(|| BuildError::Uninitialized("name".to_string()))?,
            description: self.description.empty_to_none(),
            alternate_cluster: self.alternate_cluster.empty_to_none(),
            header_struct_name: self.header_struct_name.empty_to_none(),
            address_offset: self
                .address_offset
                .ok_or_else(|| BuildError::Uninitialized("address_offset".to_string()))?,
            default_register_properties: self.default_register_properties.build(lvl)?,
            children: self
                .children
                .ok_or_else(|| BuildError::Uninitialized("children".to_string()))?,
            derived_from: self.derived_from,
        };
        if !lvl.is_disabled() {
            cluster.validate(lvl)?;
        }
        Ok(cluster)
    }
}

impl ClusterInfo {
    /// Make a builder for [`ClusterInfo`]
    pub fn builder() -> ClusterInfoBuilder {
        ClusterInfoBuilder::default()
    }
    /// Construct single [`Cluster`]
    pub const fn single(self) -> Cluster {
        Cluster::Single(self)
    }
    /// Construct [`Cluster`] array
    pub const fn array(self, dim: DimElement) -> Cluster {
        Cluster::Array(self, dim)
    }
    /// Modify an existing [`ClusterInfo`] based on a [builder](ClusterInfoBuilder).
    pub fn modify_from(
        &mut self,
        builder: ClusterInfoBuilder,
        lvl: ValidateLevel,
    ) -> Result<(), SvdError> {
        if let Some(name) = builder.name {
            self.name = name;
        }
        if builder.description.is_some() {
            self.description = builder.description.empty_to_none();
        }
        if builder.alternate_cluster.is_some() {
            self.alternate_cluster = builder.alternate_cluster.empty_to_none();
        }
        if builder.header_struct_name.is_some() {
            self.header_struct_name = builder.header_struct_name.empty_to_none();
        }
        if let Some(address_offset) = builder.address_offset {
            self.address_offset = address_offset;
        }
        if builder.derived_from.is_some() {
            self.derived_from = builder.derived_from;
            self.children = Vec::new();
            self.default_register_properties = RegisterProperties::default();
        } else {
            self.default_register_properties
                .modify_from(builder.default_register_properties, lvl)?;
            if let Some(children) = builder.children {
                self.children = children;
            }
        }
        if !lvl.is_disabled() {
            self.validate(lvl)
        } else {
            Ok(())
        }
    }

    /// Validate the [`ClusterInfo`]
    pub fn validate(&mut self, lvl: ValidateLevel) -> Result<(), SvdError> {
        if lvl.is_strict() {
            super::check_dimable_name(&self.name, "name")?;
        }
        if let Some(name) = self.derived_from.as_ref() {
            if lvl.is_strict() {
                super::check_derived_name(name, "derivedFrom")?;
            }
        } else if self.children.is_empty() && lvl.is_strict() {
            return Err(Error::EmptyCluster.into());
        }
        Ok(())
    }

    /// Returns iterator over all descendant registers
    #[deprecated(since = "0.12.1", note = "Please use `all_registers` instead")]
    pub fn reg_iter(&self) -> AllRegistersIter {
        self.all_registers()
    }

    /// Returns iterator over all descendant registers
    pub fn all_registers(&self) -> AllRegistersIter {
        let mut rem: Vec<&RegisterCluster> = Vec::with_capacity(self.children.len());
        for r in self.children.iter().rev() {
            rem.push(r);
        }
        AllRegistersIter { rem }
    }

    /// Returns mutable iterator over all descendant registers
    #[deprecated(since = "0.12.1", note = "Please use `all_registers_mut` instead")]
    pub fn reg_iter_mut(&mut self) -> AllRegistersIterMut {
        self.all_registers_mut()
    }

    /// Returns mutable iterator over all descendant registers
    pub fn all_registers_mut(&mut self) -> AllRegistersIterMut {
        let mut rem: Vec<&mut RegisterCluster> = Vec::with_capacity(self.children.len());
        for r in self.children.iter_mut().rev() {
            rem.push(r);
        }
        AllRegistersIterMut { rem }
    }

    /// Returns iterator over child registers
    pub fn registers(&self) -> RegisterIter {
        RegisterIter {
            all: self.children.iter(),
        }
    }

    /// Returns mutable iterator over child registers
    pub fn registers_mut(&mut self) -> RegisterIterMut {
        RegisterIterMut {
            all: self.children.iter_mut(),
        }
    }

    /// Returns iterator over child clusters
    pub fn clusters(&self) -> ClusterIter {
        ClusterIter {
            all: self.children.iter(),
        }
    }

    /// Returns mutable iterator over child clusters
    pub fn clusters_mut(&mut self) -> ClusterIterMut {
        ClusterIterMut {
            all: self.children.iter_mut(),
        }
    }

    /// Get register by name
    pub fn get_register(&self, name: &str) -> Option<&Register> {
        self.registers().find(|f| f.name == name)
    }

    /// Get mutable register by name
    pub fn get_mut_register(&mut self, name: &str) -> Option<&mut Register> {
        self.registers_mut().find(|f| f.name == name)
    }

    /// Get cluster by name
    pub fn get_cluster(&self, name: &str) -> Option<&Cluster> {
        self.clusters().find(|f| f.name == name)
    }

    /// Get mutable cluster by name
    pub fn get_mut_cluster(&mut self, name: &str) -> Option<&mut Cluster> {
        self.clusters_mut().find(|f| f.name == name)
    }
}
