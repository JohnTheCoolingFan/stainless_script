use class::Class;
use module::ModulePath;
use node::{AbsoluteNodeId, Node};
use object::Object;
use program::{LoadedProgramData, Program, ProgramCollection};
use socket::InputSocket;
use std::{collections::HashMap, fmt::Debug, rc::Rc};

pub mod class;
pub mod module;
pub mod node;
pub mod object;
pub mod program;
pub mod socket;
pub mod stdlib;

pub trait Plugin {
    fn classes(&self) -> HashMap<ModulePath, Class>;
}

/// Initialize with `Default::default` or `new_with_loaded` if you have already loaded data, load plugins and programs through `load_plugin` and
/// `load_program`, start execution with `start_execution`, execute step-by-step with `execute_step` (will advance automatically)
#[derive(Debug, Clone, Default)]
pub struct Executor {
    node_stack: Vec<AbsoluteNodeId>,
    loaded: LoadedProgramData,
    auto_execution: bool,
    stop_point: Option<AbsoluteNodeId>,
}

impl Executor {
    fn execute_subroutine(&mut self, node_id: AbsoluteNodeId, input_values: Vec<Rc<dyn Object>>) {
        self.node_stack.push(node_id);
        self.set_node_outputs(input_values);
    }

    fn finish_subroutine(&mut self, return_values: Vec<Rc<dyn Object>>) {
        self.node_stack.pop();
        self.set_node_outputs(return_values);
    }

    fn get_node_inputs(&self) -> Vec<Rc<dyn Object>> {
        self.loaded
            .get_inputs(&self.current_node())
            .into_iter()
            .collect::<Option<Vec<Rc<dyn Object>>>>()
            .unwrap()
    }

    fn set_node_outputs(&mut self, values: Vec<Rc<dyn Object>>) {
        self.loaded.set_outputs(&self.current_node(), values)
    }

    fn current_node(&self) -> AbsoluteNodeId {
        self.node_stack.last().unwrap().clone()
    }

    pub fn execute_step(&mut self) {
        let node_id = self.current_node();
        let node = self.get_node_by_id(node_id);
        let inputs = node.inputs();
        let mut context = ExecutionContext::new(self, inputs);
        let branch = node.execute(&mut context);
        self.advance(branch);
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
        let start_node = self
            .loaded
            .get_start_node(ModulePath(vec![], "__main__".into()), "main");
        self.node_stack.push(start_node.unwrap());
        self.execution_loop();
    }

    fn execution_loop(&mut self) {
        while !self.node_stack.is_empty() && self.auto_execution {
            self.execute_step();
            if let Some(node) = &self.stop_point {
                if &self.current_node() == node {
                    self.auto_execution = false
                }
            }
        }
    }

    pub fn resume_auto(&mut self) {
        self.auto_execution = true;
        self.execution_loop();
    }

    pub fn resume_until(&mut self, node: AbsoluteNodeId) {
        self.stop_point = Some(node);
        self.auto_execution = true;
        self.execution_loop();
    }

    pub fn new_with_loaded(loaded: LoadedProgramData) -> Self {
        Self {
            node_stack: Vec::default(),
            loaded,
            auto_execution: bool::default(),
            stop_point: None,
        }
    }
}

/// Context for nodes. Nodes get their inputs, set their ouputs, redirect to subroutine and other
/// through this context.
pub struct ExecutionContext<'a> {
    executor: &'a mut Executor,
    inputs: Vec<InputSocket>,
}

impl<'a> ExecutionContext<'a> {
    fn new(executor: &'a mut Executor, inputs: Vec<InputSocket>) -> Self {
        Self { executor, inputs }
    }
    /// Redirect execution to a subroutine. Returns whatever end node receives.
    pub fn execute_subroutine(&mut self, start: AbsoluteNodeId, input_values: Vec<Rc<dyn Object>>) {
        self.executor.execute_subroutine(start, input_values);
    }

    /// Finish executing subroutine, return to caller.
    pub fn finish_subroutine(&mut self, return_values: Vec<Rc<dyn Object>>) {
        self.executor.finish_subroutine(return_values);
    }

    pub fn get_inputs(&self) -> Vec<Rc<dyn Object>> {
        self.executor
            .get_node_inputs()
            .into_iter()
            .zip(self.inputs.iter())
            .map(|(iv, ec)| iv.cast_to(&ec.class))
            .collect()
    }

    pub fn set_outputs(&mut self, values: Vec<Rc<dyn Object>>) {
        self.executor.set_node_outputs(values)
    }
}
