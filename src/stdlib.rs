use crate::{Class, Node};
use std::rc::Rc;

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
    fn execute(&self, _context: &mut crate::ExecutionContext) -> usize {
        0
    }

    fn class(&self) -> Class {
        Class {
            name: "nop".into(),
            default_node: Rc::new(self.clone()) as Rc<dyn Node>,
        }
    }

    fn variants(&self) -> Vec<&str> {
        vec!["nop"]
    }

    fn current_variant(&self) -> &str {
        "nop"
    }

    fn set_variant(&mut self, _variant: &str) {}

    fn inputs(&self) -> Vec<crate::InputSocket> {
        vec![]
    }

    fn outputs(&self) -> Vec<crate::OutputSocket> {
        vec![]
    }

    fn branches(&self) -> usize {
        1
    }
}
