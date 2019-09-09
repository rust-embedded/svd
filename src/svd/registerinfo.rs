#[cfg(feature = "unproven")]
use std::collections::HashMap;

use crate::elementext::ElementExt;
use failure::ResultExt;
use xmltree::Element;

#[cfg(feature = "unproven")]
use crate::encode::Encode;
use crate::error::*;
#[cfg(feature = "unproven")]
use crate::new_element;
use crate::parse;
use crate::types::Parse;

use crate::svd::{
    access::Access, field::Field, modifiedwritevalues::ModifiedWriteValues,
    registerproperties::RegisterProperties, writeconstraint::WriteConstraint,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct RegisterInfo {
    pub name: String,
    pub alternate_group: Option<String>,
    pub alternate_register: Option<String>,
    pub derived_from: Option<String>,
    pub description: Option<String>,
    pub address_offset: u32,
    pub size: Option<u32>,
    pub access: Option<Access>,
    pub reset_value: Option<u32>,
    pub reset_mask: Option<u32>,
    /// `None` indicates that the `<fields>` node is not present
    pub fields: Option<Vec<Field>>,
    pub write_constraint: Option<WriteConstraint>,
    pub modified_write_values: Option<ModifiedWriteValues>,
    // Reserve the right to add more fields to this struct
    pub(crate) _extensible: (),
}

impl Parse for RegisterInfo {
    type Object = RegisterInfo;
    type Error = SVDError;

    fn parse(tree: &Element) -> Result<RegisterInfo, SVDError> {
        let name = tree.get_child_text("name")?;
        RegisterInfo::_parse(tree, name.clone())
            .context(SVDErrorKind::Other(format!("In register `{}`", name)))
            .map_err(|e| e.into())
    }
}

impl RegisterInfo {
    fn _parse(tree: &Element, name: String) -> Result<RegisterInfo, SVDError> {
        let properties = RegisterProperties::parse(tree)?;
        Ok(RegisterInfo {
            name,
            alternate_group: tree.get_child_text_opt("alternateGroup")?,
            alternate_register: tree.get_child_text_opt("alternateRegister")?,
            description: tree.get_child_text_opt("description")?,
            derived_from: tree.attributes.get("derivedFrom").map(|s| s.to_owned()),
            address_offset: tree.get_child_u32("addressOffset")?,
            size: properties.size,
            access: properties.access,
            reset_value: properties.reset_value,
            reset_mask: properties.reset_mask,
            fields: {
                if let Some(fields) = tree.get_child("fields") {
                    let fs: Result<Vec<_>, _> = fields
                        .children
                        .iter()
                        .enumerate()
                        .map(|(e, t)| {
                            Field::parse(t)
                                .context(SVDErrorKind::Other(format!("Parsing field #{}", e)))
                        })
                        .collect();
                    Some(fs?)
                } else {
                    None
                }
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
impl Encode for RegisterInfo {
    type Error = SVDError;
    fn encode(&self) -> Result<Element, SVDError> {
        let mut elem = Element {
            prefix: None,
            namespace: None,
            namespaces: None,
            name: String::from("register"),
            attributes: HashMap::new(),
            children: vec![
                new_element("name", Some(self.name.clone())),
                new_element(
                    "addressOffset",
                    Some(format!("0x{:x}", self.address_offset)),
                ),
            ],
            text: None,
        };
        if let Some(v) = &self.description {
            elem.children
                .push(new_element("description", Some(v.clone())));
        }
        if let Some(v) = &self.alternate_group {
            elem.children
                .push(new_element("alternateGroup", Some(format!("{}", v))));
        }

        if let Some(v) = &self.alternate_register {
            elem.children
                .push(new_element("alternateRegister", Some(format!("{}", v))));
        }

        if let Some(v) = &self.derived_from {
            elem.attributes
                .insert(String::from("derivedFrom"), format!("{}", v));
        }

        if let Some(v) = &self.size {
            elem.children
                .push(new_element("size", Some(format!("{}", v))));
        };

        if let Some(v) = &self.access {
            elem.children.push(v.encode()?);
        };

        if let Some(v) = &self.reset_value {
            elem.children
                .push(new_element("resetValue", Some(format!("0x{:08.x}", v))));
        };

        if let Some(v) = &self.reset_mask {
            elem.children
                .push(new_element("resetMask", Some(format!("0x{:08.x}", v))));
        };

        if let Some(v) = &self.fields {
            let children = v
                .iter()
                .map(Field::encode)
                .collect::<Result<Vec<Element>, SVDError>>()?;
            if !children.is_empty() {
                let fields = Element {
                    prefix: None,
                    namespace: None,
                    namespaces: None,
                    name: String::from("fields"),
                    attributes: HashMap::new(),
                    children,
                    text: None,
                };
                elem.children.push(fields);
            }
        };

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
    use crate::svd::bitrange::*;
    use crate::svd::fieldinfo::FieldInfo;

    #[test]
    fn decode_encode() {
        let tests = vec![(
            RegisterInfo {
                name: String::from("WRITECTRL"),
                alternate_group: Some(String::from("alternate group")),
                alternate_register: Some(String::from("alternate register")),
                derived_from: Some(String::from("derived from")),
                description: Some(String::from("Write Control Register")),
                address_offset: 8,
                size: Some(32),
                access: Some(Access::ReadWrite),
                reset_value: Some(0x00000000),
                reset_mask: Some(0x00000023),
                fields: Some(vec![Field::Single(FieldInfo {
                    name: String::from("WREN"),
                    derived_from: None,
                    description: Some(String::from("Enable Write/Erase Controller")),
                    bit_range: BitRange {
                        offset: 0,
                        width: 1,
                        range_type: BitRangeType::OffsetWidth,
                    },
                    access: Some(Access::ReadWrite),
                    enumerated_values: Vec::new(),
                    write_constraint: None,
                    modified_write_values: None,
                    _extensible: (),
                })]),
                write_constraint: None,
                modified_write_values: Some(ModifiedWriteValues::OneToToggle),
                _extensible: (),
            },
            "
            <register derivedFrom=\"derived from\">
                <name>WRITECTRL</name>
                <description>Write Control Register</description>
                <addressOffset>0x8</addressOffset>
                <alternateGroup>alternate group</alternateGroup>
                <alternateRegister>alternate register</alternateRegister>
                <size>32</size>
                <access>read-write</access>
                <resetValue>0x00000000</resetValue>
                <resetMask>0x00000023</resetMask>
                <fields>
                    <field>
                        <name>WREN</name>
                        <description>Enable Write/Erase Controller</description>
                        <bitOffset>0</bitOffset>
                        <bitWidth>1</bitWidth>
                        <access>read-write</access>
                    </field>
                </fields>
                <modifiedWriteValues>oneToToggle</modifiedWriteValues>
            </register>
            ",
        )];

        run_test::<RegisterInfo>(&tests[..]);
    }
}
