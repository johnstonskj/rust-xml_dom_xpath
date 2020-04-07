/*!
Implements the XML Path Language (XPath), [Version 1.0](https://www.w3.org/TR/xpath-10/).
*/

use std::fmt::{Display, Formatter};
use xml_dom::level2::RefNode;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Parse(parse::ParseError),
}

pub const XPATH_VERSION: &str = "1.0";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Evaluate the XPath string against the set of nodes that act as context.
///
pub fn evaluate(xpath: &str, context_nodes: &[RefNode]) -> Result<Vec<RefNode>, Error> {
    use crate::xpath1::evaluate::{evaluate_path, NodeSet};
    use std::iter::FromIterator;

    let xpath = parse::read_str(xpath);
    let results = evaluate_path(&NodeSet::from_iter(context_nodes.iter().cloned()), &xpath);
    Ok(Vec::from_iter(results.iter().cloned()))
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
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Parse(err) => Some(err),
        }
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

pub mod parse;
