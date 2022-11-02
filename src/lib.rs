use std::{fmt::{Display, Debug}, collections::HashMap, rc::Rc};
use serde::{Serialize, Deserialize};

pub struct Module {
    pub items: HashMap<String, ModuleItem>
}

pub enum ModuleItem {
    Constant(Rc<dyn Object>),
    Class(Class),
    Module(Module)
}

#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    /// Default node to be placed when selecting a class to put. Usually a constructor method.
    pub default_node: Rc<dyn Node>
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Eq for Class {}

pub trait Object: Display {
    fn class(&self) -> Class;
}

pub struct InputSocket {
    pub class: Class
}

pub struct OutputSocket {
    pub class: Class
}

pub struct ExecutionContext {

}

pub type NodeId = usize;
pub type NodeBranchId = (NodeId, usize);
pub type ConnectionId = usize;
pub type InputSocketId = (NodeId, usize);
pub type OutputSocketId = (NodeId, usize);
pub type ModulePath = (Vec<String>, String);

pub trait Node: Debug {
    fn execute(&self, context: &mut ExecutionContext) -> usize;
    fn class(&self) -> Class;
    fn id(&self) -> NodeId;
    fn variants(&self) -> Vec<&str>;
    fn current_variant(&self) -> &str;
    fn set_variant(&mut self, variant: &str);
    fn inputs(&self) -> Vec<InputSocket>;
    fn outputs(&self) -> Vec<OutputSocket>;
    fn branches(&self) -> usize;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
    pub nodes: HashMap<NodeId, NodeInfo>,
    pub branch_edges: HashMap<NodeBranchId, NodeId>,
    pub connections: HashMap<ConnectionId, Connection>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeInfo {
    pub class: ModulePath,
    pub variant: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Connection {
    pub input: InputSocketId,
    pub output: OutputSocketId
}
