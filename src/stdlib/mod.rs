use crate::{class::Class, module::ModulePath, Plugin};
use std::collections::HashMap;

mod any_type;
mod array_type;
mod bool_type;
mod dict_type;
mod flow_nodes;
mod if_node;
mod nop_node;
mod number_type;
mod print_node;
mod string_type;
mod subroutine;
mod variable_node;

pub use any_type::*;
pub use array_type::*;
pub use bool_type::*;
pub use flow_nodes::*;
pub use if_node::*;
pub use nop_node::*;
pub use number_type::*;
pub use print_node::*;
pub use string_type::*;
pub use subroutine::*;
pub use variable_node::*;

pub struct StdPlugin;

impl Plugin for StdPlugin {
    fn classes(&self) -> HashMap<ModulePath, Class> {
        [
            any_class(),
            array_class(),
            bool_class(),
            start_node_class(),
            end_node_class(),
            if_node_class(),
            nop_node_class(),
            number_class(),
            print_class(),
            string_class(),
            subroutine_class(),
            variable_get_class(),
            variable_set_class(),
        ]
        .into_iter()
        .map(|cl| (ModulePath(vec!["std".into()], cl.name.clone()), cl))
        .collect()
    }
}
