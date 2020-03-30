/*!
One-line description.

More detailed description, with

# Example

*/

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

use std::fmt::Display;

pub trait ToAbbrString: Display {
    fn to_abbr_string(&self) -> String {
        self.to_string()
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod select;
pub use select::{AxisSpecifier, NodeTest, Select};

mod predicate;
pub use predicate::{ExprNode, FunctionCall, Predicate, Terminal};

mod step;
pub use step::Step;

mod path;
pub use path::LocationPath;

mod function;
