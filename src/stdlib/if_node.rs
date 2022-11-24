use super::bool_class;
use crate::{
    class::Class,
    node::Node,
    socket::{InputSocket, OutputSocket},
    ExecutionContext,
};
use std::{borrow::Cow, rc::Rc};

pub fn if_node_class() -> Class {
    Class {
        name: "if".into(),
        nodes: vec![Rc::new(IfNode) as Rc<dyn Node>],
        obj_from_str: None,
    }
}

#[derive(Debug, Clone)]
pub struct IfNode;

impl Node for IfNode {
    fn execute(&self, context: &mut ExecutionContext) -> usize {
        let cond = context.get_inputs()[0].as_bool();
        cond as usize
    }

    fn class(&self) -> Class {
        if_node_class()
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

    fn clone_node(&self) -> Rc<dyn Node> {
        Rc::new(self.clone()) as Rc<dyn Node>
    }
}
