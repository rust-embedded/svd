use super::*;
use crate::svd::riscv::{Hart, Interrupt, Priority, Riscv};

impl Parse for Riscv {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, config: &Config) -> Result<Self, Self::Error> {
        if !tree.has_tag_name("riscv") {
            return Err(SVDError::NotExpectedTag("riscv".to_string()).at(tree.id()));
        }

        let mut builder = Riscv::builder()
            .clic(tree.get_child_text("clic").ok())
            .clint(tree.get_child_text("clint").ok())
            .plic(tree.get_child_text("plic").ok());

        if let Some(interrupts) = tree.get_child("coreInterrupts") {
            let interrupts: Result<Vec<_>, _> = interrupts
                .children()
                .filter(|t| t.is_element() && t.has_tag_name("interrupt"))
                .map(|i| Interrupt::parse(&i, config))
                .collect();
            builder = builder.core_interrupts(interrupts?);
        }

        if let Some(priorities) = tree.get_child("priorities") {
            let priorities: Result<Vec<_>, _> = priorities
                .children()
                .filter(|t| t.is_element() && t.has_tag_name("priority"))
                .map(|i| Priority::parse(&i, config))
                .collect();
            builder = builder.priorities(priorities?);
        };

        if let Some(harts) = tree.get_child("harts") {
            let harts: Result<Vec<_>, _> = harts
                .children()
                .filter(|t| t.is_element() && t.has_tag_name("hart"))
                .map(|i| Hart::parse(&i, config))
                .collect();
            builder = builder.harts(harts?);
        };

        builder
            .build(config.validate_level)
            .map_err(|e| SVDError::from(e).at(tree.id()))
    }
}

impl Parse for Priority {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, config: &Config) -> Result<Self, Self::Error> {
        if !tree.has_tag_name("priority") {
            return Err(SVDError::NotExpectedTag("priority".to_string()).at(tree.id()));
        }

        Priority::builder()
            .name(tree.get_child_text("name")?)
            .description(tree.get_child_text_opt("description")?)
            .value(tree.get_child_u32("value")?)
            .build(config.validate_level)
            .map_err(|e| SVDError::from(e).at(tree.id()))
    }
}

impl Parse for Hart {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, config: &Config) -> Result<Self, Self::Error> {
        if !tree.has_tag_name("hart") {
            return Err(SVDError::NotExpectedTag("hart".to_string()).at(tree.id()));
        }

        Hart::builder()
            .name(tree.get_child_text("name")?)
            .description(tree.get_child_text_opt("description")?)
            .value(tree.get_child_u32("value")?)
            .build(config.validate_level)
            .map_err(|e| SVDError::from(e).at(tree.id()))
    }
}
