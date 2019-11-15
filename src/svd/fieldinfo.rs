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

use crate::svd::{
    access::Access, bitrange::BitRange, enumeratedvalues::EnumeratedValues,
    modifiedwritevalues::ModifiedWriteValues, writeconstraint::WriteConstraint,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct FieldInfo {
    pub name: String,
    pub derived_from: Option<String>,
    pub description: Option<String>,
    pub bit_range: BitRange,
    pub access: Option<Access>,
    pub enumerated_values: Vec<EnumeratedValues>,
    pub write_constraint: Option<WriteConstraint>,
    pub modified_write_values: Option<ModifiedWriteValues>,
    // Reserve the right to add more fields to this struct
    pub(crate) _extensible: (),
}

impl Parse for FieldInfo {
    type Object = FieldInfo;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<FieldInfo> {
        if tree.name != "field" {
            return Err(ParseError::NotExpectedTag(tree.clone(), "field".to_string()).into());
        }
        let name = tree.get_child_text("name")?;
        FieldInfo::_parse(tree, name.clone()).context(format!("In field `{}`", name))
    }
}

impl FieldInfo {
    fn _parse(tree: &Element, name: String) -> Result<FieldInfo> {
        Ok(FieldInfo {
            name,
            derived_from: tree.attributes.get("derivedFrom").map(|s| s.to_owned()),
            description: tree.get_child_text_opt("description")?,
            bit_range: BitRange::parse(tree)?,
            access: parse::optional::<Access>("access", tree)?,
            enumerated_values: {
                let values: Result<Vec<_>, _> = tree
                    .children
                    .iter()
                    .filter(|t| t.name == "enumeratedValues")
                    .map(EnumeratedValues::parse)
                    .collect();
                values?
            },
            write_constraint: parse::optional::<WriteConstraint>("writeConstraint", tree)?,
            modified_write_values: parse::optional::<ModifiedWriteValues>(
                "modifiedWriteValues",
                tree,
            )?,
            _extensible: (),
        })
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
    use crate::svd::{bitrange::BitRangeType, enumeratedvalue::EnumeratedValue};

    #[test]
    fn decode_encode() {
        let tests = vec![
            (
                FieldInfo {
                    name: String::from("MODE"),
                    derived_from: None,
                    description: Some(String::from("Read Mode")),
                    bit_range: BitRange {
                        offset: 24,
                        width: 2,
                        range_type: BitRangeType::OffsetWidth,
                    },
                    access: Some(Access::ReadWrite),
                    enumerated_values: vec![EnumeratedValues {
                        name: None,
                        usage: None,
                        derived_from: None,
                        values: vec![EnumeratedValue {
                            name: String::from("WS0"),
                            description: Some(String::from(
                                "Zero wait-states inserted in fetch or read transfers",
                            )),
                            value: Some(0),
                            is_default: None,
                            _extensible: (),
                        }],
                        _extensible: (),
                    }],
                    write_constraint: None,
                    modified_write_values: None,
                    _extensible: (),
                },
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
                FieldInfo {
                    name: String::from("MODE"),
                    derived_from: Some(String::from("other field")),
                    description: None,
                    bit_range: BitRange {
                        offset: 24,
                        width: 2,
                        range_type: BitRangeType::OffsetWidth,
                    },
                    access: None,
                    enumerated_values: vec![],
                    write_constraint: None,
                    modified_write_values: None,
                    _extensible: (),
                },
                "
            <field derivedFrom=\"other field\">
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
