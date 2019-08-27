#[cfg(feature = "unproven")]
use std::collections::HashMap;

use elementext::ElementExt;
use failure::ResultExt;
use xmltree::Element;

#[cfg(feature = "unproven")]
use encode::Encode;
use error::*;
#[cfg(feature = "unproven")]
use new_element;
use parse;
use types::Parse;

use svd::access::Access;
use svd::bitrange::BitRange;
use svd::enumeratedvalues::EnumeratedValues;
use svd::modifiedwritevalues::ModifiedWriteValues;
use svd::writeconstraint::WriteConstraint;

#[cfg(feature = "serde_svd")]
use super::serde::{ Deserialize, Serialize };

#[cfg_attr(feature = "serde_svd", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    pub name: String,
    pub description: Option<String>,
    pub bit_range: BitRange,
    pub access: Option<Access>,
    pub enumerated_values: Vec<EnumeratedValues>,
    pub write_constraint: Option<WriteConstraint>,
    pub modified_write_values: Option<ModifiedWriteValues>,
    // Reserve the right to add more fields to this struct
    pub(crate) _extensible: (),
}

impl Parse for Field {
    type Object = Field;
    type Error = SVDError;
    fn parse(tree: &Element) -> Result<Field, SVDError> {
        if tree.name != "field" {
            return Err(SVDErrorKind::NotExpectedTag(
                tree.clone(),
                format!("field"),
            ).into());
        }
        let name = tree.get_child_text("name")?;
        Field::_parse(tree, name.clone())
            .context(SVDErrorKind::Other(format!(
                "In field `{}`",
                name
            )))
            .map_err(|e| e.into())
    }
}

impl Field {
    fn _parse(tree: &Element, name: String) -> Result<Field, SVDError> {
        Ok(Field {
            name,
            description: tree.get_child_text_opt("description")?,
            bit_range: BitRange::parse(tree)?,
            access: parse::optional::<Access>("access", tree)?,
            enumerated_values: {
                let values: Result<Vec<_>, _> = tree.children
                    .iter()
                    .filter(|t| t.name == "enumeratedValues")
                    .map(EnumeratedValues::parse)
                    .collect();
                values?
            },
            write_constraint: parse::optional::<WriteConstraint>(
                "writeConstraint",
                tree,
            )?,
            modified_write_values: parse::optional::<ModifiedWriteValues>(
                "modifiedWriteValues",
                tree,
            )?,
            _extensible: (),
        })
    }
}

#[cfg(feature = "unproven")]
impl Encode for Field {
    type Error = SVDError;
    fn encode(&self) -> Result<Element, SVDError> {
        let mut children = vec![new_element("name", Some(self.name.clone()))];

        if let Some(ref description) = self.description {
            children.push(new_element("description", Some(description.clone())))
        }

        let mut elem = Element {
            name: String::from("field"),
            attributes: HashMap::new(),
            children,
            text: None,
        };

        // Add bit range
        elem.children
            .append(&mut self.bit_range.encode()?);

        match self.access {
            Some(ref v) => {
                elem.children.push(v.encode()?);
            }
            None => (),
        };

        let enumerated_values: Result<Vec<Element>, SVDError> =
            self.enumerated_values
                .iter()
                .map(|v| v.encode())
                .collect();
        elem.children.append(&mut enumerated_values?);

        match self.write_constraint {
            Some(ref v) => {
                elem.children.push(v.encode()?);
            }
            None => (),
        };

        match self.modified_write_values {
            Some(ref v) => {
                elem.children.push(v.encode()?);
            }
            None => (),
        };

        Ok(elem)
    }
}

#[cfg(test)]
#[cfg(feature = "unproven")]
mod tests {
    use super::*;
    use run_test;
    use svd::bitrange::BitRangeType;
    use svd::enumeratedvalue::EnumeratedValue;

    #[test]
    fn decode_encode() {
        let tests = vec![
            (
                Field {
                    name: String::from("MODE"),
                    description: Some(String::from("Read Mode")),
                    bit_range: BitRange {
                        offset: 24,
                        width: 2,
                        range_type: BitRangeType::OffsetWidth,
                    },
                    access: Some(Access::ReadWrite),
                    enumerated_values: vec![
                        EnumeratedValues {
                            name: None,
                            usage: None,
                            derived_from: None,
                            values: vec![
                                EnumeratedValue {
                                    name: String::from("WS0"),
                                    description: Some(String::from(
                                        "Zero wait-states inserted in fetch or read transfers",
                                    )),
                                    value: Some(0),
                                    is_default: None,
                                    _extensible: (),
                                },
                            ],
                            _extensible: (),
                        },
                    ],
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
            // almost the same test but description info is missing
            (
                Field {
                    name: String::from("MODE"),
                    description: None,
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
        ];

        run_test::<Field>(&tests[..]);
    }
}
