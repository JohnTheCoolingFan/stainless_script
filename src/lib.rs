use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap},
    error::Error,
    fmt::{Debug, Display},
    num::ParseIntError,
    rc::Rc,
    str::FromStr,
    sync::Mutex,
};
use thiserror::Error;

pub mod stdlib;

/// Used to index items across programs/packages. Built with executor upon loading programs.
#[derive(Debug, Clone, Default)]
pub struct Module {
    pub items: HashMap<String, ModuleItem>,
}

impl Module {
    fn insert(&mut self, path: ModulePath, item: impl Into<ModuleItem>) -> &mut ModuleItem {
        let mut current_segment = &mut self.items;
        for segment in path.0 {
            let ModuleItem::Module(next_segment) = current_segment.entry(segment.clone()).or_insert_with(|| ModuleItem::Module(Module::default())) else {unreachable!()};
            current_segment = &mut next_segment.items;
        }
        current_segment.entry(path.1).or_insert_with(|| item.into())
    }

    fn get_class(&self, path: &ModulePath) -> Option<&Class> {
        let mut current_segment = &self.items;
        for segment in &path.0 {
            let ModuleItem::Module(next_segment) = current_segment.get(segment).unwrap() else { return None};
            current_segment = &next_segment.items;
        }
        let ModuleItem::Class(class) = current_segment.get(&path.1).unwrap() else {return None};
        Some(class)
    }

    fn get_class_mut(&mut self, path: &ModulePath) -> Option<&mut Class> {
        let mut current_segment = &mut self.items;
        for segment in &path.0 {
            let ModuleItem::Module(next_segment) = current_segment.get_mut(segment).unwrap() else {return None};
            current_segment = &mut next_segment.items;
        }
        let ModuleItem::Class(class) = current_segment.get_mut(&path.1).unwrap() else {return None};
        Some(class)
    }
}

#[derive(Debug, Clone)]
pub enum ModuleItem {
    /// Not implemented yet, not parsed from the program file
    Constant(Rc<dyn Object>),
    Class(Class),
    Module(Module),
}

impl From<Class> for ModuleItem {
    fn from(c: Class) -> Self {
        Self::Class(c)
    }
}

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

pub trait ObjectFromStr {
    fn from_str(s: &str) -> Result<Rc<dyn Object>, Box<dyn Error + Send + Sync>>
    where
        Self: Sized;
}

impl<T: 'static + FromStr + Object> ObjectFromStr for T
where
    T::Err: 'static + Error + Send + Sync,
{
    fn from_str(s: &str) -> Result<Rc<dyn Object>, Box<dyn Error + Send + Sync>> {
        <Self as FromStr>::from_str(s)
            .map_err(Into::into)
            .map(|o| Rc::new(o) as Rc<dyn Object>)
    }
}

/// The object of a data type. Data type is derived from the object's class. Methods specified here
/// are for use in nodes mostly.
pub trait Object: Display + Debug + ObjectFromStr {
    fn class(&self) -> Class;
    /// Since Object requires Display, this has little use and is implemented  through ToString,
    /// which is implemented for all types implementing Display. Left for consistency with
    /// as_number and other methods
    fn as_string(&self) -> String {
        self.to_string()
    }
    /// Convert to number
    fn as_number(&self) -> f64;
    /// Convert to boolean
    fn as_bool(&self) -> bool;
    /// Suggested implementation: Have a `HashMap<String, Rc<dyn Object>>` to manage fields.
    /// Default implementation is `unimplemented!()` because most types don't have fields.
    fn get_field(&self, _field: Rc<dyn Object>) -> Rc<dyn Object> {
        unimplemented!()
    }
    /// Suggested implementation: use `String::from` to convert `&str` to `String` and use that as
    /// insertion key. Default implementation is `unimplemented!()` because most types don't have
    /// fields.
    fn set_field(&mut self, _field: Rc<dyn Object>, _value: Rc<dyn Object>) {
        unimplemented!()
    }
}

/// Input of a node.
#[derive(Debug, Clone)]
pub struct InputSocket {
    /// This is merely a type suggestion used to hint what type is expected. Can be used by IDEs to
    /// force only certain type in a connection, requiring to do a proper conversion.
    pub class: Class,
}

