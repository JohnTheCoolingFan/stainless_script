use std::{collections::BTreeMap, fmt::Display, ops::Deref, rc::Rc, str::FromStr};

use stainless_script_derive::{ObjectEq, ObjectOrd, ObjectPartialEq, ObjectPartialOrd};
use thiserror::Error;

use crate::{
    class::Class,
    object::{Object, ObjectEq, ObjectFromStr, ObjectOrd, ObjectPartialEq, ObjectPartialOrd},
};

use super::{AnyType, Array};

#[derive(Debug, Clone)]
struct DictVal(Rc<dyn Object>);

impl DictVal {
    fn from_ron(val: &ron::Value) -> Self {
        match val {
            ron::Value::Bool(b) => Self(Rc::new(*b) as Rc<dyn Object>),
            ron::Value::Char(c) => Self(Rc::new(c.to_string()) as Rc<dyn Object>),
            ron::Value::Map(_) => Self(Rc::new(Self::dict_from_map(val))),
            ron::Value::Number(n) => Self(Rc::new(n.into_f64())),
            ron::Value::Option(opt) => opt
                .as_ref()
                .map(|v| Self::from_ron(v))
                .unwrap_or_else(|| Self(<AnyType as ObjectFromStr>::from_str("").unwrap())),
            ron::Value::String(s) => Self(Rc::new(s.clone())),
            ron::Value::Seq(seq) => Self(Rc::new(Self::array_from_seq(seq))),
            _ => Self(<AnyType as ObjectFromStr>::from_str("").unwrap()),
        }
    }

    fn dict_from_map(val: &ron::Value) -> Dict {
        let val = val.clone();
        let rust_map: BTreeMap<ron::Value, ron::Value> = val.into_rust().unwrap();
        Dict(
            rust_map
                .into_iter()
                .map(|(k, v)| (DictVal::from_ron(&k), DictVal::from_ron(&v)))
                .collect(),
        )
    }

    fn array_from_seq(seq: &[ron::Value]) -> Array {
        Array(seq.iter().map(|v| DictVal::from_ron(v).0).collect())
    }
}

impl Deref for DictVal {
    type Target = Rc<dyn Object>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for DictVal {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(Rc::clone(&other.0))
    }
}

impl PartialOrd for DictVal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(Rc::clone(&other.0))
    }
}

impl Eq for DictVal {}

impl Ord for DictVal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(Rc::clone(&other.0))
    }
}

pub fn dict_class() -> Class {
    Class {
        name: "dict".into(),
        nodes: vec![], // TODO: dict constructor (from pairs of values?)
        obj_from_str: Some(<Dict as ObjectFromStr>::from_str),
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    ObjectPartialEq,
    ObjectPartialOrd,
    ObjectEq,
    ObjectOrd,
)]
pub struct Dict(BTreeMap<DictVal, DictVal>);

impl FromStr for Dict {
    type Err = DictParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed_map = ron::from_str::<BTreeMap<ron::Value, ron::Value>>(s)?;
        Ok(Self(
            parsed_map
                .into_iter()
                .map(|(k, v)| (DictVal::from_ron(&k), DictVal::from_ron(&v)))
                .collect(),
        ))
    }
}

#[derive(Debug, Clone, Error)]
pub enum DictParseError {
    //#[error("{0}")]
    //ObjectParse(<AnyType as FromStr>::Err),
    #[error("{0}")]
    DeserializingError(ron::error::SpannedError),
}

impl From<ron::error::SpannedError> for DictParseError {
    fn from(value: ron::error::SpannedError) -> Self {
        Self::DeserializingError(value)
    }
}

impl Display for Dict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.0
                .iter()
                .map(|(k, v)| format!("{}: {}", **k, **v))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl Object for Dict {
    fn class(&self) -> Class {
        dict_class()
    }

    fn as_number(&self) -> f64 {
        panic!("Cannot convert dictionary to number")
    }

    fn as_bool(&self) -> bool {
        !self.0.is_empty()
    }

    fn get_field(&self, field: Rc<dyn Object>) -> Rc<dyn Object> {
        let key = DictVal(field);
        if let Some(val) = self.0.get(&key) {
            Rc::clone(val)
        } else {
            match key.as_string().as_str() {
                "keys" => Rc::new(Array(self.0.keys().map(|v| Rc::clone(v)).collect())),
                "values" => Rc::new(Array(self.0.values().map(|v| Rc::clone(v)).collect())),
                _ => panic!("Unknown field: {}", key.0),
            }
        }
    }

    fn set_field(&mut self, field: Rc<dyn Object>, value: Rc<dyn Object>) {
        let new_key = DictVal(field);
        let new_val = DictVal(value);
        self.0.remove(&new_key);
        self.0.insert(new_key, new_val);
    }
}
