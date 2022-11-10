use crate::{Class, ExecutionContext, InputSocket, Node, OutputSocket};
use std::{borrow::Cow, rc::Rc};

pub fn nop_node_class() -> Class {
    Class {
        name: "nop".into(),
        nodes: vec![Rc::new(NopNode) as Rc<dyn Node>],
        obj_from_str: None,
    }
}

/// Does nothing. Literal NOP. The easiest node to implement
#[derive(Debug, Clone)]
pub struct NopNode;

impl Node for NopNode {
    fn execute(&self, _context: &mut ExecutionContext) -> u32 {
        0
    }

    fn class(&self) -> Class {
        nop_node_class()
    }

    fn variants(&self) -> Vec<Cow<'_, str>> {
        vec!["nop".into()]
    }

    fn current_variant(&self) -> Cow<'_, str> {
        "nop".into()
    }

    fn set_variant(&mut self, _variant: &str) {}

    fn inputs(&self) -> Vec<InputSocket> {
        vec![]
    }

    fn outputs(&self) -> Vec<OutputSocket> {
        vec![]
    }
}
