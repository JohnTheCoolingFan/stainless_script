use crate::{
    class::Class,
    node::Node,
    object::{Object, ObjectEq, ObjectFromStr, ObjectOrd, ObjectPartialEq, ObjectPartialOrd},
    socket::{InputSocket, OutputSocket},
    ExecutionContext,
};

use super::any_class;
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

impl ObjectPartialEq for bool {
    fn eq(&self, other: Rc<dyn Object>) -> bool {
        other.class() == self.class() && *self == other.as_bool()
    }
}

impl ObjectPartialOrd for bool {
    fn partial_cmp(&self, other: Rc<dyn Object>) -> Option<std::cmp::Ordering> {
        if other.class() == self.class() {
            PartialOrd::partial_cmp(self, &other.as_bool())
        } else {
            None
        }
    }
}

impl ObjectEq for bool {}

impl ObjectOrd for bool {
    fn cmp(&self, other: Rc<dyn Object>) -> std::cmp::Ordering {
        ObjectPartialOrd::partial_cmp(self, other).unwrap()
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
