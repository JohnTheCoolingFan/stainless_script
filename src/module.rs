use crate::{class::Class, object::Object};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display, rc::Rc, str::FromStr};
use thiserror::Error;

/// Path in the module
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModulePath(pub Vec<String>, pub String);

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

/// Used to index items across programs/packages. Built with executor upon loading programs.
#[derive(Debug, Clone, Default)]
pub struct Module {
    pub items: HashMap<String, ModuleItem>,
}

impl Module {
    pub fn insert(&mut self, path: ModulePath, item: impl Into<ModuleItem>) -> &mut ModuleItem {
        let mut current_segment = &mut self.items;
        for segment in path.0 {
            let ModuleItem::Module(next_segment) = current_segment.entry(segment.clone()).or_insert_with(|| ModuleItem::Module(Module::default())) else {panic!("Attempt to index non-module item.")};
            current_segment = &mut next_segment.items;
        }
        current_segment.entry(path.1).or_insert_with(|| item.into())
    }

    pub fn get_class(&self, path: &ModulePath) -> Option<&Class> {
        let mut current_segment = &self.items;
        for segment in &path.0 {
            let ModuleItem::Module(next_segment) = current_segment.get(segment)? else { return None};
            current_segment = &next_segment.items;
        }
        let ModuleItem::Class(class) = current_segment.get(&path.1)? else {return None};
        Some(class)
    }

    pub fn get_class_mut(&mut self, path: &ModulePath) -> Option<&mut Class> {
        let mut current_segment = &mut self.items;
        for segment in &path.0 {
            let ModuleItem::Module(next_segment) = current_segment.get_mut(segment)? else {return None};
            current_segment = &mut next_segment.items;
        }
        let ModuleItem::Class(class) = current_segment.get_mut(&path.1)? else {return None};
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
