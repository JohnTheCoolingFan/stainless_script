use super::any_class;
use crate::{Class, ExecutionContext, InputSocket, Node, Object, OutputSocket};
use std::{borrow::Cow, rc::Rc};

pub fn bool_class() -> Class {
    Class {
        name: "bool".into(),
        default_node: Rc::new(BoolConstructor) as Rc<dyn Node>,
    }
}

#[derive(Debug, Clone)]
pub struct BoolConstructor;

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

    fn get_field(&self, _field: &str) -> &Rc<dyn Object> {
        unimplemented!()
    }

    fn set_field(&mut self, _field: &str, _value: &Rc<dyn Object>) {
        unimplemented!()
    }
}

impl Node for BoolConstructor {
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
}
