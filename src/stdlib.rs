use crate::{Class, ExecutionContext, InputSocket, Node, Object, OutputSocket};
use std::{borrow::Cow, fmt::Display, num::ParseIntError, rc::Rc, str::FromStr};
use thiserror::Error;

pub fn nop_node_class() -> Class {
    Class {
        name: "nop".into(),
        default_node: Rc::new(NopNode) as Rc<dyn Node>,
    }
}

pub fn if_node_class() -> Class {
    Class {
        name: "if".into(),
        default_node: Rc::new(IfNode) as Rc<dyn Node>,
    }
}

pub fn bool_class() -> Class {
    Class {
        name: "bool".into(),
        default_node: Rc::new(BoolConstructor) as Rc<dyn Node>,
    }
}

pub fn any_class() -> Class {
    Class {
        name: "any".into(),
        default_node: Rc::new(NopNode) as Rc<dyn Node>,
    }
}

pub fn print_class() -> Class {
    Class {
        name: "print".into(),
        default_node: Rc::new(Print(PrintVariant {
            ln: true,
            amount: 1,
        })) as Rc<dyn Node>,
    }
}

/// Does nothing. Literal NOP. The easiest node to implement
#[derive(Debug, Clone)]
pub struct NopNode;

impl Node for NopNode {
    fn execute(&self, _context: &mut ExecutionContext) -> usize {
        0
    }

    fn class(&self) -> Class {
        Class {
            name: "nop".into(),
            default_node: Rc::new(self.clone()) as Rc<dyn Node>,
        }
    }

    fn variants(&self) -> Vec<Cow<'_, str>> {
        vec!["nop".into()]
    }

    fn current_variant(&self) -> Cow<'_, str> {
        "nop".into()
    }

    fn set_variant(&mut self, _variant: &str) {}

    fn inputs(&self) -> Vec<InputSocket> {
        vec![]
    }

    fn outputs(&self) -> Vec<OutputSocket> {
        vec![]
    }

    fn branches(&self) -> usize {
        1
    }
}

#[derive(Debug, Clone)]
pub struct BoolConstructor;

impl Object for bool {
    fn class(&self) -> Class {
        bool_class()
    }

    fn as_number(&self) -> f32 {
        if *self {
            1.0
        } else {
            0.0
        }
    }

    fn as_bool(&self) -> bool {
        *self
    }

    fn get_field(&self, _field: &str) -> &Rc<dyn Object> {
        unimplemented!()
    }

    fn set_field(&mut self, _field: &str, _value: &Rc<dyn Object>) {
        unimplemented!()
    }
}

impl Node for BoolConstructor {
    fn execute(&self, context: &mut ExecutionContext) -> usize {
        let cond = context.get_inputs()[0].as_bool();
        context.set_outputs(vec![Rc::new(cond) as Rc<dyn Object>]);
        0
    }

    fn class(&self) -> Class {
        bool_class()
    }

    fn variants(&self) -> Vec<Cow<'_, str>> {
        vec!["from-object".into()]
    }

    fn current_variant(&self) -> Cow<'_, str> {
        "from-object".into()
    }

    fn set_variant(&mut self, _variant: &str) {}

    fn inputs(&self) -> Vec<InputSocket> {
        vec![InputSocket { class: any_class() }]
    }

    fn outputs(&self) -> Vec<OutputSocket> {
        vec![OutputSocket {
            class: bool_class(),
        }]
    }

    fn branches(&self) -> usize {
        1
    }
}

#[derive(Debug, Clone)]
pub struct IfNode;

impl Node for IfNode {
    fn execute(&self, context: &mut ExecutionContext) -> usize {
        let cond = context.get_inputs()[0].as_bool();
        cond as usize
    }

    fn class(&self) -> Class {
        Class {
            name: "if".into(),
            default_node: Rc::new(self.clone()) as Rc<dyn Node>,
        }
    }

    fn variants(&self) -> Vec<Cow<'_, str>> {
        vec!["if".into()]
    }

    fn current_variant(&self) -> Cow<'_, str> {
        "if".into()
    }

    fn set_variant(&mut self, _variant: &str) {}

    fn inputs(&self) -> Vec<InputSocket> {
        vec![InputSocket {
            class: bool_class(),
        }]
    }

    fn outputs(&self) -> Vec<OutputSocket> {
        vec![]
    }

    fn branches(&self) -> usize {
        2
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PrintVariant {
    ln: bool,
    amount: usize,
}

impl Display for PrintVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}",
            match self.ln {
                true => "println",
                false => "print",
            },
            self.amount
        )
    }
}

impl FromStr for PrintVariant {
    type Err = PrintVariantParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "print" {
            Ok(Self {
                ln: false,
                amount: 1,
            })
        } else if s == "println" {
            Ok(Self {
                ln: true,
                amount: 1,
            })
        } else if let [print_kind, print_amount] = s.split(':').collect::<Vec<&str>>()[..] {
            let ln = match print_kind {
                "print" => false,
                "println" => true,
                s => return Err(PrintVariantParseError::InvalidPrintKind(s.into())),
            };
            let amount = print_amount.parse()?;
            Ok(Self { ln, amount })
        } else {
            Err(PrintVariantParseError::InvalidVariant(s.into()))
        }
    }
}

#[derive(Debug, Clone, Error)]
enum PrintVariantParseError {
    #[error("Invalid variant: {0}")]
    InvalidVariant(String),
    #[error("Invalid print kind: {0}")]
    InvalidPrintKind(String),
    #[error("Failed to parse amount: {0}")]
    AmountParseError(ParseIntError),
}

impl From<ParseIntError> for PrintVariantParseError {
    fn from(e: ParseIntError) -> Self {
        Self::AmountParseError(e)
    }
}

#[derive(Debug, Clone)]
pub struct Print(PrintVariant);

impl Node for Print {
    fn execute(&self, context: &mut ExecutionContext) -> usize {
        let to_print: String = context
            .get_inputs()
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(" ");
        if self.0.ln {
            println!("{}", to_print);
        } else {
            print!("{}", to_print);
        };
        0
    }

    fn class(&self) -> Class {
        print_class()
    }

    fn variants(&self) -> Vec<Cow<'_, str>> {
        vec![
            "print".into(),
            "println".into(),
            Cow::Owned(self.0.to_string()),
        ]
    }

    fn current_variant(&self) -> Cow<'_, str> {
        self.0.to_string().into()
    }

    fn set_variant(&mut self, variant: &str) {
        self.0 = variant.parse().unwrap()
    }

    fn inputs(&self) -> Vec<InputSocket> {
        vec![InputSocket { class: any_class() }; self.0.amount]
    }

    fn outputs(&self) -> Vec<OutputSocket> {
        vec![]
    }

    fn branches(&self) -> usize {
        1
    }
}
