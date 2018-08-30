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
use svd::field::Field;
use svd::modifiedwritevalues::ModifiedWriteValues;
use svd::writeconstraint::WriteConstraint;

#[derive(Clone, Debug, PartialEq)]
pub struct RegisterInfo {
    pub name: String,
    pub alternate_group: Option<String>,
    pub alternate_register: Option<String>,
    pub derived_from: Option<String>,
    pub description: String,
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
            .context(SVDErrorKind::Other(format!(
                "In register `{}`",
                name
            )))
            .map_err(|e| e.into())
    }
}

impl RegisterInfo {
    fn _parse(tree: &Element, name: String) -> Result<RegisterInfo, SVDError> {
        Ok(RegisterInfo {
            name,
            alternate_group: tree.get_child_text_opt("alternateGroup")?,
            alternate_register: tree.get_child_text_opt("alternateRegister")?,
            derived_from: tree.get_child_text_opt("derivedFrom")?,
            description: tree.get_child_text("description")?,
            address_offset: tree.get_child_u32("addressOffset")?,
            size: parse::optional::<u32>("size", tree)?,
            access: parse::optional::<Access>("access", tree)?,
            reset_value: parse::optional::<u32>("resetValue", tree)?,
            reset_mask: parse::optional::<u32>("resetMask", tree)?,
            fields: {
                if let Some(fields) = tree.get_child("fields") {
                    let fs: Result<Vec<_>, _> = fields
                        .children
                        .iter()
                        .enumerate()
                        .map(|(e, t)| {
                            Field::parse(t).context(SVDErrorKind::Other(
                                format!("Parsing field #{}", e).into(),
                            ))
                        })
                        .collect();
                    Some(fs?)
                } else {
                    None
                }
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
impl Encode for RegisterInfo {
    type Error = SVDError;
    fn encode(&self) -> Result<Element, SVDError> {
        let mut elem = Element {
            name: String::from("register"),
            attributes: HashMap::new(),
            children: vec![
                new_element("name", Some(self.name.clone())),
                new_element("description", Some(self.description.clone())),
                new_element(
                    "addressOffset",
                    Some(format!("0x{:x}", self.address_offset)),
                ),
            ],
            text: None,
        };

        match self.alternate_group {
            Some(ref v) => {
                elem.children.push(new_element(
                    "alternateGroup",
                    Some(format!("{}", v)),
                ));
            }
            None => (),
        }

        match self.alternate_register {
            Some(ref v) => {
                elem.children.push(new_element(
                    "alternateRegister",
                    Some(format!("{}", v)),
                ));
            }
            None => (),
        }

        match self.derived_from {
            Some(ref v) => {
                elem.children.push(new_element(
                    "derivedFrom",
                    Some(format!("{}", v)),
                ));
            }
            None => (),
        }

        match self.size {
            Some(ref v) => {
                elem.children
                    .push(new_element("size", Some(format!("{}", v))));
            }
            None => (),
        };

        match self.access {
            Some(ref v) => {
                elem.children.push(v.encode()?);
            }
            None => (),
        };

        match self.reset_value {
            Some(ref v) => {
                elem.children.push(new_element(
                    "resetValue",
                    Some(format!("0x{:08.x}", v)),
                ));
            }
            None => (),
        };

        match self.reset_mask {
            Some(ref v) => {
                elem.children.push(new_element(
                    "resetMask",
                    Some(format!("0x{:08.x}", v)),
                ));
            }
            None => (),
        };

        match self.fields {
            Some(ref v) => {
                let children: Result<Vec<Element>, SVDError> =
                    v.iter().map(Field::encode).collect();
                let fields = Element {
                    name: String::from("fields"),
                    attributes: HashMap::new(),
                    children: children?,
                    text: None,
                };
                elem.children.push(fields);
            }
            None => (),
        };

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
    use svd::bitrange::*;

    #[test]
    fn decode_encode() {
        let tests = vec![(
            RegisterInfo {
                name: String::from("WRITECTRL"),
                alternate_group: Some(String::from("alternate group")),
                alternate_register: Some(String::from("alternate register")),
                derived_from: Some(String::from("derived from")),
                description: String::from("Write Control Register"),
                address_offset: 8,
                size: Some(32),
                access: Some(Access::ReadWrite),
                reset_value: Some(0x00000000),
                reset_mask: Some(0x00000023),
                fields: Some(vec![Field {
                    name: String::from("WREN"),
                    description: Some(String::from(
                        "Enable Write/Erase Controller",
                    )),
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
                }]),
                write_constraint: None,
                modified_write_values: Some(ModifiedWriteValues::OneToToggle),
                _extensible: (),
            },
            "
            <register>
                <name>WRITECTRL</name>
                <description>Write Control Register</description>
                <addressOffset>0x8</addressOffset>
                <alternateGroup>alternate group</alternateGroup>
                <alternateRegister>alternate register</alternateRegister>
                <derivedFrom>derived from</derivedFrom>
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
