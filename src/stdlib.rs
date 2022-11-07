use crate::{Class, ExecutionContext, Node, Object, OutputSocket, InputSocket};
use std::rc::Rc;

pub fn nop_node_class() -> Class {
    Class {
        name: "nop".into(),
        default_node: Rc::new(NopNode) as Rc<dyn Node>,
    }
}

pub fn if_node_class() -> Class {
    Class {
        name: "if".into(),
        default_node: Rc::new(IfNode) as Rc<dyn Node>,
    }
}

pub fn bool_class() -> Class {
    Class {
        name: "bool".into(),
        default_node: Rc::new(BoolConstructor) as Rc<dyn Node>,
    }
}

pub fn any_class() -> Class {
    Class {
        name: "any".into(),
        default_node: Rc::new(NopNode) as Rc<dyn Node>,
    }
}

/// Does nothing. Literal NOP. The easiest node to implement
#[derive(Debug, Clone)]
pub struct NopNode;

impl Node for NopNode {
    fn execute(&self, _context: &mut ExecutionContext) -> usize {
        0
    }

    fn class(&self) -> Class {
        Class {
            name: "nop".into(),
            default_node: Rc::new(self.clone()) as Rc<dyn Node>,
        }
    }

    fn variants(&self) -> Vec<&str> {
        vec!["nop"]
    }

    fn current_variant(&self) -> &str {
        "nop"
    }

    fn set_variant(&mut self, _variant: &str) {}

    fn inputs(&self) -> Vec<InputSocket> {
        vec![]
    }

    fn outputs(&self) -> Vec<OutputSocket> {
        vec![]
    }

    fn branches(&self) -> usize {
        1
    }
}

#[derive(Debug, Clone)]
pub struct BoolConstructor;

impl Object for bool {
    fn class(&self) -> Class {
        bool_class()
    }

    fn as_number(&self) -> f32 {
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

    fn variants(&self) -> Vec<&str> {
        vec!["from-object"]
    }

    fn current_variant(&self) -> &str {
        "from-object"
    }

    fn set_variant(&mut self, _variant: &str) {}

    fn inputs(&self) -> Vec<InputSocket> {
        vec![InputSocket {
            class: any_class()
        }]
    }

    fn outputs(&self) -> Vec<OutputSocket> {
        vec![OutputSocket {
            class: bool_class(),
        }]
    }

    fn branches(&self) -> usize {
        1
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
        Class {
            name: "if".into(),
            default_node: Rc::new(self.clone()) as Rc<dyn Node>,
        }
    }

    fn variants(&self) -> Vec<&str> {
        vec!["if"]
    }

    fn current_variant(&self) -> &str {
        "if"
    }

    fn set_variant(&mut self, _variant: &str) {}

    fn inputs(&self) -> Vec<InputSocket> {
        vec![InputSocket {
            class: bool_class()
        }]
    }

    fn outputs(&self) -> Vec<OutputSocket> {
        vec![]
    }

    fn branches(&self) -> usize {
        2
    }
}
