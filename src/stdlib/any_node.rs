use super::NopNode;
use crate::{Class, Node};
use std::rc::Rc;

pub fn any_class() -> Class {
    Class {
        name: "any".into(),
        default_node: Rc::new(NopNode) as Rc<dyn Node>,
    }
}
