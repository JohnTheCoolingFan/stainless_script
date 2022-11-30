use crate::{
    class::{Class, ProtoClass},
    module::{Module, ModulePath},
    node::{AbsoluteNodeId, Node, NodeBranchId, NodeId, NodeInfo, NodeStorage},
    object::Object,
    socket::{Connection, InputSocketId},
    Plugin,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    rc::Rc,
};

/// ID of a program, constructed by an executor
pub type ProgramId = ModulePath;

#[derive(Debug, Clone)]
pub struct LoadedProgram {
    pub nodes: NodeStorage,
    pub branch_edges: HashMap<NodeBranchId, NodeId>,
    pub connections: HashMap<Connection, Option<Rc<dyn Object>>>,
    pub const_inputs: HashMap<InputSocketId, String>,
}

impl From<&Program> for LoadedProgram {
    fn from(p: &Program) -> Self {
        Self {
            nodes: NodeStorage::default(),
            branch_edges: p.branch_edges.clone(),
            connections: p
                .connections
                .iter()
                .cloned()
                .zip([None].into_iter().cycle())
                .collect(),
            const_inputs: p.const_inputs.clone(),
        }
    }
}

impl LoadedProgram {
    pub fn get_node(&self, node_id: NodeId) -> Option<Rc<dyn Node>> {
        self.nodes.get_node(node_id)
    }

    pub fn get_start_node(&self, name: &str) -> Option<NodeId> {
        for (node_id, node) in &self.nodes.nodes {
            if node.class().name == "start" {
                let variant = node.current_variant();
                let mut parts = variant.split('#');
                parts.next();
                let node_name = parts.next()?;
                if node_name == name {
                    return Some(*node_id)
                }
            }
        }
        None
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

    pub fn get_next_node(&self, current: NodeId, branch: usize) -> Option<NodeId> {
        self.branch_edges
            .get(&NodeBranchId(current, branch))
            .copied()
    }

    // Problem with subroutines: data passing doesn't quite work for subroutines. The  possible
    // solution is to have a dangling connection with an output node ID of u32::MAX and rely on
    // editors to set that  correctly

    /// Set connection values where they originate from given node id
    pub fn set_outputs(&mut self, node_id: NodeId, outputs: Vec<Rc<dyn Object>>) {
        for (i, output) in outputs.into_iter().enumerate() {
            let connections = self
                .connections
                .iter_mut()
                .filter(|(c, _)| c.output.0 .0 == node_id && c.output.0 .1 == i);
            for connection in connections {
                *connection.1 = Some(output.clone())
            }
        }
    }

    /// Get inputs of a node from connections that end in the specified node, as well as collect
    /// const inputs (generally, assumed they are present where it's not  provideds by a
    /// connection. Although the connection mightt be empty, so this is kinda handled.)
    pub fn get_inputs(&self, node_id: NodeId) -> Vec<Option<Rc<dyn Object>>> {
        let connections: BTreeMap<usize, Rc<dyn Object>> = self
            .connections
            .iter()
            .filter_map(|(c, i)| {
                (c.input.0 .0 == node_id).then(|| Some((c.input.0 .1, i.clone()?)))?
            })
            .chain(self.const_inputs.iter().filter_map(|(s, v)| {
                let inputs = self.get_node(node_id).unwrap().inputs();
                let class = &inputs.get(s.0 .1)?.class;
                (s.0 .0 == node_id).then(|| (s.0 .1, class.obj_from_str.unwrap()(v).unwrap()))
            }))
            .collect();
        let mut result: Vec<Option<Rc<dyn Object>>> = Vec::with_capacity(connections.keys().len());
        for i in 0..=connections.iter().map(|(c, _)| *c).max().unwrap_or(0) {
            result.push(connections.get(&i).cloned())
        }
        result
    }
}

#[derive(Debug, Clone, Default)]
pub struct LoadedProgramData {
    pub programs: HashMap<ProgramId, LoadedProgram>,
    pub modules: Module,
}

impl LoadedProgramData {
    pub fn load_plugin(&mut self, plugin: impl Plugin) {
        for (path, class) in plugin.classes() {
            self.modules.insert(path, class);
        }
    }

    pub fn load_program(&mut self, path: &ProgramId, program: &Program) {
        let imported_classes: Vec<(ModulePath, Vec<NodeId>)> = program
            .classes
            .iter()
            .map(|pc| {
                let mut class_path = path.clone();
                class_path.1 = pc.name.clone();
                let class = Class {
                    name: pc.name.clone(),
                    nodes: vec![],
                    obj_from_str: None, // TODO: Add a generic class initializer when
                                        // DeserializeObject is implemented
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

    pub fn load_programs(&mut self, programs: &ProgramCollection) {
        for (path, program) in &programs.programs {
            self.load_program(path, program)
        }
    }

    pub fn get_node(&self, node_id: &AbsoluteNodeId) -> Option<Rc<dyn Node>> {
        let program = self.programs.get(&node_id.0)?;
        program.get_node(node_id.1)
    }

    pub fn get_start_node(&self, program_id: ProgramId, name: &str) -> Option<AbsoluteNodeId> {
        let program = self.programs.get(&program_id)?;
        program.get_start_node(name).map(|i| AbsoluteNodeId(program_id.clone(), i))
    }

    pub fn get_next_node(&self, node_id: &AbsoluteNodeId, branch: usize) -> Option<AbsoluteNodeId> {
        let AbsoluteNodeId(program_path, node_id) = node_id;
        let program = self.programs.get(program_path)?;
        let next_node_id = program.get_next_node(*node_id, branch)?;
        Some(AbsoluteNodeId(program_path.clone(), next_node_id))
    }

    pub fn set_outputs(&mut self, node_id: &AbsoluteNodeId, outputs: Vec<Rc<dyn Object>>) {
        self.programs
            .get_mut(&node_id.0)
            .unwrap()
            .set_outputs(node_id.1, outputs)
    }

    pub fn get_inputs(&self, node_id: &AbsoluteNodeId) -> Vec<Option<Rc<dyn Object>>> {
        self.programs.get(&node_id.0).unwrap().get_inputs(node_id.1)
    }

    pub fn get_class(&self, path: ModulePath) -> Option<&Class> {
        self.modules.get_class(&path)
    }
}

/// Collection of programs loaded into an executor
#[derive(Debug, Clone, Default)]
pub struct ProgramCollection {
    pub programs: HashMap<ProgramId, Program>,
}

/// A program that contains nodes, classes, constant objects, etc.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Program {
    /// What programs to load for this program to work
    pub imports: Option<Vec<String>>,
    /// Collection of all nodes placed in the program
    pub nodes: HashMap<NodeId, NodeInfo>,
    /// Used for setting the position in a graphical view of the program. Third value is z-index.
    pub node_positions: Option<HashMap<NodeId, (f32, f32, f32)>>,
    /// All classes defined in a program
    pub classes: Vec<ProtoClass>,
    /// Execution order connections between nodes
    pub branch_edges: HashMap<NodeBranchId, NodeId>,
    /// Data connections between nodes
    pub connections: HashSet<Connection>,
    /// COnstant inputs that are not getting a value through a connection
    pub const_inputs: HashMap<InputSocketId, String>,
}
