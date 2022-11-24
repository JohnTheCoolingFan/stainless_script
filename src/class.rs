use crate::{
    node::{Node, NodeId},
    object::Object,
};
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt::Debug, rc::Rc};

type ObjFromStrFn = fn(&str) -> Result<Rc<dyn Object>, Box<dyn Error + Send + Sync>>;

/// Describes a data type. Provides default node that is usually a constructor or some other node.
/// Variations of the default node are methods of this class.
#[derive(Clone)]
pub struct Class {
    pub name: String,
    /// Default node to be placed when selecting a class to put. Usually a constructor method.
    pub nodes: Vec<Rc<dyn Node>>,
    pub obj_from_str: Option<ObjFromStrFn>,
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Eq for Class {}

impl Debug for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Class")
            .field("name", &self.name)
            .field("node", &self.nodes)
            .finish()
    }
}

impl Serialize for Class {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.name.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Class {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let name = String::deserialize(deserializer)?;
        Ok(Self {
            name,
            nodes: vec![],
            obj_from_str: None,
        })
    }
}

/// Description of a class stored in the program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtoClass {
    pub name: String,
    /// IDs of subroutine call nodes that define the methods
    pub nodes: Vec<NodeId>,
}
