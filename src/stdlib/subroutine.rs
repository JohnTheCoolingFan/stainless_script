use crate::object::{ObjectEq, ObjectFromStr, ObjectOrd, ObjectPartialEq, ObjectPartialOrd};
use stainless_script_derive::{ObjectEq, ObjectOrd, ObjectPartialEq, ObjectPartialOrd};
use thiserror::Error;

use crate::{
    class::Class,
    module::ModulePath,
    node::{AbsoluteNodeId, AbsoluteNodeIdParseError, Node, NodeId},
    object::Object,
    socket::{InputSocket, OutputSocket},
    ExecutionContext,
};
use std::{any::Any, borrow::Cow, collections::VecDeque, fmt::Display, rc::Rc, str::FromStr};

/// The node provided should be cloned and set the proper ids before any use. By default, all ids
/// are at their max values
pub fn subroutine_class() -> Class {
    let empty_path = ModulePath(vec![], String::new());
    Class {
        name: "subroutine".into(),
        nodes: vec![Rc::new(SubroutineCall(SubroutineCallTarget::Supplied)) as Rc<dyn Node>],
        obj_from_str: Some(<Subroutine as ObjectFromStr>::from_str),
    }
}

/// This is a special class that tells to look to the node id outputs provided in the class for inputs
pub fn subroutine_input_class(id: &AbsoluteNodeId) -> Class {
    Class {
        name: format!("subroutine_input@{id}"),
        nodes: vec![],
        obj_from_str: None,
    }
}

/// This is a special class that tells to look to the node id inputs provided in the class for outputs
pub fn subroutine_output_class(id: &AbsoluteNodeId) -> Class {
    Class {
        name: format!("subroutine_output@{id}"),
        nodes: vec![],
        obj_from_str: None,
    }
}

pub fn supplied_subroutine_io_class() -> Class {
    Class {
        name: format!("from_supplied_subroutine"),
        nodes: vec![],
        obj_from_str: None,
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    ObjectPartialEq,
    ObjectEq,
    ObjectPartialOrd,
    ObjectOrd,
)]
pub struct Subroutine {
    input: AbsoluteNodeId,
    output: AbsoluteNodeId,
}

impl Display for Subroutine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "subroutine:{}:{}", self.input, self.output)
    }
}

#[derive(Debug, Clone, Error)]
pub enum SubroutineParseError {
    #[error("Failed to parse input node id: {0}")]
    InputNodeIdParseError(AbsoluteNodeIdParseError),
    #[error("Failed to parse output node id: {0}")]
    OutputNodeIdParseError(AbsoluteNodeIdParseError),
}

impl FromStr for Subroutine {
    type Err = SubroutineParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ids = s.strip_prefix("subroutine:").unwrap().split(':');
        let id_start = ids.next().unwrap();
        let id_end = ids.next().unwrap();
        let input_node = AbsoluteNodeId::from_str(id_start)
            .map_err(SubroutineParseError::InputNodeIdParseError)?;
        let output_node = AbsoluteNodeId::from_str(id_end)
            .map_err(SubroutineParseError::OutputNodeIdParseError)?;
        Ok(Self {
            input: input_node,
            output: output_node,
        })
    }
}

impl Object for Subroutine {
    fn class(&self) -> Class {
        subroutine_class()
    }

    fn as_number(&self) -> f64 {
        panic!("Cannot convert subroutine to number");
    }

    fn as_bool(&self) -> bool {
        panic!("Cannot convert subroutine to bool");
    }
}

#[derive(Debug, Clone)]
enum SubroutineCallTarget {
    Supplied,
    Fixed(Subroutine),
}

impl Display for SubroutineCallTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Supplied => write!(f, "supplied"),
            Self::Fixed(sub) => write!(f, "{}", sub),
        }
    }
}

// The end node id is kinda unused... It would be awesome to guarantee that the subroutine doesn't
// branch out to some different end
#[derive(Debug, Clone)]
pub struct SubroutineCall(SubroutineCallTarget);

impl Node for SubroutineCall {
    fn execute(&self, context: &mut ExecutionContext) -> usize {
        let inputs = context.get_inputs();
        match self.0 {
            SubroutineCallTarget::Supplied => {
                let inputs_dequeue = VecDeque::from(inputs);
                let subroutine = inputs_dequeue.pop_front().unwrap();
                match (subroutine as Rc<dyn Any>).downcast::<Subroutine>() {
                    Ok(sub) => context.execute_subroutine(sub.input, Vec::from(inputs_dequeue)),
                    Err(obj) => panic!(
                        "Failed to execute subroutine, expected subroutine object to be supplied"
                    ),
                }
            }
            SubroutineCallTarget::Fixed(sub) => {
                context.execute_subroutine(sub.input, inputs);
            }
        }
        0
    }

    fn class(&self) -> Class {
        subroutine_class()
    }

    /// Format: subroutine:<start_node_id>:<end_node_id>
    fn variants(&self) -> Vec<Cow<'_, str>> {
        let mut result = vec![self.current_variant()];
        if matches!(self.0, SubroutineCallTarget::Fixed(_)) {
            result.push(SubroutineCallTarget::Supplied.to_string().into())
        }
        result
    }

    /// Format: subroutine:<start_node_id>:<end_node_id>
    fn current_variant(&self) -> Cow<'_, str> {
        self.0.to_string().into()
    }

    /// Format: subroutine@<start_node_id>:<end_node_id>
    fn set_variant(&mut self, variant: &str) {
        match variant {
            "supplied" => self.0 = SubroutineCallTarget::Supplied,
            _ => {
                if let Ok(subroutine) = <Subroutine as FromStr>::from_str(variant) {
                    self.0 = SubroutineCallTarget::Fixed(subroutine);
                }
            }
        }
    }

    fn inputs(&self) -> Vec<InputSocket> {
        match self.0 {
            SubroutineCallTarget::Fixed(s) => {
                vec![InputSocket {
                    class: subroutine_input_class(&s.input),
                }]
            }
            SubroutineCallTarget::Supplied => {
                vec![
                    InputSocket {
                        class: subroutine_class(),
                    },
                    InputSocket {
                        class: supplied_subroutine_io_class(),
                    },
                ]
            }
        }
    }

    fn outputs(&self) -> Vec<OutputSocket> {
        match self.0 {
            SubroutineCallTarget::Fixed(s) => {
                vec![OutputSocket {
                    class: subroutine_output_class(&s.output),
                }]
            }
            SubroutineCallTarget::Supplied => {
                vec![OutputSocket {
                    class: supplied_subroutine_io_class(),
                }]
            }
        }
    }

    fn clone_node(&self) -> Rc<dyn Node> {
        Rc::new(self.clone()) as Rc<dyn Node>
    }

    fn accepts_arbitrary_variants(&self) -> bool {
        true
    }
}
