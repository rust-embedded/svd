use crate::elementext::ElementExt;
use xmltree::Element;

use crate::encode::Encode;
use crate::error::*;

use crate::new_element;
use crate::parse;
use crate::types::Parse;

use crate::svd::{
    access::Access, bitrange::BitRange, enumeratedvalues::EnumeratedValues,
    modifiedwritevalues::ModifiedWriteValues, writeconstraint::WriteConstraint,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct FieldInfo {
    /// Name string used to identify the field.
    /// Field names must be unique within a register
    pub name: String,

    /// String describing the details of the register
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<String>,

    /// Bit position of the field within the register
    pub bit_range: BitRange,

    /// Predefined strings set the access type.
    /// The element can be omitted if access rights get inherited from parent elements
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub access: Option<Access>,

    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub modified_write_values: Option<ModifiedWriteValues>,

    /// Specifies the subset of allowed write values
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub write_constraint: Option<WriteConstraint>,

    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub enumerated_values: Vec<EnumeratedValues>,

    /// Specify the field name from which to inherit data.
    /// Elements specified subsequently override inherited values
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub derived_from: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FieldInfoBuilder {
    name: Option<String>,
    description: Option<String>,
    bit_range: Option<BitRange>,
    access: Option<Access>,
    modified_write_values: Option<ModifiedWriteValues>,
    write_constraint: Option<WriteConstraint>,
    enumerated_values: Option<Vec<EnumeratedValues>>,
    derived_from: Option<String>,
}

impl From<FieldInfo> for FieldInfoBuilder {
    fn from(f: FieldInfo) -> Self {
        Self {
            name: Some(f.name),
            description: f.description,
            bit_range: Some(f.bit_range),
            access: f.access,
            modified_write_values: f.modified_write_values,
            write_constraint: f.write_constraint,
            enumerated_values: Some(f.enumerated_values),
            derived_from: f.derived_from,
        }
    }
}

impl FieldInfoBuilder {
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }
    pub fn bit_range(mut self, value: BitRange) -> Self {
        self.bit_range = Some(value);
        self
    }
    pub fn access(mut self, value: Option<Access>) -> Self {
        self.access = value;
        self
    }
    pub fn modified_write_values(mut self, value: Option<ModifiedWriteValues>) -> Self {
        self.modified_write_values = value;
        self
    }
    pub fn write_constraint(mut self, value: Option<WriteConstraint>) -> Self {
        self.write_constraint = value;
        self
    }
    pub fn enumerated_values(mut self, value: Vec<EnumeratedValues>) -> Self {
        self.enumerated_values = Some(value);
        self
    }
    pub fn derived_from(mut self, value: Option<String>) -> Self {
        self.derived_from = value;
        self
    }
    pub fn build(self) -> Result<FieldInfo> {
        (FieldInfo {
            name: self
                .name
                .ok_or_else(|| BuildError::Uninitialized("name".to_string()))?,
            description: self.description,
            bit_range: self
                .bit_range
                .ok_or_else(|| BuildError::Uninitialized("bit_range".to_string()))?,
            access: self.access,
            modified_write_values: self.modified_write_values,
            write_constraint: self.write_constraint,
            enumerated_values: self.enumerated_values.unwrap_or_default(),
            derived_from: self.derived_from,
        })
        .validate()
    }
}

impl FieldInfo {
    fn validate(self) -> Result<Self> {
        #[cfg(feature = "strict")]
        check_dimable_name(&self.name, "name")?;
        #[cfg(feature = "strict")]
        {
            if let Some(name) = self.derived_from.as_ref() {
                check_derived_name(name, "derivedFrom")?;
            }
        }

        if self.bit_range.width == 0 {
            anyhow::bail!("bitRange width of 0 does not make sense");
        }

        // If the bit_range has its maximum width, all enumerated values will of
        // course fit in so we can skip validation.
        if self.bit_range.width < 64 {
            for ev in &self.enumerated_values {
                ev.check_range(0..2_u64.pow(self.bit_range.width))?;
            }
        }
        Ok(self)
    }
}

impl Parse for FieldInfo {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        if tree.name != "field" {
            return Err(SVDError::NotExpectedTag(tree.clone(), "field".to_string()).into());
        }
        let name = tree.get_child_text("name")?;
        Self::_parse(tree, name.clone()).with_context(|| format!("In field `{}`", name))
    }
}

