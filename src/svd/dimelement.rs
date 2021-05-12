use super::{BuildError, SvdError};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct DimElement {
    /// Defines the number of elements in an array or list
    pub dim: u32,

    /// Specify the address increment between two neighboring array or list members in the address map
    pub dim_increment: u32,

    /// Specify the strings that substitue the placeholder `%s` within `name` and `displayName`.
    /// By default, <dimIndex> is a value starting with 0
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub dim_index: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DimElementBuilder {
    dim: Option<u32>,
    dim_increment: Option<u32>,
    dim_index: Option<Vec<String>>,
}

impl From<DimElement> for DimElementBuilder {
    fn from(d: DimElement) -> Self {
        Self {
            dim: Some(d.dim),
            dim_increment: Some(d.dim_increment),
            dim_index: d.dim_index,
        }
    }
}

impl DimElementBuilder {
    pub fn dim(mut self, value: u32) -> Self {
        self.dim = Some(value);
        self
    }
    pub fn dim_increment(mut self, value: u32) -> Self {
        self.dim_increment = Some(value);
        self
    }
    pub fn dim_index(mut self, value: Option<Vec<String>>) -> Self {
        self.dim_index = value;
        self
    }
    pub fn build(self) -> Result<DimElement, SvdError> {
        Ok(DimElement {
            dim: self
                .dim
                .ok_or_else(|| BuildError::Uninitialized("dim".to_string()))?,
            dim_increment: self
                .dim_increment
                .ok_or_else(|| BuildError::Uninitialized("dim_increment".to_string()))?,
            dim_index: self.dim_index,
        })
    }
}

impl DimElement {
    pub fn builder() -> DimElementBuilder {
        DimElementBuilder::default()
    }
}
