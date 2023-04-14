use crate::{
    class::Class,
    node::Node,
    object::{Object, ObjectEq, ObjectFromStr, ObjectOrd, ObjectPartialEq, ObjectPartialOrd},
    socket::{InputSocket, OutputSocket},
    ExecutionContext,
};

use super::any_class;
use std::rc::Rc;

pub fn string_class() -> Class {
    Class {
        name: "string".into(),
        nodes: vec![Rc::new(StringNode) as Rc<dyn Node>],
        obj_from_str: Some(<String as ObjectFromStr>::from_str),
    }
}

impl Object for String {
    fn class(&self) -> Class {
        string_class()
    }

    fn as_number(&self) -> f64 {
        self.parse().unwrap()
    }

    fn as_bool(&self) -> bool {
        !self.is_empty()
    }
}

impl ObjectPartialEq for String {
    fn eq(&self, other: Rc<dyn Object>) -> bool {
        if self.class() == other.class() {
            PartialEq::eq(self, &other.as_string())
        } else {
            false
        }
    }
}

impl ObjectPartialOrd for String {
    fn partial_cmp(&self, other: Rc<dyn Object>) -> Option<std::cmp::Ordering> {
        if self.class() == other.class() {
            PartialOrd::partial_cmp(self, &other.as_string())
        } else {
            None
        }
    }
}

impl ObjectEq for String {}

impl ObjectOrd for String {
    fn cmp(&self, other: Rc<dyn Object>) -> std::cmp::Ordering {
        ObjectPartialOrd::partial_cmp(self, other).unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct StringNode;

impl Node for StringNode {
    fn execute(&self, context: &mut ExecutionContext) -> usize {
        let ret = context.get_inputs()[0].as_string();
        context.set_outputs(vec![Rc::new(ret) as Rc<dyn Object>]);
        0
    }

    fn class(&self) -> Class {
        string_class()
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
            class: string_class(),
        }]
    }

    fn clone_node(&self) -> Rc<dyn Node> {
        Rc::new(self.clone()) as Rc<dyn Node>
    }
}
