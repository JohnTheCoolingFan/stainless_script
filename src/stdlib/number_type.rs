use crate::{Class, ExecutionContext, InputSocket, Node, Object, OutputSocket};
use std::rc::Rc;

use super::any_class;

pub fn number_class() -> Class {
    Class {
        name: "number".into(),
        default_node: Rc::new(NumberNode) as Rc<dyn Node>,
    }
}

impl Object for f64 {
    fn class(&self) -> Class {
        number_class()
    }

    fn as_number(&self) -> f64 {
        *self
    }

    fn as_bool(&self) -> bool {
        !(*self == 0.0)
    }

    fn get_field(&self, _field: &str) -> &std::rc::Rc<dyn Object> {
        unimplemented!()
    }

    fn set_field(&mut self, _field: &str, _valuee: &std::rc::Rc<dyn Object>) {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub struct NumberNode;

impl Node for NumberNode {
    fn execute(&self, context: &mut ExecutionContext) -> u32 {
        let res = context.get_inputs()[0].as_number();
        context.set_outputs(vec![Rc::new(res) as Rc<dyn Object>]);
        0
    }

    fn class(&self) -> Class {
        number_class()
    }

    fn variants(&self) -> Vec<std::borrow::Cow<'_, str>> {
        vec!["from-object".into()]
    }

    fn current_variant(&self) -> std::borrow::Cow<'_, str> {
        "from-object".into()
    }

    fn set_variant(&mut self, _variant: &str) {}

    fn inputs(&self) -> Vec<InputSocket> {
        vec![InputSocket { class: any_class() }]
    }

    fn outputs(&self) -> Vec<OutputSocket> {
        vec![OutputSocket {
            class: number_class(),
        }]
    }
}
