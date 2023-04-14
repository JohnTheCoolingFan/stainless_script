use super::{any_class, number_class, AnyType};
use crate::{
    class::Class,
    node::Node,
    object::{Object, ObjectEq, ObjectFromStr, ObjectOrd, ObjectPartialEq, ObjectPartialOrd},
    socket::{InputSocket, OutputSocket},
    ExecutionContext,
};
use stainless_script_derive::{ObjectEq, ObjectOrd};
use std::{fmt::Display, rc::Rc, str::FromStr};

pub fn array_class() -> Class {
    Class {
        name: "array".into(),
        nodes: vec![Rc::new(ArrayConstructor(1)) as Rc<dyn Node>],
        obj_from_str: Some(<Array as ObjectFromStr>::from_str),
    }
}

#[derive(Debug, Clone, ObjectEq, ObjectOrd)]
pub struct Array(Vec<Rc<dyn Object>>);

impl FromStr for Array {
    type Err = <AnyType as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_eq!(&s[0..2], "[");
        assert_eq!(&s[s.len() - 1..s.len()], "]");
        let items: Result<Vec<Rc<dyn Object>>, Self::Err> = s[1..s.len() - 1]
            .split(',')
            .map(|s| {
                let trimmed = s.trim();
                Ok(Rc::new(trimmed.parse::<AnyType>()?) as Rc<dyn Object>)
            })
            .collect();
        items.map(Array)
    }
}

impl Display for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl Object for Array {
    fn class(&self) -> Class {
        array_class()
    }

    fn as_number(&self) -> f64 {
        panic!("Cannot convert array to number")
    }

    fn as_bool(&self) -> bool {
        !self.0.is_empty()
    }

    fn get_field(&self, field: Rc<dyn Object>) -> Rc<dyn Object> {
        if field.class() == number_class() {
            Rc::clone(&self.0[field.as_number() as usize])
        } else {
            match field.as_string().as_str() {
                "len" => Rc::new(self.0.len() as f64) as Rc<dyn Object>,
                _ => panic!("Unknown fields: {field}"),
            }
        }
    }

    fn set_field(&mut self, field: Rc<dyn Object>, value: Rc<dyn Object>) {
        if field.class() == number_class() {
            self.0[field.as_number() as usize] = value;
        }
    }
}

impl ObjectPartialEq for Array {
    fn eq(&self, other: Rc<dyn Object>) -> bool {
        if other.class() == self.class() {
            if other
                .get_field(Rc::new("len".to_string()) as Rc<dyn Object>)
                .as_number() as usize
                == self.0.len()
            {
                let other = &other as &dyn std::any::Any;
                if let Some(other) = other.downcast_ref::<Self>() {
                    self.0
                        .iter()
                        .zip(other.0.iter())
                        .all(|(l, r)| l.eq(Rc::clone(r)))
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl ObjectPartialOrd for Array {
    /// UNIMPLEMENTED, WILL PANIC, DO NOT USE AS DICT KEY
    fn partial_cmp(&self, _other: Rc<dyn Object>) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct ArrayConstructor(usize);

impl Node for ArrayConstructor {
    fn execute(&self, context: &mut ExecutionContext) -> usize {
        let items = context.get_inputs();
        context.set_outputs(vec![Rc::new(Array(items)) as Rc<dyn Object>]);
        0
    }

    fn class(&self) -> Class {
        array_class()
    }

    fn variants(&self) -> Vec<std::borrow::Cow<'_, str>> {
        vec![self.current_variant()]
    }

    fn current_variant(&self) -> std::borrow::Cow<'_, str> {
        format!("array-{}", self.0).into()
    }

    fn set_variant(&mut self, variant: &str) {
        self.0 = variant.strip_prefix("array-").unwrap().parse().unwrap()
    }

    fn accepts_arbitrary_variants(&self) -> bool {
        true
    }

    fn inputs(&self) -> Vec<InputSocket> {
        vec![InputSocket { class: any_class() }; self.0]
    }

    fn outputs(&self) -> Vec<OutputSocket> {
        vec![OutputSocket {
            class: array_class(),
        }]
    }

    fn clone_node(&self) -> Rc<dyn Node> {
        Rc::new(self.clone()) as Rc<dyn Node>
    }
}
