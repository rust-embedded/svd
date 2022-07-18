use super::{new_node, Config, Element, Encode, EncodeError};

use crate::{config::change_case, svd::Interrupt};

impl Encode for Interrupt {
    type Error = EncodeError;

    fn encode_with_config(&self, config: &Config) -> Result<Element, EncodeError> {
        let mut children = vec![new_node(
            "name",
            change_case(&self.name, config.interrupt_name),
        )];
        if let Some(d) = self.description.clone() {
            children.push(new_node("description", d));
        }
        children.push(new_node("value", format!("{}", self.value)));
        let mut elem = Element::new("interrupt");
        elem.children = children;
        Ok(elem)
    }
}
