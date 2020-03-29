/*!
One-line description.

More detailed description, with

# Example

*/

use crate::xpath1::model::predicate::Predicate;
use crate::xpath1::model::select::{AxisSpecifier, NodeTest, Select};
use crate::xpath1::model::ToAbbrString;
use std::fmt::{Display, Formatter, Result};

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
    ($fn_name:ident, $axis:ident, $node_test:ident) => {
        pub fn $fn_name() -> Self {
            Self {
                select: Select::new(AxisSpecifier::$axis, NodeTest::$node_test),
                predicates: Vec::default(),
            }
        }
    };
    ($fn_name:ident, $axis:ident) => {
        pub fn $fn_name(named: &str) -> Self {
            Self {
                select: Select::new(AxisSpecifier::$axis, NodeTest::Named(named.to_string())),
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
    pub fn new(axis: AxisSpecifier, node_test: NodeTest) -> Self {
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

    step_fn!(all_ancestors, Ancestor, Node);
    step_fn!(all_ancestor_elements, Ancestor, All);
    step_fn!(all_ancestor_text, Ancestor, Text);
    step_fn!(all_ancestor_comments, Ancestor, Comment);
    step_fn!(ancestor_elements, Ancestor);

    step_fn!(all_ancestors_or_self, AncestorOrSelf, Node);
    step_fn!(all_ancestor_or_self_elements, AncestorOrSelf, All);
    step_fn!(all_ancestor_or_self_text, AncestorOrSelf, Text);
    step_fn!(all_ancestor_or_self_comments, AncestorOrSelf, Comment);
    step_fn!(ancestor_or_self_elements, AncestorOrSelf);

    step_fn!(all_attributes, Attribute, All);
    step_fn!(attributes, Attribute);

    step_fn!(all_children, Child, Node);
    step_fn!(all_child_elements, Child, All);
    step_fn!(all_child_text, Child, Text);
    step_fn!(all_child_comments, Child, Comment);
    step_fn!(child_elements, Child);

    step_fn!(all_descendants, Descendant, Node);
    step_fn!(all_descendant_elements, Descendant, All);
    step_fn!(all_descendant_text, Descendant, Text);
    step_fn!(all_descendant_comments, Descendant, Comment);
    step_fn!(descendant_elements, Descendant);

    step_fn!(all_descendants_or_self, DescendantOrSelf, Node);
    step_fn!(all_descendant_or_self_elements, DescendantOrSelf, All);
    step_fn!(all_descendant_or_self_text, DescendantOrSelf, Text);
    step_fn!(all_descendant_or_self_comments, DescendantOrSelf, Comment);
    step_fn!(descendant_or_self_elements, DescendantOrSelf);

    step_fn!(all_following, Following, Node);
    step_fn!(all_following_elements, Following, All);
    step_fn!(all_following_text, Following, Text);
    step_fn!(all_following_comments, Following, Comment);
    step_fn!(following_elements, Following);

    step_fn!(all_following_sibling, FollowingSibling, Node);
    step_fn!(all_following_sibling_elements, FollowingSibling, All);
    step_fn!(all_following_sibling_text, FollowingSibling, Text);
    step_fn!(all_following_sibling_comments, FollowingSibling, Comment);
    step_fn!(following_sibling_elements, FollowingSibling);

    // Namespace

    step_fn!(all_parent, Parent, Node);
    step_fn!(all_parent_elements, Parent, All);
    step_fn!(all_parent_text, Parent, Text);
    step_fn!(all_parent_comments, Parent, Comment);
    step_fn!(parent_elements, Parent);

    step_fn!(all_preceding, Preceding, Node);
    step_fn!(all_preceding_elements, Preceding, All);
    step_fn!(all_preceding_text, Preceding, Text);
    step_fn!(all_preceding_comments, Preceding, Comment);
    step_fn!(preceding_elements, Preceding);

    step_fn!(all_preceding_sibling, PrecedingSibling, Node);
    step_fn!(all_preceding_sibling_elements, PrecedingSibling, All);
    step_fn!(all_preceding_sibling_text, PrecedingSibling, Text);
    step_fn!(all_preceding_sibling_comments, PrecedingSibling, Comment);
    step_fn!(preceding_sibling_elements, PrecedingSibling);

    step_fn!(all_self, SelfNode, Node);
    step_fn!(all_self_elements, SelfNode, All);
    step_fn!(all_self_text, SelfNode, Text);
    step_fn!(all_self_comments, SelfNode, Comment);
    step_fn!(self_elements, SelfNode);
}
