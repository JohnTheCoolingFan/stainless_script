use crate::{
    class::Class,
    node::Node,
    socket::{InputSocket, OutputSocket},
    ExecutionContext,
};
use std::{borrow::Cow, rc::Rc};

use super::{any_class, string_class};

pub fn variable_get_class() -> Class {
    Class {
        name: "variable_get".into(),
        nodes: vec![Rc::new(VariableGet) as Rc<dyn Node>],
        obj_from_str: None,
    }
}

pub fn variable_set_class() -> Class {
    Class {
        name: "variable_set".into(),
        nodes: vec![],
        obj_from_str: None,
    }
}

#[derive(Debug, Clone)]
pub struct VariableGet;

impl Node for VariableGet {
    fn execute(&self, context: &mut ExecutionContext) -> usize {
        let inputs = context.get_inputs();
        context.set_outputs(
            inputs
                .get(0)
                .and_then(|name| context.get_variable(&name.as_string()))
                .into_iter()
                .collect(),
        );
        0
    }

    fn class(&self) -> Class {
        variable_get_class()
    }

    fn variants(&self) -> Vec<Cow<'_, str>> {
        vec!["get".into()]
    }

    fn current_variant(&self) -> Cow<'_, str> {
        "get".into()
    }

    fn set_variant(&mut self, _variant: &str) {}

    fn inputs(&self) -> Vec<InputSocket> {
        vec![InputSocket {
            class: string_class(),
        }]
    }

    fn outputs(&self) -> Vec<OutputSocket> {
        vec![OutputSocket { class: any_class() }]
    }

    fn clone_node(&self) -> Rc<dyn Node> {
        Rc::new(self.clone()) as Rc<dyn Node>
    }
}

#[derive(Debug, Clone)]
pub struct VariableSet;

impl Node for VariableSet {
    fn execute(&self, context: &mut ExecutionContext) -> usize {
        let inputs = context.get_inputs();
        let name = inputs[0].as_string();
        let value = &inputs[1];
        context.set_variable(&name, Rc::clone(value));
        0
    }

    fn class(&self) -> Class {
        variable_set_class()
    }

    fn variants(&self) -> Vec<Cow<'_, str>> {
        vec!["set".into()]
    }

    fn current_variant(&self) -> Cow<'_, str> {
        "set".into()
    }

    fn set_variant(&mut self, _variant: &str) {}

    fn inputs(&self) -> Vec<InputSocket> {
        vec![
            InputSocket {
                class: string_class(),
            },
            InputSocket { class: any_class() },
        ]
    }

    fn outputs(&self) -> Vec<OutputSocket> {
        vec![]
    }

    fn clone_node(&self) -> Rc<dyn Node> {
        Rc::new(self.clone()) as Rc<dyn Node>
    }
}
