use crate::{
    class::Class,
    module::{ModulePath, ModulePathParseError},
    program::ProgramId,
    socket::{InputSocket, OutputSocket},
    ExecutionContext,
};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    collections::BTreeMap,
    fmt::{Debug, Display},
    num::ParseIntError,
    rc::Rc,
    str::FromStr,
};
use thiserror::Error;

/// ID of a node
pub type NodeId = u32;

/// An ID to point to a node in other program
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct AbsoluteNodeId(pub ProgramId, pub NodeId);

impl Display for AbsoluteNodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}", self.0, self.1)
    }
}

impl FromStr for AbsoluteNodeId {
    type Err = AbsoluteNodeIdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut seq: Vec<String> = s.split('@').map(String::from).collect();
        let node_id: NodeId = seq
            .pop()
            .ok_or(AbsoluteNodeIdParseError::IdNotFound)?
            .parse()?;
        let path: ProgramId = seq[0].parse()?;
        Ok(Self(path, node_id))
    }
}

#[derive(Debug, Clone, Error)]
pub enum AbsoluteNodeIdParseError {
    #[error("Node ID not found in string")]
    IdNotFound,
    #[error("Failed to parse Node ID: {0}")]
    NodeIdParseError(ParseIntError),
    #[error("Failed to parse program ID path: {0}")]
    ProgramIdParseError(ModulePathParseError),
}

impl From<ParseIntError> for AbsoluteNodeIdParseError {
    fn from(e: ParseIntError) -> Self {
        Self::NodeIdParseError(e)
    }
}

impl From<ModulePathParseError> for AbsoluteNodeIdParseError {
    fn from(e: ModulePathParseError) -> Self {
        Self::ProgramIdParseError(e)
    }
}

/// ID of a branch of node
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct NodeBranchId(pub NodeId, pub usize);

impl From<&NodeBranchId> for u64 {
    fn from(s: &NodeBranchId) -> Self {
        (s.0 as u64) << 32 | s.1 as u64
    }
}

impl From<u64> for NodeBranchId {
    fn from(n: u64) -> Self {
        let branch_idx: u32 = (((1 << 33) - 1) & n) as u32;
        let node_id: NodeId = ((((1 << 33) - 1) << 32) & n) as NodeId;
        Self(node_id, branch_idx as usize)
    }
}

impl Serialize for NodeBranchId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        u64::from(self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for NodeBranchId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(NodeBranchId::from(u64::deserialize(deserializer)?))
    }
}

pub trait Node: Debug {
    /// Execution of the node's code. Returns a branch index.
    fn execute(&self, context: &mut ExecutionContext) -> usize;

    /// The class of the node
    fn class(&self) -> Class;

    /// Variants of a node. Internally can be anythingg that can be converted to string
    fn variants(&self) -> Vec<Cow<'_, str>>;

    /// Current selected variant of the node
    fn current_variant(&self) -> Cow<'_, str>;

    /// Set a specific variant of a node
    fn set_variant(&mut self, variant: &str);

    /// Whether variation can be set as a custom string (not listed in `variants`) or not
    fn accepts_arbitrary_variants(&self) -> bool {
        false
    }

    /// Get information about node's inputs
    fn inputs(&self) -> Vec<InputSocket>;

    /// Get information about node's outputs
    fn outputs(&self) -> Vec<OutputSocket>;

    /// How many branches this node has
    fn branches(&self) -> u32 {
        1
    }

    /// Clone the node itself instead of it wrapped in Rc
    fn clone_node(&self) -> Rc<dyn Node>;
}

#[derive(Debug, Clone, Default)]
pub struct NodeStorage {
    pub nodes: BTreeMap<NodeId, Rc<dyn Node>>,
    next_vacant: NodeId,
}

impl NodeStorage {
    pub fn get_node(&self, node_id: NodeId) -> Option<Rc<dyn Node>> {
        self.nodes.get(&node_id).cloned()
    }

    pub fn remove_node(&mut self, node_id: NodeId) -> Option<Rc<dyn Node>> {
        let node = self.nodes.remove(&node_id);
        if node_id < self.next_vacant {
            self.next_vacant = node_id
        }
        node
    }

    pub fn insert_node(&mut self, node: Rc<dyn Node>) -> NodeId {
        let mut node_id = self.next_vacant;
        self.nodes.insert(node_id, node);
        while self.nodes.get(&node_id).is_some() {
            node_id += 1;
        }
        self.next_vacant = node_id;
        node_id
    }

    pub fn insert_node_at(&mut self, node_id: NodeId, node: Rc<dyn Node>) {
        self.nodes.insert(node_id, node);
        while self.nodes.get(&self.next_vacant).is_some() {
            self.next_vacant += 1;
        }
    }
}

/// Information about a node stored in the program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub class: ModulePath,
    pub idx: usize,
    pub variant: String,
}