/// Output of a node
#[derive(Debug, Clone)]
pub struct OutputSocket {
    pub class: Class,
}

#[derive(Debug, Clone, Default)]
pub struct NodeStorage {
    nodes: BTreeMap<NodeId, Rc<dyn Node>>,
    next_vacant: NodeId,
}

impl NodeStorage {
    fn get_node(&self, node_id: NodeId) -> Option<Rc<dyn Node>> {
        self.nodes.get(&node_id).cloned()
    }

    fn remove_node(&mut self, node_id: NodeId) -> Option<Rc<dyn Node>> {
        let node = self.nodes.remove(&node_id);
        if node_id < self.next_vacant {
            self.next_vacant = node_id
        }
        node
    }

    fn insert_node(&mut self, node: Rc<dyn Node>) -> NodeId {
        let mut node_id = self.next_vacant;
        self.nodes.insert(node_id, node);
        while self.nodes.get(&node_id).is_some() {
            node_id += 1;
        }
        self.next_vacant = node_id;
        node_id
    }

    fn insert_node_at(&mut self, node_id: NodeId, node: Rc<dyn Node>) {
        self.nodes.insert(node_id, node);
        while self.nodes.get(&self.next_vacant).is_some() {
            self.next_vacant += 1;
        }
    }
}

#[derive(Debug, Clone)]
pub struct LoadedProgram {
    nodes: NodeStorage,
    branch_edges: HashMap<NodeBranchId, NodeId>,
    connections: HashMap<ConnectionId, Connection>,
    const_inputs: HashMap<InputSocketId, String>,
}

impl From<&Program> for LoadedProgram {
    fn from(p: &Program) -> Self {
        Self {
            nodes: NodeStorage::default(),
            branch_edges: p.branch_edges.clone(),
            connections: p.connections.clone(),
            const_inputs: p.const_inputs.clone(),
        }
    }
}

impl LoadedProgram {
    pub fn get_node(&self, node_id: NodeId) -> Option<Rc<dyn Node>> {
        self.nodes.get_node(node_id)
    }

    pub fn insert_node(&mut self, node: Rc<dyn Node>) -> NodeId {
        self.nodes.insert_node(node)
    }

    pub fn remove_node(&mut self, node_id: NodeId) {
        self.nodes.remove_node(node_id);
    }

    pub fn insert_raw_node_at(
        &mut self,
        node_id: NodeId,
        node: &NodeInfo,
        class: &Class,
    ) -> Rc<dyn Node> {
        assert_eq!(node.class.1, class.name);
        let mut loaded_node = class.nodes[node.idx].clone_node();
        Rc::get_mut(&mut loaded_node)
            .unwrap()
            .set_variant(&node.variant);
        self.nodes.insert_node_at(node_id, Rc::clone(&loaded_node));
        loaded_node as Rc<dyn Node>
    }

    fn get_next_node(&self, current: NodeId, branch: usize) -> Option<NodeId> {
        self.branch_edges
            .get(&NodeBranchId(current, branch))
            .copied()
    }
}

#[derive(Debug, Clone, Default)]
pub struct LoadedProgramData {
    programs: HashMap<ProgramId, LoadedProgram>,
    modules: Module,
}

impl LoadedProgramData {
    fn load_plugin(&mut self, plugin: impl Plugin) {
        for (path, class) in plugin.classes() {
            self.modules.insert(path, class);
        }
    }

    fn load_program(&mut self, path: &ProgramId, program: &Program) {
        let imported_classes: Vec<(ModulePath, Vec<NodeId>)> = program
            .classes
            .iter()
            .map(|pc| {
                let mut class_path = path.clone();
                class_path.1 = pc.name.clone();
                let class = Class {
                    name: pc.name.clone(),
                    nodes: vec![],
                    obj_from_str: None,
                };
                self.modules.insert(class_path.clone(), class);
                (class_path, pc.nodes.clone())
            })
            .collect();
        let inserted_program = self
            .programs
            .entry(path.clone())
            .or_insert_with(|| program.into());
        for (node_id, node) in &program.nodes {
            let class = self.modules.get_class(&node.class).unwrap();
            inserted_program.insert_raw_node_at(*node_id, node, class);
        }
        for (class_path, node_ids) in imported_classes {
            let class = self.modules.get_class_mut(&class_path).unwrap();
            let loaded_nodes = node_ids
                .iter()
                .map(|id| inserted_program.get_node(*id).unwrap())
                .collect();
            class.nodes = loaded_nodes;
        }
    }

