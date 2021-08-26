use super::{BuildError, EmptyToNone, SvdError, ValidateLevel};
use std::borrow::Cow;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
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
    pub fn build(self, lvl: ValidateLevel) -> Result<DimElement, SvdError> {
        let mut de = DimElement {
            dim: self
                .dim
                .ok_or_else(|| BuildError::Uninitialized("dim".to_string()))?,
            dim_increment: self
                .dim_increment
                .ok_or_else(|| BuildError::Uninitialized("dim_increment".to_string()))?,
            dim_index: self.dim_index.empty_to_none(),
        };
        if !lvl.is_disabled() {
            de.validate(lvl)?;
        }
        Ok(de)
    }
}

impl DimElement {
    pub fn builder() -> DimElementBuilder {
        DimElementBuilder::default()
    }
    pub fn modify_from(
        &mut self,
        builder: DimElementBuilder,
        lvl: ValidateLevel,
    ) -> Result<(), SvdError> {
        if let Some(dim) = builder.dim {
            self.dim = dim;
        }
        if let Some(dim_increment) = builder.dim_increment {
            self.dim_increment = dim_increment;
        }
        if builder.dim_index.is_some() {
            self.dim_index = builder.dim_index.empty_to_none();
        }
        if !lvl.is_disabled() {
            self.validate(lvl)
        } else {
            Ok(())
        }
    }
    pub fn validate(&mut self, _lvl: ValidateLevel) -> Result<(), SvdError> {
        // TODO
        Ok(())
    }
    pub fn indexes(&self) -> Indexes {
        Indexes {
            i: 0,
            dim: self.dim,
            dim_index: &self.dim_index,
        }
    }
}

pub struct Indexes<'a> {
    i: u32,
    dim: u32,
    dim_index: &'a Option<Vec<String>>,
}

impl<'a> Iterator for Indexes<'a> {
    type Item = Cow<'a, str>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.dim {
            return None;
        }
        let i = self.i;
        self.i += 1;
        if let Some(index) = self.dim_index.as_ref() {
            Some(index[i as usize].as_str().into())
        } else {
            Some(i.to_string().into())
        }
    }
}
