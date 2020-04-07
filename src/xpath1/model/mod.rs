/*!
Provides types for constructing XPath paths, steps, and expressions to be executed against an
XML document.

Once constructed the expression can be turned into a String using the standard [`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html)
trait, it can also be turned into a String using the [`ToAbbrString`](trait.ToAbbrString.html)
trait that uses the _Abbreviated Syntax_ in the specification.

# Example

```rust
use xml_dom_xpath::xpath1::model::*;

let mut path = LocationPath::default();

let mut step = Step::following_sibling_elements("chapter");

step.append(Predicate::eq(
    Predicate::function("position"),
    Predicate::integer(1),
));

path.append(step);
assert_eq!(
    path.to_string(),
    "following-sibling::chapter[position() = 1]"
);
assert_eq!(path.to_abbr_string(), "following-sibling::chapter[1]");
```
*/

use std::fmt::Display;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Supports the display of paths and steps in their abbreviated form, as described in the XPath
/// specification, §2.5, [Abbreviated Syntax](https://www.w3.org/TR/xpath-10/#path-abbrev).
///
pub trait ToAbbrString: Display {
    /// Display in abbreviated form.
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
