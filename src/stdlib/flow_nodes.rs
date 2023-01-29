use crate::{
    class::Class,
    node::Node,
    socket::{InputSocket, OutputSocket},
    ExecutionContext,
};
use std::rc::Rc;

pub fn start_node_class() -> Class {
    Class {
        name: "start".into(),
        nodes: vec![Rc::new(StartNode {
            outputs: vec![],
            name: "default".into(),
        }) as Rc<dyn Node>],
        obj_from_str: None,
    }
}

pub fn end_node_class() -> Class {
    Class {
        name: "end".into(),
        nodes: vec![Rc::new(EndNode(vec![])) as Rc<dyn Node>],
        obj_from_str: None,
    }
}

/// Start of a program or subroutine
#[derive(Debug, Clone)]
pub struct StartNode {
    outputs: Vec<OutputSocket>,
    name: String,
}

impl Node for StartNode {
    fn execute(&self, _context: &mut ExecutionContext) -> usize {
        0
    }

    fn class(&self) -> Class {
        start_node_class()
    }

    fn variants(&self) -> Vec<std::borrow::Cow<'_, str>> {
        vec!["start#default#[]".into(), self.current_variant()]
    }

    fn current_variant(&self) -> std::borrow::Cow<'_, str> {
        format!(
            "start#{}#{}",
            self.name,
            ron::to_string(&self.outputs).unwrap()
        )
        .into()
    }

    fn set_variant(&mut self, variant: &str) {
        let mut parts = variant.split('#');
        parts.next();
        let name = String::from(parts.next().unwrap());
        let outputs = ron::from_str(parts.next().unwrap()).unwrap();
        self.name = name;
        self.outputs = outputs
    }

    fn inputs(&self) -> Vec<InputSocket> {
        vec![]
    }

    fn outputs(&self) -> Vec<OutputSocket> {
        self.outputs.clone()
    }

    fn clone_node(&self) -> Rc<dyn Node> {
        Rc::new(self.clone()) as Rc<dyn Node>
    }

    fn accepts_arbitrary_variants(&self) -> bool {
        true
    }
}

/// End of a program or subroutine
#[derive(Debug, Clone)]
pub struct EndNode(Vec<InputSocket>);

impl Node for EndNode {
    fn execute(&self, context: &mut ExecutionContext) -> usize {
        let inputs = context.get_inputs();
        context.finish_subroutine(inputs);
        0
    }

    fn class(&self) -> Class {
        end_node_class()
    }

    fn variants(&self) -> Vec<std::borrow::Cow<'_, str>> {
        vec!["end[]".into(), self.current_variant()]
    }

    fn current_variant(&self) -> std::borrow::Cow<'_, str> {
        format!("end{}", ron::to_string(&self.0).unwrap()).into()
    }

    fn set_variant(&mut self, variant: &str) {
        self.0 = ron::from_str(variant.strip_prefix("end").unwrap()).unwrap()
    }

    fn inputs(&self) -> Vec<InputSocket> {
        self.0.clone()
    }

    fn outputs(&self) -> Vec<OutputSocket> {
        vec![]
    }

    fn clone_node(&self) -> Rc<dyn Node> {
        Rc::new(self.clone()) as Rc<dyn Node>
    }

    fn accepts_arbitrary_variants(&self) -> bool {
        true
    }
}
