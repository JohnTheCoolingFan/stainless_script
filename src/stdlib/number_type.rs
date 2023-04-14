use crate::{
    class::Class,
    node::Node,
    object::{Object, ObjectEq, ObjectFromStr, ObjectOrd, ObjectPartialEq, ObjectPartialOrd},
    socket::{InputSocket, OutputSocket},
    ExecutionContext,
};
use std::rc::Rc;

use super::any_class;

pub fn number_class() -> Class {
    Class {
        name: "number".into(),
        nodes: vec![Rc::new(NumberNode) as Rc<dyn Node>],
        obj_from_str: Some(<f64 as ObjectFromStr>::from_str),
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
        *self != 0.0
    }

    fn get_field(&self, field: Rc<dyn Object>) -> Rc<dyn Object> {
        match field.as_string().as_ref() {
            "is_integer" => Rc::new(self.fract() == 0.0) as Rc<dyn Object>,
            "as_integer" => Rc::new(self - self.fract()) as Rc<dyn Object>,
            _ => panic!("Unknown field: {field}"),
        }
    }
}

impl ObjectPartialEq for f64 {
    fn eq(&self, other: Rc<dyn Object>) -> bool {
        if other.class() == self.class() {
            PartialEq::eq(self, &other.as_number())
        } else {
            false
        }
    }
}

impl ObjectPartialOrd for f64 {
    fn partial_cmp(&self, other: Rc<dyn Object>) -> Option<std::cmp::Ordering> {
        if other.class() == self.class() {
            PartialOrd::partial_cmp(self, &other.as_number())
        } else {
            None
        }
    }
}

impl ObjectEq for f64 {}

impl ObjectOrd for f64 {
    fn cmp(&self, other: Rc<dyn Object>) -> std::cmp::Ordering {
        ObjectPartialOrd::partial_cmp(self, other).unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct NumberNode;

impl Node for NumberNode {
    fn execute(&self, context: &mut ExecutionContext) -> usize {
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

    fn clone_node(&self) -> Rc<dyn Node> {
        Rc::new(self.clone()) as Rc<dyn Node>
    }
}
