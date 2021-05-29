use super::{elementext::ElementExt, Config, Node, Parse, Result, SVDError};
use crate::svd::Interrupt;

fn parse_interrupt(tree: &Node, name: String) -> Result<Interrupt> {
    Ok(Interrupt {
        name,
        description: tree.get_child_text_opt("description")?,
        value: tree.get_child_u32("value")?,
    })
}

impl Parse for Interrupt {
    type Object = Self;
    type Error = anyhow::Error;
    type Config = Config;

    fn parse(tree: &Node, _config: &Self::Config) -> Result<Self> {
        if !tree.has_tag_name("interrupt") {
            return Err(SVDError::NotExpectedTag("interrupt".to_string())
                .at(tree.id())
                .into());
        }
        let name = tree.get_child_text("name")?;
        parse_interrupt(tree, name.clone())
    }
}
