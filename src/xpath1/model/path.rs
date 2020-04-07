/*!
One-line description.

More detailed description, with

# Example

*/

use crate::xpath1::model::step::Step;
use crate::xpath1::model::ToAbbrString;
use std::fmt::{Display, Formatter, Result};
use std::slice::Iter;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct LocationPath {
    root: bool,
    steps: Vec<Step>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

macro_rules! path_fn {
    ($fn_name:ident) => {
        pub fn $fn_name(&mut self) -> &mut Self {
            self.append(Step::$fn_name());
            self
        }
    };
    ($fn_name:ident, named) => {
        pub fn $fn_name(&mut self, named: &str) -> &mut Self {
            self.append(Step::$fn_name(named));
            self
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for LocationPath {
    fn default() -> Self {
        Self {
            root: false,
            steps: Vec::default(),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for LocationPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}{}",
            if self.root { "/" } else { "" },
            self.steps
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join("/")
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl ToAbbrString for LocationPath {
    fn to_abbr_string(&self) -> String {
        format!(
            "{}{}",
            if self.root { "/" } else { "" },
            self.steps
                .iter()
                .map(|s| s.to_abbr_string())
                .collect::<Vec<String>>()
                .join("/")
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl LocationPath {
    pub fn new() -> Self {
        Self {
            root: false,
            steps: Vec::default(),
        }
    }

    pub fn new_with(step: Step) -> Self {
        Self {
            root: false,
            steps: vec![step],
        }
    }

    pub fn root() -> Self {
        Self {
            root: true,
            steps: Vec::default(),
        }
    }

    pub fn root_with(step: Step) -> Self {
        Self {
            root: true,
            steps: vec![step],
        }
    }

    pub fn append(&mut self, step: Step) -> &mut Self {
        self.steps.push(step);
        self
    }

    pub fn is_root(&self) -> bool {
        self.root
    }

    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    pub fn steps(&self) -> Iter<Step> {
        self.steps.iter()
    }

    path_fn!(all_ancestors);
    path_fn!(all_ancestor_elements);
    path_fn!(all_ancestor_text);
    path_fn!(all_ancestor_comments);
    path_fn!(ancestor_elements, named);

    path_fn!(all_ancestors_or_self);
    path_fn!(all_ancestor_or_self_elements);
    path_fn!(all_ancestor_or_self_text);
    path_fn!(all_ancestor_or_self_comments);
    path_fn!(ancestor_or_self_elements, named);

    path_fn!(all_attributes);
    path_fn!(attributes, named);

    path_fn!(all_children);
    path_fn!(all_child_elements);
    path_fn!(all_child_text);
    path_fn!(all_child_comments);
    path_fn!(child_elements, named);

    path_fn!(all_descendants);
    path_fn!(all_descendant_elements);
    path_fn!(all_descendant_text);
    path_fn!(all_descendant_comments);
    path_fn!(descendant_elements, named);

    path_fn!(all_descendants_or_self);
    path_fn!(all_descendant_or_self_elements);
    path_fn!(all_descendant_or_self_text);
    path_fn!(all_descendant_or_self_comments);
    path_fn!(descendant_or_self_elements, named);

    path_fn!(all_following);
    path_fn!(all_following_elements);
    path_fn!(all_following_text);
    path_fn!(all_following_comments);
    path_fn!(following_elements, named);

    path_fn!(all_following_sibling);
    path_fn!(all_following_sibling_elements);
    path_fn!(all_following_sibling_text);
    path_fn!(all_following_sibling_comments);
    path_fn!(following_sibling_elements, named);

    // Namespace

    path_fn!(all_parent);
    path_fn!(all_parent_elements);
    path_fn!(all_parent_text);
    path_fn!(all_parent_comments);
    path_fn!(parent_elements, named);

    path_fn!(all_preceding);
    path_fn!(all_preceding_elements);
    path_fn!(all_preceding_text);
    path_fn!(all_preceding_comments);
    path_fn!(preceding_elements, named);

    path_fn!(all_preceding_sibling);
    path_fn!(all_preceding_sibling_elements);
    path_fn!(all_preceding_sibling_text);
    path_fn!(all_preceding_sibling_comments);
    path_fn!(preceding_sibling_elements, named);

    path_fn!(all_self);
    path_fn!(all_self_elements);
    path_fn!(all_self_text);
    path_fn!(all_self_comments);
}
