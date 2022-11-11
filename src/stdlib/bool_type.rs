use super::any_class;
use crate::{Class, ExecutionContext, InputSocket, Node, Object, ObjectFromStr, OutputSocket};
use std::{borrow::Cow, rc::Rc};

pub fn bool_class() -> Class {
    Class {
        name: "bool".into(),
        nodes: vec![Rc::new(BoolNode) as Rc<dyn Node>],
        obj_from_str: Some(<bool as ObjectFromStr>::from_str),
    }
}

#[derive(Debug, Clone)]
pub struct BoolNode;

impl Object for bool {
    fn class(&self) -> Class {
        bool_class()
    }

    fn as_number(&self) -> f64 {
        if *self {
            1.0
        } else {
            0.0
        }
    }

    fn as_bool(&self) -> bool {
        *self
    }
}

impl Node for BoolNode {
    fn execute(&self, context: &mut ExecutionContext) -> usize {
        let cond = context.get_inputs()[0].as_bool();
        context.set_outputs(vec![Rc::new(cond) as Rc<dyn Object>]);
        0
    }

    fn class(&self) -> Class {
        bool_class()
    }

    fn variants(&self) -> Vec<Cow<'_, str>> {
        vec!["from-object".into()]
    }

    fn current_variant(&self) -> Cow<'_, str> {
        "from-object".into()
    }

    fn set_variant(&mut self, _variant: &str) {}

    fn inputs(&self) -> Vec<InputSocket> {
        vec![InputSocket { class: any_class() }]
    }

    fn outputs(&self) -> Vec<OutputSocket> {
        vec![OutputSocket {
            class: bool_class(),
        }]
    }

    fn clone_node(&self) -> Rc<dyn Node> {
        Rc::new(self.clone()) as Rc<dyn Node>
    }
}
