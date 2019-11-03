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
#[derive(Clone, Debug, PartialEq, derive_builder::Builder)]
pub struct FieldInfo {
    /// Name string used to identify the field.
    /// Field names must be unique within a register
    pub name: String,

    /// Specify the field name from which to inherit data.
    /// Elements specified subsequently override inherited values
    #[builder(default)]
    pub derived_from: Option<String>,

    /// String describing the details of the register
    #[builder(default)]
    pub description: Option<String>,

    pub bit_range: BitRange,

    /// Predefined strings set the access type.
    /// The element can be omitted if access rights get inherited from parent elements
    #[builder(default)]
    pub access: Option<Access>,

    #[builder(default)]
    pub enumerated_values: Vec<EnumeratedValues>,

    #[builder(default)]
    pub write_constraint: Option<WriteConstraint>,

    #[builder(default)]
    pub modified_write_values: Option<ModifiedWriteValues>,

    // Reserve the right to add more fields to this struct
    #[builder(default)]
    _extensible: (),
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
        FieldInfoBuilder::default()
            .name(name)
            .derived_from(tree.attributes.get("derivedFrom").map(|s| s.to_owned()))
            .description(tree.get_child_text_opt("description")?)
            .bit_range(BitRange::parse(tree)?)
            .access(parse::optional::<Access>("access", tree)?)
            .enumerated_values({
                let values: Result<Vec<_>, _> = tree
                    .children
                    .iter()
                    .filter(|t| t.name == "enumeratedValues")
                    .map(EnumeratedValues::parse)
                    .collect();
                values?
            })
            .write_constraint(parse::optional::<WriteConstraint>("writeConstraint", tree)?)
            .modified_write_values(parse::optional::<ModifiedWriteValues>(
                "modifiedWriteValues",
                tree,
            )?)
            .build()
            .map_err(|e| anyhow::anyhow!(e))
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
                    .derived_from(Some("other field".to_string()))
                    .bit_range(BitRange {
                        offset: 24,
                        width: 2,
                        range_type: BitRangeType::OffsetWidth,
                    })
                    .build()
                    .unwrap(),
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
