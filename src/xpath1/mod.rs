/*!
This module implements [Version 1.0](https://www.w3.org/TR/xpath-10/) of the XML Path Language
(XPath).

The primary API is the [`evaluate_path`](fn.evaluate_path.html) function, however access to the
underlying [`parser`](parser/index.html), [`model`](model/index.html), and [`evaluate`](evaluate/index.html)
modules is also possible.
*/

use crate::xpath1::evaluate::EvaluationError;
use crate::xpath1::parser::ParseError;
use std::fmt::{Display, Formatter};
use xml_dom::level2::RefNode;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Potential errors returned by [`evaluate_path`](fn.evaluate_path.html).
///
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// An error parsing the string representation into the model form.
    Parse(parser::ParseError),
    /// An error evaluating the model form against a set of nodes.
    Evaluate(evaluate::EvaluationError),
}

///
/// The result type for evaluation of an expression.
///
/// # Specification
///
/// The primary syntactic construct in XPath is the expression. An expression matches the
/// production `Expr`. An expression is evaluated to yield an object, which has one of the
/// following four basic types:
///
/// * `node-set` (an unordered collection of nodes without duplicates)
/// * `boolean` (true or false)
/// * `number` (a floating-point number)
/// * `string` (a sequence of UCS characters)
///
pub enum XPathObject {
    /// an unordered collection of nodes without duplicates
    NodeSet(evaluate::NodeSet),
    /// true or false
    Boolean(bool),
    /// a floating-point number
    Number(f64),
    /// a sequence of UCS characters
    String(String),
}

///
/// The version of the XPath specification supported by this module.
///
pub const XPATH_VERSION: &str = "1.0";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Evaluate the XPath string against the set of nodes that act as context.
///
/// This function will first parse the provided `xpath` string with
/// [`parser::read_str`](parser/fn.read_str.html), then call the underlying
/// [`evaluate::evaluate_path`](evaluate/fn.evaluate_path.html) function with the parsed expression
/// and a [`evaluate::NodeSet`](evaluate/struct.NodeSet.html) created from the array `context_nodes`.
///
pub fn evaluate_path(xpath: &str, context_nodes: &[RefNode]) -> Result<XPathObject, Error> {
    use std::iter::FromIterator;

    let xpath = parser::read_str(xpath)?;
    evaluate::evaluate_path(
        &evaluate::NodeSet::from_iter(context_nodes.iter().cloned()),
        &xpath,
    )
    .map_err(|err| err.into())
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::Parse(e) => e.to_string(),
                Error::Evaluate(e) => e.to_string(),
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Parse(err) => Some(err),
            Error::Evaluate(err) => Some(err),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Self::Parse(err)
    }
}

// ------------------------------------------------------------------------------------------------

impl From<EvaluationError> for Error {
    fn from(err: EvaluationError) -> Self {
        Self::Evaluate(err)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod evaluate;

pub mod model;

pub mod parser;
