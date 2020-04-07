/*!
One-line description.

More detailed description, with

# Example

*/

use crate::xpath1::model::predicate::Predicate;
use crate::xpath1::model::select::{AxisSpecifier, NodeTest, Select};
use crate::xpath1::model::ToAbbrString;
use std::fmt::{Display, Formatter, Result};
use std::slice::Iter;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Step {
    select: Select,
    predicates: Vec<Predicate>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

macro_rules! step_fn {
    ($fn_name:ident) => {
        pub fn $fn_name() -> Self {
            Self {
                select: Select::$fn_name(),
                predicates: Vec::default(),
            }
        }
    };
    ($fn_name:ident, named) => {
        pub fn $fn_name(named: &str) -> Self {
            Self {
                select: Select::$fn_name(named),
                predicates: Vec::default(),
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Step {
    fn default() -> Self {
        Self {
            select: Default::default(),
            predicates: Default::default(),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Step {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}{}",
            self.select,
            self.predicates
                .iter()
                .map(|p| format!("[{}]", p))
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl ToAbbrString for Step {
    fn to_abbr_string(&self) -> String {
        format!(
            "{}{}",
            self.select.to_abbr_string(),
            self.predicates
                .iter()
                .map(|p| format!("[{}]", p.to_abbr_string()))
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Step {
    pub fn new(select: Select) -> Self {
        Self {
            select,
            predicates: Default::default(),
        }
    }

    pub fn new_from(axis: AxisSpecifier, node_test: NodeTest) -> Self {
        Self {
            select: Select::new(axis, node_test),
            predicates: Default::default(),
        }
    }

    pub fn new_with_predicate(
        axis: AxisSpecifier,
        node_test: NodeTest,
        predicate: Predicate,
    ) -> Self {
        Self {
            select: Select::new(axis, node_test),
            predicates: vec![predicate],
        }
    }

    pub fn append(&mut self, predicate: Predicate) -> &mut Self {
        self.predicates.push(predicate);
        self
    }

    pub fn select_expr(&self) -> Select {
        self.select.clone()
    }

    pub fn predicate_exprs(&self) -> Iter<Predicate> {
        self.predicates.iter()
    }

    step_fn!(all_ancestors);
    step_fn!(all_ancestor_elements);
    step_fn!(all_ancestor_text);
    step_fn!(all_ancestor_comments);
    step_fn!(ancestor_elements, named);

    step_fn!(all_ancestors_or_self);
    step_fn!(all_ancestor_or_self_elements);
    step_fn!(all_ancestor_or_self_text);
    step_fn!(all_ancestor_or_self_comments);
    step_fn!(ancestor_or_self_elements, named);

    step_fn!(all_attributes);
    step_fn!(attributes, named);

    step_fn!(all_children);
    step_fn!(all_child_elements);
    step_fn!(all_child_text);
    step_fn!(all_child_comments);
    step_fn!(child_elements, named);

    step_fn!(all_descendants);
    step_fn!(all_descendant_elements);
    step_fn!(all_descendant_text);
    step_fn!(all_descendant_comments);
    step_fn!(descendant_elements, named);

    step_fn!(all_descendants_or_self);
    step_fn!(all_descendant_or_self_elements);
    step_fn!(all_descendant_or_self_text);
    step_fn!(all_descendant_or_self_comments);
    step_fn!(descendant_or_self_elements, named);

    step_fn!(all_following);
    step_fn!(all_following_elements);
    step_fn!(all_following_text);
    step_fn!(all_following_comments);
    step_fn!(following_elements, named);

    step_fn!(all_following_sibling);
    step_fn!(all_following_sibling_elements);
    step_fn!(all_following_sibling_text);
    step_fn!(all_following_sibling_comments);
    step_fn!(following_sibling_elements, named);

    // Namespace

    step_fn!(all_parent);
    step_fn!(all_parent_elements);
    step_fn!(all_parent_text);
    step_fn!(all_parent_comments);
    step_fn!(parent_elements, named);

    step_fn!(all_preceding);
    step_fn!(all_preceding_elements);
    step_fn!(all_preceding_text);
    step_fn!(all_preceding_comments);
    step_fn!(preceding_elements, named);

    step_fn!(all_preceding_sibling);
    step_fn!(all_preceding_sibling_elements);
    step_fn!(all_preceding_sibling_text);
    step_fn!(all_preceding_sibling_comments);
    step_fn!(preceding_sibling_elements, named);

    step_fn!(all_self);
    step_fn!(all_self_elements);
    step_fn!(all_self_text);
    step_fn!(all_self_comments);
    step_fn!(self_elements, named);
}
