#[cfg(feature = "unproven")]
use std::collections::HashMap;

use crate::elementext::ElementExt;
use xmltree::Element;

#[cfg(feature = "unproven")]
use crate::encode::Encode;
use crate::error::*;
#[cfg(feature = "unproven")]
use crate::new_element;
use crate::parse;
use crate::types::Parse;

use crate::Build;

use crate::svd::{
    access::Access, bitrange::BitRange, enumeratedvalues::EnumeratedValues,
    modifiedwritevalues::ModifiedWriteValues, writeconstraint::WriteConstraint,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct FieldInfo {
    /// Name string used to identify the field.
    /// Field names must be unique within a register
    pub name: String,

    pub bit_range: BitRange,

    /// Specify the field name from which to inherit data.
    /// Elements specified subsequently override inherited values
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub derived_from: Option<String>,

    /// String describing the details of the register
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<String>,

    /// Predefined strings set the access type.
    /// The element can be omitted if access rights get inherited from parent elements
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub access: Option<Access>,

    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub enumerated_values: Vec<EnumeratedValues>,

    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub write_constraint: Option<WriteConstraint>,

    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub modified_write_values: Option<ModifiedWriteValues>,

    // Reserve the right to add more fields to this struct
    #[cfg_attr(feature = "serde", serde(skip))]
    _extensible: (),
}

impl Build for FieldInfo {
    type Builder = FieldInfoBuilder;
}

#[derive(Default)]
pub struct FieldInfoBuilder {
    name: Option<String>,
    bit_range: Option<BitRange>,
    derived_from: Option<String>,
    description: Option<String>,
    access: Option<Access>,
    enumerated_values: Option<Vec<EnumeratedValues>>,
    write_constraint: Option<WriteConstraint>,
    modified_write_values: Option<ModifiedWriteValues>,
}

impl FieldInfoBuilder {
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    pub fn bit_range(mut self, value: BitRange) -> Self {
        self.bit_range = Some(value);
        self
    }
    pub fn derived_from(mut self, value: Option<String>) -> Self {
        self.derived_from = value;
        self
    }
    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }
    pub fn access(mut self, value: Option<Access>) -> Self {
        self.access = value;
        self
    }
    pub fn enumerated_values(mut self, value: Vec<EnumeratedValues>) -> Self {
        self.enumerated_values = Some(value);
        self
    }
    pub fn write_constraint(mut self, value: Option<WriteConstraint>) -> Self {
        self.write_constraint = value;
        self
    }
    pub fn modified_write_values(mut self, value: Option<ModifiedWriteValues>) -> Self {
        self.modified_write_values = value;
        self
    }
    pub fn build(self) -> Result<FieldInfo> {
        (FieldInfo {
            name: self
                .name
                .ok_or_else(|| BuildError::Uninitialized("name".to_string()))?,
            bit_range: self
                .bit_range
                .ok_or_else(|| BuildError::Uninitialized("bit_range".to_string()))?,
            derived_from: self.derived_from,
            description: self.description,
            access: self.access,
            enumerated_values: self.enumerated_values.unwrap_or_default(),
            write_constraint: self.write_constraint,
            modified_write_values: self.modified_write_values,
            _extensible: (),
        })
        .validate()
    }
}

impl FieldInfo {
    fn validate(self) -> Result<Self> {
        check_name(&self.name, "name")?;
        if let Some(name) = self.derived_from.as_ref() {
            check_name(name, "derivedFrom")?;
        }
        for ev in &self.enumerated_values {
            ev.check_range(0..2_u32.pow(self.bit_range.width))?;
        }
        Ok(self)
    }
}

impl Parse for FieldInfo {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        if tree.name != "field" {
            return Err(ParseError::NotExpectedTag(tree.clone(), "field".to_string()).into());
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
            .derived_from(tree.attributes.get("derivedFrom").map(|s| s.to_owned()))
            .description(tree.get_child_text_opt("description")?)
            .bit_range(bit_range)
            .access(parse::optional::<Access>("access", tree)?)
            .enumerated_values({
                let values: Result<Vec<_>, _> = tree
                    .children
                    .iter()
                    .filter(|t| t.name == "enumeratedValues")
                    .map(|t| EnumeratedValues::parse(t))
                    .collect();
                values?
            })
            .write_constraint(parse::optional::<WriteConstraint>("writeConstraint", tree)?)
            .modified_write_values(parse::optional::<ModifiedWriteValues>(
                "modifiedWriteValues",
                tree,
            )?)
            .build()
    }
}

#[cfg(feature = "unproven")]
impl Encode for FieldInfo {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        let mut children = vec![new_element("name", Some(self.name.clone()))];

        if let Some(description) = &self.description {
            children.push(new_element("description", Some(description.clone())))
        }

        let mut elem = Element {
            prefix: None,
            namespace: None,
            namespaces: None,
            name: String::from("field"),
            attributes: HashMap::new(),
            children,
            text: None,
        };

        if let Some(v) = &self.derived_from {
            elem.attributes
                .insert(String::from("derivedFrom"), format!("{}", v));
        }

        // Add bit range
        elem.children.append(&mut self.bit_range.encode()?);

        if let Some(v) = &self.access {
            elem.children.push(v.encode()?);
        };

        let enumerated_values: Result<Vec<Element>> =
            self.enumerated_values.iter().map(|v| v.encode()).collect();
        elem.children.append(&mut enumerated_values?);

        if let Some(v) = &self.write_constraint {
            elem.children.push(v.encode()?);
        };

        if let Some(v) = &self.modified_write_values {
            elem.children.push(v.encode()?);
        };

        Ok(elem)
    }
}

#[cfg(test)]
#[cfg(feature = "unproven")]
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
                  <value>0x00000000</value>
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
