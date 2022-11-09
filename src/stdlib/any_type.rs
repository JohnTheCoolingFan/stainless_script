use super::NopNode;
use crate::{Class, Node, Object};
use std::{rc::Rc, str::FromStr, fmt::Display};

pub fn any_class() -> Class {
    Class {
        name: "any".into(),
        node: Rc::new(NopNode) as Rc<dyn Node>,
    }
}

#[derive(Debug, Clone)]
pub struct AnyType(String);

impl FromStr for AnyType {
    type Err = <String as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        String::from_str(s).map(AnyType)
    }
}

impl Display for AnyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Object for AnyType {
    fn class(&self) -> Class {
        any_class()
    }

    fn as_number(&self) -> f64 {
        self.0.parse().unwrap()
    }

    fn as_bool(&self) -> bool {
        !self.0.is_empty()
    }
}
