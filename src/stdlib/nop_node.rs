use crate::{Class, ExecutionContext, InputSocket, Node, OutputSocket};
use std::{borrow::Cow, rc::Rc};

pub fn nop_node_class() -> Class {
    Class {
        name: "nop".into(),
        default_node: Rc::new(NopNode) as Rc<dyn Node>,
    }
}

/// Does nothing. Literal NOP. The easiest node to implement
#[derive(Debug, Clone)]
pub struct NopNode;

impl Node for NopNode {
    fn execute(&self, _context: &mut ExecutionContext) -> usize {
        0
    }

    fn class(&self) -> Class {
        Class {
            name: "nop".into(),
            default_node: Rc::new(self.clone()) as Rc<dyn Node>,
        }
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
