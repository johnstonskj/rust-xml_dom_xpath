use crate::xpath1::model::function::is_function;
use crate::xpath1::model::select::Select;
use crate::xpath1::model::{AxisSpecifier, NodeTest, ToAbbrString};
use std::borrow::Borrow;
use std::fmt::{Display, Formatter, Result};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This models the predicate component of a `Step`, each step having zero or more predicates.
///
/// Corresponds to the BNF production `Predicate` (8).
///
#[derive(Clone, Debug)]
pub enum Predicate {
    /// An expression
    Expr(ExprNode),
    /// A terminal value
    Terminal(Terminal),
    /// A function call.
    Function(FunctionCall),
}

///
/// This models the set of binary (and one unary) expressions. Note that we use the parser to
/// determine precedence so that the tree build using these nodes is precedence-unaware.
///
/// Corresponds to the BNF productions 14, 18-27.
///
#[derive(Clone, Debug)]
pub enum ExprNode {
    /// Predicate `"and"` Predicate
    And {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    /// Predicate `"or"` Predicate
    Or {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    /// Predicate `"="` Predicate
    Equals {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    /// Predicate `"!="` Predicate
    NotEquals {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    /// Predicate `"<"` Predicate
    LessThan {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    /// Predicate `"<="` Predicate
    LessThanOrEqual {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    /// Predicate `">"` Predicate
    GreaterThan {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    /// Predicate `">="` Predicate
    GreaterThanOrEqual {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    /// Predicate `"+"` Predicate
    Add {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    /// Predicate `"-"` Predicate
    Subtract {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    /// Predicate `"*"` Predicate
    Multiply {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    /// Predicate `"/"` Predicate
    Divide {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    /// Predicate `"mod"` Predicate
    Modulus {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    /// Predicate `"div"` Predicate
    FPDiv {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    /// `"-"` Predicate
    UnaryMinus { value: Box<Predicate> },
}

///
/// This models a terminal value in the expression.
///  
/// Corresponds to the BNF production `PrimaryExpr` (15).
///
#[derive(Clone, Debug)]
pub enum Terminal {
    /// A variable reference
    Variable(String),
    /// A String literal
    Literal(String),
    /// A Number value, note that the specification makes these all floats
    Number(f64),
    /// A Select expression
    Select(Select),
}

///
/// This models a call to one of the set of pre-defined functions.
///
/// Corresponds to the BNF production `FunctionCall` (16).
///
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
        /// Construct the corresponding `Predicate` with the `left` and `right` values provided.
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
    /// Construct a new Predicate as simply a string literal value.
    pub fn literal(value: &str) -> Self {
        Predicate::Terminal(Terminal::Literal(value.to_string()))
    }

    /// Construct a new Predicate as simply a number value.
    pub fn number(value: f64) -> Self {
        Predicate::Terminal(Terminal::Float(value))
    }

    /// Construct a new Predicate as simply a variable reference.
    pub fn variable(named: &str) -> Self {
        Predicate::Terminal(Terminal::Variable(named.to_string()))
    }

    /// Construct a new Predicate as simply a function call (no arguments).
    pub fn function(named: &str) -> Self {
        Predicate::Function(FunctionCall::with(named))
    }

    /// Construct a new Predicate as simply a function call with arguments.
    pub fn function_with(named: &str, args: &[Predicate]) -> Self {
        Predicate::Function(FunctionCall::with_both(named, args))
    }

    /// Construct a new Predicate as simply a select expression.
    pub fn select(axis: AxisSpecifier, node_test: NodeTest) -> Self {
        Predicate::Terminal(Terminal::Select(Select::with(axis, node_test)))
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

    /// Construct a unary minus predicate with the value provided.
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
    /// Construct a new function call to the function named `name`.
    pub fn with(name: &str) -> Self {
        Self::with_both(name, &[])
    }

    /// Construct a new function call to the function named `name` with the provided `arguments`.
    pub fn with_both(name: &str, arguments: &[Predicate]) -> Self {
        assert!(is_function(name));
        // TODO: validate arg count
        FunctionCall {
            name: name.to_string(),
            arguments: arguments.to_vec(),
        }
    }

    /// Append an argument to those made to this function.
    pub fn append(&mut self, argument: Predicate) {
        self.arguments.push(argument);
    }
}
