use std::collections::HashMap;

use crate::elementext::ElementExt;
use xmltree::Element;

use crate::encode::Encode;
use crate::error::*;

use crate::new_element;
use crate::parse;
use crate::types::Parse;

use crate::svd::{
    access::Access, field::Field, modifiedwritevalues::ModifiedWriteValues,
    registerproperties::RegisterProperties, writeconstraint::WriteConstraint,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct RegisterInfo {
    /// String to identify the register.
    /// Register names are required to be unique within the scope of a peripheral
    pub name: String,

    /// Define the address offset relative to the enclosing element
    pub address_offset: u32,

    /// Specifies a group name associated with all alternate register that have the same name
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub alternate_group: Option<String>,

    /// This tag can reference a register that has been defined above to
    /// current location in the description and that describes the memory location already
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub alternate_register: Option<String>,

    /// Specify the register name from which to inherit data.
    /// Elements specified subsequently override inherited values
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub derived_from: Option<String>,

    /// String describing the details of the register
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<String>,

    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub display_name: Option<String>,

    #[cfg_attr(feature = "serde", serde(flatten))]
    pub properties: RegisterProperties,

    /// `None` indicates that the `<fields>` node is not present
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub fields: Option<Vec<Field>>,

    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub write_constraint: Option<WriteConstraint>,

    /// Element to describe the manipulation of data written to a register
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub modified_write_values: Option<ModifiedWriteValues>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RegisterInfoBuilder {
    name: Option<String>,
    address_offset: Option<u32>,
    alternate_group: Option<String>,
    alternate_register: Option<String>,
    derived_from: Option<String>,
    description: Option<String>,
    display_name: Option<String>,
    properties: RegisterProperties,
    fields: Option<Vec<Field>>,
    write_constraint: Option<WriteConstraint>,
    modified_write_values: Option<ModifiedWriteValues>,
}

impl From<RegisterInfo> for RegisterInfoBuilder {
    fn from(r: RegisterInfo) -> Self {
        Self {
            name: Some(r.name),
            address_offset: Some(r.address_offset),
            alternate_group: r.alternate_group,
            alternate_register: r.alternate_register,
            derived_from: r.derived_from,
            display_name: r.display_name,
            description: r.description,
            properties: r.properties,
            fields: r.fields,
            write_constraint: r.write_constraint,
            modified_write_values: r.modified_write_values,
        }
    }
}

impl RegisterInfoBuilder {
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    pub fn address_offset(mut self, value: u32) -> Self {
        self.address_offset = Some(value);
        self
    }
    pub fn alternate_group(mut self, value: Option<String>) -> Self {
        self.alternate_group = value;
        self
    }
    pub fn alternate_register(mut self, value: Option<String>) -> Self {
        self.alternate_register = value;
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
    pub fn display_name(mut self, value: Option<String>) -> Self {
        self.display_name = value;
        self
    }
    pub fn properties(mut self, value: RegisterProperties) -> Self {
        self.properties = value;
        self
    }
    pub fn size(mut self, value: Option<u32>) -> Self {
        self.properties.size = value;
        self
    }
    pub fn access(mut self, value: Option<Access>) -> Self {
        self.properties.access = value;
        self
    }
    pub fn reset_value(mut self, value: Option<u64>) -> Self {
        self.properties.reset_value = value;
        self
    }
    pub fn reset_mask(mut self, value: Option<u64>) -> Self {
        self.properties.reset_mask = value;
        self
    }
    pub fn fields(mut self, value: Option<Vec<Field>>) -> Self {
        self.fields = value;
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
    pub fn build(self) -> Result<RegisterInfo> {
        (RegisterInfo {
            name: self
                .name
                .ok_or_else(|| BuildError::Uninitialized("name".to_string()))?,
            address_offset: self
                .address_offset
                .ok_or_else(|| BuildError::Uninitialized("address_offset".to_string()))?,
            alternate_group: self.alternate_group,
            alternate_register: self.alternate_register,
            derived_from: self.derived_from,
            description: self.description,
            display_name: self.display_name,
            properties: self.properties,
            fields: self.fields,
            write_constraint: self.write_constraint,
            modified_write_values: self.modified_write_values,
        })
        .validate()
    }
}

impl RegisterInfo {
    #[allow(clippy::unnecessary_wraps)]
    fn validate(self) -> Result<Self> {
        #[cfg(feature = "strict")]
        check_dimable_name(&self.name, "name")?;
        #[cfg(feature = "strict")]
        {
            if let Some(name) = self.alternate_group.as_ref() {
                check_name(name, "alternateGroup")?;
            }
            if let Some(name) = self.alternate_register.as_ref() {
                check_dimable_name(name, "alternateRegister")?;
            }
        }
        if let Some(_name) = self.derived_from.as_ref() {
            #[cfg(feature = "strict")]
            check_derived_name(_name, "derivedFrom")?;
        } else if let Some(fields) = self.fields.as_ref() {
            if fields.is_empty() {
                #[cfg(feature = "strict")]
                return Err(SVDError::EmptyFields)?;
            }
        }
        Ok(self)
    }
}

impl Parse for RegisterInfo {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        let name = tree.get_child_text("name")?;
        Self::_parse(tree, name.clone()).with_context(|| format!("In register `{}`", name))
    }
}

impl RegisterInfo {
    fn _parse(tree: &Element, name: String) -> Result<Self> {
        RegisterInfoBuilder::default()
            .name(name)
            .alternate_group(tree.get_child_text_opt("alternateGroup")?)
            .alternate_register(tree.get_child_text_opt("alternateRegister")?)
            .description(tree.get_child_text_opt("description")?)
            .display_name(tree.get_child_text_opt("displayName")?)
            .derived_from(tree.attributes.get("derivedFrom").map(|s| s.to_owned()))
            .address_offset(tree.get_child_u32("addressOffset")?)
            .properties(RegisterProperties::parse(tree)?)
            .fields({
                if let Some(fields) = tree.get_child("fields") {
                    let fs: Result<Vec<_>, _> = fields
                        .children
                        .iter()
                        .enumerate()
                        .map(|(e, t)| {
                            Field::parse(t).with_context(|| format!("Parsing field #{}", e))
                        })
                        .collect();
                    Some(fs?)
                } else {
                    None
                }
            })
            .write_constraint(parse::optional::<WriteConstraint>("writeConstraint", tree)?)
            .modified_write_values(parse::optional::<ModifiedWriteValues>(
                "modifiedWriteValues",
                tree,
            )?)
            .build()
    }
}

impl Encode for RegisterInfo {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
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
        if let Some(v) = &self.display_name {
            elem.children
                .push(new_element("displayName", Some(v.clone())));
        }
        if let Some(v) = &self.alternate_group {
            elem.children
                .push(new_element("alternateGroup", Some(v.to_string())));
        }

        if let Some(v) = &self.alternate_register {
            elem.children
                .push(new_element("alternateRegister", Some(v.to_string())));
        }

        if let Some(v) = &self.derived_from {
            elem.attributes
                .insert(String::from("derivedFrom"), v.to_string());
        }

        if let Some(v) = &self.properties.size {
            elem.children.push(new_element("size", Some(v.to_string())));
        };

        if let Some(v) = &self.properties.access {
            elem.children.push(v.encode()?);
        };

        if let Some(v) = &self.properties.reset_value {
            elem.children
                .push(new_element("resetValue", Some(format!("0x{:08.x}", v))));
        };

        if let Some(v) = &self.properties.reset_mask {
            elem.children
                .push(new_element("resetMask", Some(format!("0x{:08.x}", v))));
        };

        if let Some(v) = &self.fields {
            let children = v
                .iter()
                .map(Field::encode)
                .collect::<Result<Vec<Element>>>()?;
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
mod tests {
    use super::*;
    use crate::run_test;
    use crate::svd::bitrange::*;
    use crate::svd::fieldinfo::FieldInfoBuilder;

    #[test]
    fn decode_encode() {
        let tests = vec![(
            RegisterInfoBuilder::default()
                .name("WRITECTRL".to_string())
                .alternate_group(Some("alternate_group".to_string()))
                .alternate_register(Some("alternate_register".to_string()))
                .derived_from(Some("derived_from".to_string()))
                .description(Some("Write Control Register".to_string()))
                .address_offset(8)
                .size(Some(32))
                .access(Some(Access::ReadWrite))
                .reset_value(Some(0x00000000))
                .reset_mask(Some(0x00000023))
                .fields(Some(vec![Field::Single(
                    FieldInfoBuilder::default()
                        .name("WREN".to_string())
                        .description(Some("Enable Write/Erase Controller".to_string()))
                        .bit_range(BitRange {
                            offset: 0,
                            width: 1,
                            range_type: BitRangeType::OffsetWidth,
                        })
                        .access(Some(Access::ReadWrite))
                        .build()
                        .unwrap(),
                )]))
                .modified_write_values(Some(ModifiedWriteValues::OneToToggle))
                .build()
                .unwrap(),
            "
            <register derivedFrom=\"derived_from\">
                <name>WRITECTRL</name>
                <description>Write Control Register</description>
                <addressOffset>0x8</addressOffset>
                <alternateGroup>alternate_group</alternateGroup>
                <alternateRegister>alternate_register</alternateRegister>
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
