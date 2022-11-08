use super::bool_class;
use crate::{Class, ExecutionContext, InputSocket, Node, OutputSocket};
use std::{borrow::Cow, rc::Rc};

pub fn if_node_class() -> Class {
    Class {
        name: "if".into(),
        default_node: Rc::new(IfNode) as Rc<dyn Node>,
    }
}

#[derive(Debug, Clone)]
pub struct IfNode;

impl Node for IfNode {
    fn execute(&self, context: &mut ExecutionContext) -> u32 {
        let cond = context.get_inputs()[0].as_bool();
        cond as u32
    }

    fn class(&self) -> Class {
        Class {
            name: "if".into(),
            default_node: Rc::new(self.clone()) as Rc<dyn Node>,
        }
    }

    fn variants(&self) -> Vec<Cow<'_, str>> {
        vec!["if".into()]
    }

    fn current_variant(&self) -> Cow<'_, str> {
        "if".into()
    }

    fn set_variant(&mut self, _variant: &str) {}

    fn inputs(&self) -> Vec<InputSocket> {
        vec![InputSocket {
            class: bool_class(),
        }]
    }

    fn outputs(&self) -> Vec<OutputSocket> {
        vec![]
    }

    fn branches(&self) -> u32 {
        2
    }
}
