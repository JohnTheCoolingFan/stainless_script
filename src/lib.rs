use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    error::Error,
    fmt::{Debug, Display},
    rc::Rc,
    str::FromStr,
};

pub mod stdlib;

/// Used to index items across programs/packages. Built with executor upon loading programs.
pub struct Module {
    pub items: HashMap<String, ModuleItem>,
}

pub enum ModuleItem {
    /// Not implemented yet, not parsed from the program file
    Constant(Rc<dyn Object>),
    Class(Class),
    Module(Module),
}

/// Describes a data type. Provides default node that is usually a constructor or some other node.
/// Variations of the default node are methods of this class.
#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    /// Default node to be placed when selecting a class to put. Usually a constructor method.
    pub default_node: Rc<dyn Node>,
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Eq for Class {}

pub trait ObjectFromStr {
    fn from_str(s: &str) -> Result<Self, Box<dyn Error + Send + Sync>>
    where
        Self: Sized;
}

impl<T: FromStr> ObjectFromStr for T
where
    T::Err: 'static + Error + Send + Sync,
{
    fn from_str(s: &str) -> Result<Self, Box<dyn Error + Send + Sync>>
    where
        Self: Sized,
    {
        <Self as FromStr>::from_str(s).map_err(Into::into)
    }
}

/// The object of a data type. Data type is derived from the object's class. Methods specified here
/// are for use in nodes mostly.
pub trait Object: Display + ObjectFromStr {
    fn class(&self) -> Class;
    /// Since Object requires Display, this has little use and is implemented  through ToString,
    /// which is implemented for all types implementing Display. Left for consistency with
    /// as_number and other methods
    fn as_string(&self) -> String {
        self.to_string()
    }
    /// Convert to number
    fn as_number(&self) -> f32;
    /// Convert to boolean
    fn as_bool(&self) -> bool;
    /// Suggested implementation: Have a `HashMap<String, Rc<dyn Object>>` to manage fields
    fn get_field(&self, field: &str) -> &Rc<dyn Object>;
    /// Suggested implementation: use `String::from` to convert `&str` to `String` and use that as
    /// insertion key.
    fn set_field(&mut self, field: &str, value: &Rc<dyn Object>);
}

/// Input of a node.
pub struct InputSocket {
    pub class: Class,
}

/// Output of a node
pub struct OutputSocket {
    pub class: Class,
}

/// Context for nodes. Nodes get their inputs, set their ouputs, redirect to subroutine and other
/// through this context.
pub struct ExecutionContext<'a> {
    parent: Option<&'a ExecutionContext<'a>>,
}

impl<'a> ExecutionContext<'a> {
    pub fn new(parent: Option<&'a ExecutionContext<'a>>) -> ExecutionContext<'a> {
        Self { parent }
    }

    /// Redirect execution to a subroutine. Returns whatever end node receives.
    pub fn execute_subroutine(&'a self, start: AbsoluteNodeId) -> Vec<Rc<dyn Object>> {
        let sub_context = Self::new(Some(self));
        todo!()
    }

    /// Finish executing subroutine, return to caller.
    pub fn finish_subroutine(&self, return_values: Vec<Rc<dyn Object>>) {
        todo!()
    }

    pub fn get_inputs(&self) -> Vec<Rc<dyn Object>> {
        todo!()
    }

    pub fn set_outputs(&mut self, values: Vec<Rc<dyn Object>>) {
        todo!()
    }
}

/// ID of a node
pub type NodeId = usize;
/// ID of a program, constructed by an executor
pub type ProgramId = ModulePath;
/// An ID to point to a node in other program
pub type AbsoluteNodeId = (ProgramId, NodeId);
/// ID of a branch of node
pub type NodeBranchId = (NodeId, usize);
/// ID of data connection
pub type ConnectionId = usize;
/// ID of an input socket
pub type InputSocketId = (NodeId, usize);
/// ID of an output socket
pub type OutputSocketId = (NodeId, usize);
/// Path in the module
pub type ModulePath = (Vec<String>, String);

pub trait Node: Debug {
    /// Execution of the node's code. Returns a branch index.
    fn execute(&self, context: &mut ExecutionContext) -> usize;
    /// The class of the node
    fn class(&self) -> Class;
    /// Variants of a node. Internally can be anythingg that can be converted to string
    fn variants(&self) -> Vec<&str>;
    /// Current selected variant of the node
    fn current_variant(&self) -> &str;
    /// Set a specific variant of a node
    fn set_variant(&mut self, variant: &str);
    /// Get information about node's inputs
    fn inputs(&self) -> Vec<InputSocket>;
    /// Get information about node's outputs
    fn outputs(&self) -> Vec<OutputSocket>;
    /// How many branches this node has
    fn branches(&self) -> usize;
}

/// Collection of programs loaded into an executor
pub struct ProgramCollection {
    pub programs: HashMap<ProgramId, Program>,
}

/// A program that contains nodes, classes, constant objects, etc.
#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
    pub nodes: HashMap<NodeId, NodeInfo>,
    pub classes: HashMap<String, ProtoClass>,
    pub branch_edges: HashMap<NodeBranchId, NodeId>,
    pub connections: HashMap<ConnectionId, Connection>,
    pub const_inputs: HashMap<InputSocketId, String>,
}

/// Information about a node stored in the program
#[derive(Debug, Serialize, Deserialize)]
pub struct NodeInfo {
    pub class: ModulePath,
    pub variant: String,
}

/// Connection of a output to an input
#[derive(Debug, Serialize, Deserialize)]
pub struct Connection {
    pub output: OutputSocketId,
    pub input: InputSocketId,
}

/// Description of a class stored in the program
#[derive(Debug, Serialize, Deserialize)]
pub struct ProtoClass {
    pub name: String,
    pub nodes: Vec<NodeId>,
}
