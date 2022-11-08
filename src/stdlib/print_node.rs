use super::any_class;
use crate::{Class, ExecutionContext, InputSocket, Node, OutputSocket};
use std::{borrow::Cow, fmt::Display, num::ParseIntError, rc::Rc, str::FromStr};
use thiserror::Error;

pub fn print_class() -> Class {
    Class {
        name: "print".into(),
        default_node: Rc::new(Print(PrintVariant {
            ln: true,
            amount: 1,
        })) as Rc<dyn Node>,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PrintVariant {
    ln: bool,
    amount: u32,
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
    fn execute(&self, context: &mut ExecutionContext) -> u32 {
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

    fn accepts_arbitrary_variants(&self) -> bool {
        true
    }

    fn inputs(&self) -> Vec<InputSocket> {
        vec![InputSocket { class: any_class() }; self.0.amount as usize]
    }

    fn outputs(&self) -> Vec<OutputSocket> {
        vec![]
    }
}