impl FieldInfo {
    fn _parse(tree: &Element, name: String) -> Result<Self> {
        let bit_range = BitRange::parse(tree)?;
        FieldInfoBuilder::default()
            .name(name)
            .description(tree.get_child_text_opt("description")?)
            .bit_range(bit_range)
            .access(parse::optional::<Access>("access", tree)?)
            .modified_write_values(parse::optional::<ModifiedWriteValues>(
                "modifiedWriteValues",
                tree,
            )?)
            .write_constraint(parse::optional::<WriteConstraint>("writeConstraint", tree)?)
            .enumerated_values({
                let values: Result<Vec<_>, _> = tree
                    .children
                    .iter()
                    .filter(|t| t.name == "enumeratedValues")
                    .map(|t| EnumeratedValues::parse(t))
                    .collect();
                values?
            })
            .derived_from(tree.attributes.get("derivedFrom").map(|s| s.to_owned()))
            .build()
    }
}

impl Encode for FieldInfo {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        let mut elem = new_element("field", None);
        elem.children
            .push(new_element("name", Some(self.name.clone())));

        if let Some(description) = &self.description {
            elem.children
                .push(new_element("description", Some(description.clone())))
        }

        // Add bit range
        elem.children.append(&mut self.bit_range.encode()?);

        if let Some(v) = &self.access {
            elem.children.push(v.encode()?);
        }

        if let Some(v) = &self.modified_write_values {
            elem.children.push(v.encode()?);
        }

        if let Some(v) = &self.write_constraint {
            elem.children.push(v.encode()?);
        }

        let enumerated_values: Result<Vec<Element>> =
            self.enumerated_values.iter().map(|v| v.encode()).collect();
        elem.children.append(&mut enumerated_values?);

        if let Some(v) = &self.derived_from {
            elem.attributes
                .insert(String::from("derivedFrom"), v.to_string());
        }

        Ok(elem)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run_test;
    use crate::svd::{
        bitrange::BitRangeType, enumeratedvalue::EnumeratedValueBuilder,
        enumeratedvalues::EnumeratedValuesBuilder,
    };

    #[test]
    fn decode_encode() {
        let tests = vec![
            (
                FieldInfoBuilder::default()
                    .name("MODE".to_string())
                    .description(Some("Read Mode".to_string()))
                    .bit_range(BitRange {
                        offset: 24,
                        width: 2,
                        range_type: BitRangeType::OffsetWidth,
                    })
                    .access(Some(Access::ReadWrite))
                    .enumerated_values(vec![EnumeratedValuesBuilder::default()
                        .values(vec![EnumeratedValueBuilder::default()
                            .name("WS0".to_string())
                            .description(Some(
                                "Zero wait-states inserted in fetch or read transfers".to_string(),
                            ))
                            .value(Some(0))
                            .is_default(None)
                            .build()
                            .unwrap()])
                        .build()
                        .unwrap()])
                    .build()
                    .unwrap(),
                "
            <field>
              <name>MODE</name>
              <description>Read Mode</description>
              <bitOffset>24</bitOffset>
              <bitWidth>2</bitWidth>
              <access>read-write</access>
              <enumeratedValues>
                <enumeratedValue>
                  <name>WS0</name>
                  <description>Zero wait-states inserted in fetch or read transfers</description>
                  <value>0</value>
                </enumeratedValue>
              </enumeratedValues>
            </field>
            ",
            ),
            (
                FieldInfoBuilder::default()
                    .name("MODE".to_string())
                    .derived_from(Some("other_field".to_string()))
                    .bit_range(BitRange {
                        offset: 24,
                        width: 2,
                        range_type: BitRangeType::OffsetWidth,
                    })
                    .build()
                    .unwrap(),
                "
            <field derivedFrom=\"other_field\">
              <name>MODE</name>
              <bitOffset>24</bitOffset>
              <bitWidth>2</bitWidth>
            </field>
            ",
            ),
        ];

        run_test::<FieldInfo>(&tests[..]);
    }
}
