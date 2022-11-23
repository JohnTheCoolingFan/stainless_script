use crate::{Class, ExecutionContext, InputSocket, Node, Object, OutputSocket};
use std::rc::Rc;

pub fn start_node_class() -> Class {
    Class {
        name: "start".into(),
        nodes: vec![Rc::new(StartNode(vec![])) as Rc<dyn Node>],
        obj_from_str: None,
    }
}

pub fn end_node_class() -> Class {
    Class {
        name: "end".into(),
        nodes: vec![Rc::new(EndNode(vec![])) as Rc<dyn Node>],
        obj_from_str: None,
    }
}

/// Start of a program or subroutine
#[derive(Debug, Clone)]
pub struct StartNode(Vec<OutputSocket>);

impl Node for StartNode {
    fn execute(&self, context: &mut ExecutionContext) -> usize {
        todo!()
    }

    fn class(&self) -> Class {
        start_node_class()
    }

    fn variants(&self) -> Vec<std::borrow::Cow<'_, str>> {
        vec!["start".into()]
    }

    fn current_variant(&self) -> std::borrow::Cow<'_, str> {
        "start".into()
    }

    fn set_variant(&mut self, _variant: &str) {}

    fn inputs(&self) -> Vec<InputSocket> {
        vec![]
    }

    fn outputs(&self) -> Vec<OutputSocket> {
        self.0.clone()
    }

    fn clone_node(&self) -> Rc<dyn Node> {
        Rc::new(self.clone()) as Rc<dyn Node>
    }
}

/// End of a program or subroutine
#[derive(Debug, Clone)]
pub struct EndNode(Vec<InputSocket>);

impl Node for EndNode {
    fn execute(&self, context: &mut ExecutionContext) -> usize {
        todo!()
    }

    fn class(&self) -> Class {
        end_node_class()
    }

    fn variants(&self) -> Vec<std::borrow::Cow<'_, str>> {
        vec!["end".into()]
    }

    fn current_variant(&self) -> std::borrow::Cow<'_, str> {
        "end".into()
    }

    fn set_variant(&mut self, _variant: &str) {}

    fn inputs(&self) -> Vec<InputSocket> {
        self.0.clone()
    }

    fn outputs(&self) -> Vec<OutputSocket> {
        vec![]
    }

    fn clone_node(&self) -> Rc<dyn Node> {
        Rc::new(self.clone()) as Rc<dyn Node>
    }
}
