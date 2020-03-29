/*!
One-line description.

More detailed description, with

# Example

*/

use crate::xpath1::model::select::Select;
use crate::xpath1::model::{AxisSpecifier, NodeTest, ToAbbrString};
use std::borrow::Borrow;
use std::fmt::{Display, Formatter, Result};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum Predicate {
    Expr(ExprNode),
    Terminal(Terminal),
    Function(FunctionCall),
}

#[derive(Clone, Debug)]
pub enum ExprNode {
    And {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    Or {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    Equals {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    NotEquals {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    LessThan {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    LessThanOrEqual {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    GreaterThan {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    GreaterThanOrEqual {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    Add {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    Subtract {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    Multiply {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    Divide {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    Modulus {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    FPDiv {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    UnaryMinus {
        value: Box<Predicate>,
    },
}

#[derive(Clone, Debug)]
pub enum Terminal {
    Variable(String),
    Literal(String),
    Number(i64),
    Float(f64),
    Select(Select),
}

#[derive(Clone, Debug)]
pub struct FunctionCall {
    name: String,
    arguments: Vec<Predicate>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

macro_rules! predicate_fn {
    ($fn_name:ident, $expr_t:ident) => {
        pub fn $fn_name(left: Predicate, right: Predicate) -> Self {
            Predicate::Expr(ExprNode::$expr_t {
                left: Box::new(left),
                right: Box::new(right),
            })
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Predicate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Predicate::Expr(v) => v.to_string(),
                Predicate::Terminal(v) => v.to_string(),
                Predicate::Function(v) => v.to_string(),
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl ToAbbrString for Predicate {
    fn to_abbr_string(&self) -> String {
        match self {
            Predicate::Expr(v) => v.to_abbr_string(),
            Predicate::Terminal(Terminal::Select(v)) => v.to_abbr_string(),
            Predicate::Terminal(v) => v.to_string(),
            Predicate::Function(v) => v.to_string(),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Predicate {
    pub fn literal(value: &str) -> Self {
        Predicate::Terminal(Terminal::Literal(value.to_string()))
    }
    pub fn integer(value: i64) -> Self {
        Predicate::Terminal(Terminal::Number(value))
    }
    pub fn float(value: f64) -> Self {
        Predicate::Terminal(Terminal::Float(value))
    }
    pub fn variable(named: &str) -> Self {
        Predicate::Terminal(Terminal::Variable(named.to_string()))
    }
    pub fn function(named: &str) -> Self {
        Predicate::Function(FunctionCall {
            name: named.to_string(),
            arguments: Vec::default(),
        })
    }
    pub fn function_with(named: &str, args: &[Predicate]) -> Self {
        Predicate::Function(FunctionCall {
            name: named.to_string(),
            arguments: args.to_vec(),
        })
    }
    pub fn select(axis: AxisSpecifier, node_test: NodeTest) -> Self {
        Predicate::Terminal(Terminal::Select(Select::new(axis, node_test)))
    }

    predicate_fn!(and, And);
    predicate_fn!(or, Or);
    predicate_fn!(eq, Equals);
    predicate_fn!(neq, NotEquals);
    predicate_fn!(lt, LessThan);
    predicate_fn!(lteq, LessThanOrEqual);
    predicate_fn!(gt, GreaterThan);
    predicate_fn!(gteq, GreaterThanOrEqual);
    predicate_fn!(add, Add);
    predicate_fn!(subtract, Subtract);
    predicate_fn!(multiply, Multiply);
    predicate_fn!(divide, Divide);
    predicate_fn!(a_mod, Modulus);
    predicate_fn!(div, FPDiv);

    pub fn minus(value: Predicate) -> Self {
        Predicate::Expr(ExprNode::UnaryMinus {
            value: Box::new(value),
        })
    }
}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

impl Display for ExprNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_some_string(false))
    }
}

// ------------------------------------------------------------------------------------------------

impl ToAbbrString for ExprNode {
    fn to_abbr_string(&self) -> String {
        self.to_some_string(true)
    }
}

// ------------------------------------------------------------------------------------------------

impl ExprNode {
    pub(crate) fn to_some_string(&self, abbr: bool) -> String {
        let format_fn = if abbr {
            Predicate::to_abbr_string
        } else {
            Predicate::to_string
        };
        match self {
            ExprNode::And { left, right } => {
                format!("{} and {}", format_fn(left), format_fn(right))
            }
            ExprNode::Or { left, right } => format!("{} or {}", format_fn(left), format_fn(right)),
            ExprNode::Equals { left, right } => match (abbr, left.borrow(), right.borrow()) {
                (
                    true,
                    Predicate::Function(FunctionCall { name, arguments }),
                    Predicate::Terminal(Terminal::Number(n)),
                ) => {
                    if name == &"position".to_string() && arguments.is_empty() {
                        format!("{}", n)
                    } else {
                        format!("{} = {}", format_fn(left), format_fn(right))
                    }
                }
                _ => format!("{} = {}", format_fn(left), format_fn(right)),
            },
            ExprNode::NotEquals { left, right } => {
                format!("{} != {}", format_fn(left), format_fn(right))
            }
            ExprNode::LessThan { left, right } => {
                format!("{} < {}", format_fn(left), format_fn(right))
            }
            ExprNode::LessThanOrEqual { left, right } => {
                format!("{} <= {}", format_fn(left), format_fn(right))
            }
            ExprNode::GreaterThan { left, right } => {
                format!("{} > {}", format_fn(left), format_fn(right))
            }
            ExprNode::GreaterThanOrEqual { left, right } => {
                format!("{} >= {}", format_fn(left), format_fn(right))
            }
            ExprNode::Add { left, right } => format!("{} + {}", format_fn(left), format_fn(right)),
            ExprNode::Subtract { left, right } => {
                format!("{} - {}", format_fn(left), format_fn(right))
            }
            ExprNode::Multiply { left, right } => {
                format!("{} * {}", format_fn(left), format_fn(right))
            }
            ExprNode::Divide { left, right } => {
                format!("{} / {}", format_fn(left), format_fn(right))
            }
            ExprNode::Modulus { left, right } => {
                format!("{} mod {}", format_fn(left), format_fn(right))
            }
            ExprNode::FPDiv { left, right } => {
                format!("{} div {}", format_fn(left), format_fn(right))
            }
            ExprNode::UnaryMinus { value } => format!("- {}", format_fn(value)),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

impl Display for Terminal {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Terminal::Variable(v) => format!("${}", v),
                Terminal::Literal(v) => format!("'{}'", v),
                Terminal::Number(v) => format!("{}", v),
                Terminal::Float(v) => format!("{}", v),
                Terminal::Select(v) => format!("{}", v),
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl ToAbbrString for Terminal {}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

impl Display for FunctionCall {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}({})",
            self.name,
            self.arguments
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl ToAbbrString for FunctionCall {}

// ------------------------------------------------------------------------------------------------

impl FunctionCall {
    pub fn append(&mut self, argument: Predicate) {
        self.arguments.push(argument);
    }
}