    fn load_programs(&mut self, programs: &ProgramCollection) {
        for (path, program) in &programs.programs {
            self.load_program(path, program)
        }
    }

    fn get_node(&self, node_id: &AbsoluteNodeId) -> Option<Rc<dyn Node>> {
        let program = self.programs.get(&node_id.0)?;
        program.get_node(node_id.1)
    }

    fn get_next_node(&self, node_id: &AbsoluteNodeId, branch: usize) -> Option<AbsoluteNodeId> {
        let AbsoluteNodeId(program_path, node_id) = node_id;
        let program = self.programs.get(program_path)?;
        let next_node_id = program.get_next_node(*node_id, branch)?;
        Some(AbsoluteNodeId(program_path.clone(), next_node_id))
    }
}

/// Initialize with Default::default, load plugins and programs through load_plugin and
/// load_program, start execution with start_execution, execute step-by-step with execute_current_node (will advance automatically)
#[derive(Debug, Clone, Default)]
pub struct Executor {
    node_stack: Vec<AbsoluteNodeId>,
    node_outputs: HashMap<AbsoluteNodeId, Vec<Rc<dyn Object>>>,
    loaded: LoadedProgramData,
    auto_execution: bool,
}

pub trait Plugin {
    fn classes(&self) -> HashMap<ModulePath, Class>;
}

impl Executor {
    fn finish_subroutine(&mut self, return_values: Vec<Rc<dyn Object>>) {
        self.node_stack.pop();
        self.set_node_outputs(return_values);
    }

    fn execute_subroutine(&mut self, node_id: AbsoluteNodeId, input_values: Vec<Rc<dyn Object>>) {
        self.node_stack.push(node_id);
        self.set_node_outputs(input_values);
    }

    fn get_node_inputs(&self) -> Vec<Rc<dyn Object>> {
        todo!()
    }

    fn set_node_outputs(&mut self, values: Vec<Rc<dyn Object>>) {
        self.node_outputs.insert(self.current_node(), values);
    }

    fn current_node(&self) -> AbsoluteNodeId {
        self.node_stack.last().unwrap().clone()
    }

    pub fn execute_current_node(self_mutex: Mutex<Self>) {
        let (node, inputs) = {
            let lock = self_mutex.lock().unwrap();
            let node_id = lock.current_node();
            let inputs = lock.get_node_inputs();
            let node = lock.get_node_by_id(node_id);
            drop(lock);
            (node, inputs)
        };
        let mut context = ExecutionContext::new(&self_mutex, inputs);
        let branch = node.execute(&mut context);
        {
            let mut lock = self_mutex.lock().unwrap();
            lock.set_node_outputs(context.node_outputs.unwrap());
            lock.advance(branch)
        }
    }

    fn get_node_by_id(&self, node_id: AbsoluteNodeId) -> Rc<dyn Node> {
        self.loaded.get_node(&node_id).unwrap()
    }

    fn advance(&mut self, branch: usize) {
        let node_id = self.node_stack.pop().unwrap();
        let next_node_id = self.get_next_node(node_id, branch);
        self.node_stack.push(next_node_id)
    }

    fn get_next_node(&self, current: AbsoluteNodeId, branch: usize) -> AbsoluteNodeId {
        self.loaded.get_next_node(&current, branch).unwrap()
    }

    pub fn load_program(&mut self, program: Program, path: ModulePath) {
        self.loaded.load_program(&path, &program)
    }

    pub fn load_programs(&mut self, programs: ProgramCollection) {
        self.loaded.load_programs(&programs)
    }

    pub fn load_plugin(&mut self, plugin: impl Plugin) {
        self.loaded.load_plugin(plugin)
    }

    pub fn start_execution(&mut self, auto: bool) {
        self.auto_execution = auto;
        todo!()
    }
}

/// Context for nodes. Nodes get their inputs, set their ouputs, redirect to subroutine and other
/// through this context.
pub struct ExecutionContext<'a> {
    executor: &'a Mutex<Executor>,
    node_inputs: Vec<Rc<dyn Object>>,
    node_outputs: Option<Vec<Rc<dyn Object>>>,
}

