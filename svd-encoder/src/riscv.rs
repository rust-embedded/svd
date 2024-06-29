use super::{new_node, Config, Element, Encode, EncodeError, XMLNode};
use crate::svd::riscv::{Hart, Priority, Riscv};

impl Encode for Riscv {
    type Error = EncodeError;

    fn encode_with_config(&self, config: &Config) -> Result<Element, EncodeError> {
        let mut elem = Element::new("riscv");

        if let Some(clic) = &self.clic {
            elem.children.push(new_node("clic", clic.clone()));
        }
        if let Some(clint) = &self.clint {
            elem.children.push(new_node("clint", clint.clone()));
        }
        if let Some(plic) = &self.plic {
            elem.children.push(new_node("plic", plic.clone()));
        }
        if !self.core_interrupts.is_empty() {
            let mut interrupts = Element::new("coreInterrupts");
            for interrupt in &self.core_interrupts {
                interrupts
                    .children
                    .push(interrupt.encode_node_with_config(config)?);
            }
            elem.children.push(XMLNode::Element(interrupts));
        }
        if !self.priorities.is_empty() {
            let mut priorities = Element::new("priorities");
            for priority in &self.priorities {
                priorities
                    .children
                    .push(priority.encode_node_with_config(config)?);
            }
            elem.children.push(XMLNode::Element(priorities));
        }
        if !self.harts.is_empty() {
            let mut harts = Element::new("harts");
            for hart in &self.harts {
                harts.children.push(hart.encode_node_with_config(config)?);
            }
            elem.children.push(XMLNode::Element(harts));
        }

        Ok(elem)
    }
}

impl Encode for Priority {
    type Error = EncodeError;

    fn encode_with_config(&self, _config: &Config) -> Result<Element, EncodeError> {
        let mut children = vec![new_node("name", self.name.clone())];
        if let Some(desc) = &self.description {
            children.push(new_node("description", desc.clone()));
        }
        children.push(new_node("value", format!("{}", self.value)));

        let mut elem = Element::new("priority");
        elem.children = children;
        Ok(elem)
    }
}

impl Encode for Hart {
    type Error = EncodeError;

    fn encode_with_config(&self, _config: &Config) -> Result<Element, EncodeError> {
        let mut children = vec![new_node("name", self.name.clone())];
        if let Some(desc) = &self.description {
            children.push(new_node("description", desc.clone()));
        }
        children.push(new_node("value", format!("{}", self.value)));

        let mut elem = Element::new("hart");
        elem.children = children;
        Ok(elem)
    }
}
