use xmltree::Element;

use error::*;
use {ElementExt, Parse};

#[derive(Debug)]
pub struct EnumeratedValue {
    pub description: Option<String>,
    pub is_default: Option<bool>,
    pub name: String,
    pub value: Option<u32>,
}

impl Parse for EnumeratedValue {
    fn parse(elem: &Element) -> Result<EnumeratedValue> {
        ensure!(
            elem.name == "enumeratedValue",
            "element name (`{}`) should be `enumeratedValue`",
            elem.name
        );

        // `<name>..</name>`
        let name = elem.children
            .iter()
            .find(|e| e.name == "name")
            .ok_or("missing obligatory `<name>` element")?
            .text()
            .chain_err(|| "parsing `<name>`")?;

        let mut description = None;
        let mut is_default = None;
        let mut value = None;

        // NOTE this closure is here to append "named {}" to the error chain
        (|| -> Result<()> {
            for (i, child) in elem.children.iter().enumerate() {
                match &*child.name {
                    "description" => {
                        ensure!(
                            description.is_none(),
                            "`<description>` element appears twice"
                        );

                        description = Some(child.text().chain_err(|| {
                            format!("parsing `<description>` (child #{})", i)
                        })?);
                    }
                    "isDefault" => {
                        ensure!(
                            is_default.is_none(),
                            "`<isDefault>` element appears twice"
                        );

                        is_default =
                            Some(::parse::bool(&child.text()?).chain_err(|| {
                                format!("parsing `<isDefault>` (child #{})", i)
                            })?);
                    }
                    "name" => {
                        // skip, this was handled above
                    }
                    "value" => {
                        ensure!(
                            value.is_none(),
                            "`<value>` element appears twice"
                        );

                        value = Some(::parse::u32(&child.text()?).chain_err(
                            || format!("parsing `<value>` (child #{})", i),
                        )?);
                    }
                    name => bail!(
                        "found unexpected `<{}>` element (child #{})",
                        name,
                        i
                    ),
                }
            }

            Ok(())
        })()
            .chain_err(|| format!("named `{}`", name))?;

        Ok(EnumeratedValue {
            description,
            is_default,
            name,
            value,
        })
    }
}