impl<'a> ExecutionContext<'a> {
    fn new(executor: &'a Mutex<Executor>, node_inputs: Vec<Rc<dyn Object>>) -> Self {
        Self {
            executor,
            node_inputs,
            node_outputs: None,
        }
    }
    /// Redirect execution to a subroutine. Returns whatever end node receives.
    pub fn execute_subroutine(&self, start: AbsoluteNodeId, input_values: Vec<Rc<dyn Object>>) {
        self.executor
            .lock()
            .unwrap()
            .execute_subroutine(start, input_values);
    }

    /// Finish executing subroutine, return to caller.
    pub fn finish_subroutine(&self, return_values: Vec<Rc<dyn Object>>) {
        self.executor
            .lock()
            .unwrap()
            .finish_subroutine(return_values);
    }

    pub fn get_inputs(&self) -> Vec<Rc<dyn Object>> {
        self.node_inputs.clone()
    }

    pub fn set_outputs(&mut self, values: Vec<Rc<dyn Object>>) {
        self.node_outputs = Some(values)
    }
}

/// ID of a node
pub type NodeId = u32;

/// ID of data connection
pub type ConnectionId = u32;

/// ID of a program, constructed by an executor
pub type ProgramId = ModulePath;

/// An ID to point to a node in other program
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct AbsoluteNodeId(ProgramId, NodeId);

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
pub struct NodeBranchId(NodeId, usize);

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

/// ID of a socket, either input or output
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SocketId(NodeId, u32);

impl From<&SocketId> for u64 {
    fn from(s: &SocketId) -> Self {
        (s.0 as u64) << 32 | s.1 as u64
    }
}

impl From<u64> for SocketId {
    fn from(n: u64) -> Self {
        let socket_idx: u32 = (((1_u64 << 33) - 1) & n) as u32;
        let node_id: NodeId = ((((1_u64 << 33) - 1) << 32) & n) as NodeId;
        Self(node_id, socket_idx)
    }
}

impl Serialize for SocketId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        u64::from(self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SocketId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(SocketId::from(u64::deserialize(deserializer)?))
    }
}

/// ID of an input socket
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct InputSocketId(SocketId);

/// ID of an output socket
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct OutputSocketId(SocketId);

/// Path in the module
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ModulePath(Vec<String>, String);

impl Display for ModulePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = self.0.clone();
        res.push(self.1.clone());
        write!(f, "{}", res.join("."))
    }
}

impl FromStr for ModulePath {
    type Err = ModulePathParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut seq: Vec<String> = s.split('.').map(String::from).collect();
        let item_name = seq.pop().ok_or(ModulePathParseError::NotEnoughItems)?;
        Ok(Self(seq, item_name))
    }
}

#[derive(Debug, Clone, Error)]
pub enum ModulePathParseError {
    #[error("Not enough items")]
    NotEnoughItems,
}

impl Serialize for ModulePath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut ret = self.0.clone();
        ret.push(self.1.clone());
        ret.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ModulePath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut seq = Vec::<String>::deserialize(deserializer)?;
        let item_name = seq.pop().unwrap();
        Ok(Self(seq, item_name))
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

/// Collection of programs loaded into an executor
#[derive(Debug, Clone, Default)]
pub struct ProgramCollection {
    pub programs: HashMap<ProgramId, Program>,
}

/// A program that contains nodes, classes, constant objects, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub imports: Vec<String>, // Is this even useful?
    pub nodes: HashMap<NodeId, NodeInfo>,
    pub classes: Vec<ProtoClass>,
    pub branch_edges: HashMap<NodeBranchId, NodeId>,
    pub connections: HashMap<ConnectionId, Connection>,
    pub const_inputs: HashMap<InputSocketId, String>,
}

/// Information about a node stored in the program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub class: ModulePath,
    pub idx: usize,
    pub variant: String,
}

/// Connection of a output to an input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub output: OutputSocketId,
    pub input: InputSocketId,
}

/// Description of a class stored in the program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtoClass {
    pub name: String,
    pub nodes: Vec<NodeId>,
}
