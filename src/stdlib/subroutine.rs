use crate::{
    class::Class,
    module::ModulePath,
    node::{AbsoluteNodeId, Node, NodeId},
    socket::{InputSocket, OutputSocket},
};
use std::{borrow::Cow, rc::Rc, str::FromStr};

/// The node provided should be cloned and set the proper ids before any use. By default, all ids
/// are at their max values
pub fn subroutine_class() -> Class {
    let empty_path = ModulePath(vec![], String::new());
    Class {
        name: "subroutine".into(),
        nodes: vec![Rc::new(Subroutine(
            AbsoluteNodeId(empty_path.clone(), NodeId::MAX),
            AbsoluteNodeId(empty_path, NodeId::MAX),
        )) as Rc<dyn Node>],
        obj_from_str: None,
    }
}

/// This is a special class that tells to look to the node ids provided in the class for inputs
pub fn subroutine_input_class(id: &AbsoluteNodeId) -> Class {
    Class {
        name: format!("subroutine_input@{}", id),
        nodes: vec![],
        obj_from_str: None,
    }
}

/// This is a special class that tells to look to the node ids provided in the class for outputs
pub fn subroutine_output_class(id: &AbsoluteNodeId) -> Class {
    Class {
        name: format!("subroutine_output@{}", id),
        nodes: vec![],
        obj_from_str: None,
    }
}

// The end node id is kinda unused... It would be awesome to guarantee that the subroutine doesn't
// branch out to some different end
#[derive(Debug, Clone)]
pub struct Subroutine(AbsoluteNodeId, AbsoluteNodeId);

impl Node for Subroutine {
    fn execute(&self, context: &mut crate::ExecutionContext) -> usize {
        let inputs = context.get_inputs();
        context.execute_subroutine(self.0.clone(), inputs);
        0
    }

    fn class(&self) -> Class {
        subroutine_class()
    }

    /// Format: subroutine:<start_node_id>:<end_node_id>
    fn variants(&self) -> Vec<Cow<'_, str>> {
        vec![self.current_variant()]
    }

    /// Format: subroutine:<start_node_id>:<end_node_id>
    fn current_variant(&self) -> Cow<'_, str> {
        format!("subroutine:{}:{}", self.0, self.1).into()
    }

    /// Format: subroutine@<start_node_id>:<end_node_id>
    fn set_variant(&mut self, variant: &str) {
        let mut ids = variant.strip_prefix("subroutine:").unwrap().split(':');
        let id_start = ids.next().unwrap();
        let id_end = ids.next().unwrap();
        self.0 = AbsoluteNodeId::from_str(id_start).unwrap();
        self.1 = AbsoluteNodeId::from_str(id_end).unwrap()
    }

    fn inputs(&self) -> Vec<InputSocket> {
        vec![InputSocket {
            class: subroutine_input_class(&self.0),
        }]
    }

    fn outputs(&self) -> Vec<OutputSocket> {
        vec![OutputSocket {
            class: subroutine_output_class(&self.1),
        }]
    }

    fn clone_node(&self) -> Rc<dyn Node> {
        Rc::new(self.clone()) as Rc<dyn Node>
    }

    fn accepts_arbitrary_variants(&self) -> bool {
        true
    }
}
