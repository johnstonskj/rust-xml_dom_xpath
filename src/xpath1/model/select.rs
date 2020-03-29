/*!
One-line description.

More detailed description, with

# Example

*/

use crate::xpath1::model::ToAbbrString;
use std::fmt::{Display, Formatter, Result};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AxisSpecifier {
    Ancestor,
    AncestorOrSelf,
    Attribute,
    Child,
    Descendant,
    DescendantOrSelf,
    Following,
    FollowingSibling,
    Namespace,
    Parent,
    Preceding,
    PrecedingSibling,
    SelfNode,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NodeTest {
    All,
    Named(String),
    Comment,
    Text,
    ProcessingInstruction(Option<String>),
    Node,
}

#[derive(Clone, Debug)]
pub struct Select {
    axis: AxisSpecifier,
    node_test: NodeTest,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

macro_rules! select_fn {
    ($fn_name:ident, $axis:ident, $node_test:ident) => {
        pub fn $fn_name() -> Self {
            Self::new(AxisSpecifier::$axis, NodeTest::$node_test)
        }
    };
    ($fn_name:ident, $axis:ident) => {
        pub fn $fn_name(named: &str) -> Self {
            Self::new(AxisSpecifier::$axis, NodeTest::Named(named.to_string()))
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for AxisSpecifier {
    fn default() -> Self {
        AxisSpecifier::Child
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for AxisSpecifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            if f.alternate() {
                println!("ALTER");
                match self {
                    AxisSpecifier::Ancestor => "ancestor::",
                    AxisSpecifier::AncestorOrSelf => "ancestor-or-self::",
                    AxisSpecifier::Attribute => "@",
                    AxisSpecifier::Child => "",
                    AxisSpecifier::Descendant => "descendant::",
                    AxisSpecifier::DescendantOrSelf => "descendant-or-self::",
                    AxisSpecifier::Following => "following::",
                    AxisSpecifier::FollowingSibling => "following-sibling::",
                    AxisSpecifier::Namespace => "namespace::",
                    AxisSpecifier::Parent => "..",
                    AxisSpecifier::Preceding => "preceding::",
                    AxisSpecifier::PrecedingSibling => "preceding-sibling::",
                    AxisSpecifier::SelfNode => ".",
                }
            } else {
                match self {
                    AxisSpecifier::Ancestor => "ancestor::",
                    AxisSpecifier::AncestorOrSelf => "ancestor-or-self::",
                    AxisSpecifier::Attribute => "attribute::",
                    AxisSpecifier::Child => "child::",
                    AxisSpecifier::Descendant => "descendant::",
                    AxisSpecifier::DescendantOrSelf => "descendant-or-self::",
                    AxisSpecifier::Following => "following::",
                    AxisSpecifier::FollowingSibling => "following-sibling::",
                    AxisSpecifier::Namespace => "namespace::",
                    AxisSpecifier::Parent => "parent::",
                    AxisSpecifier::Preceding => "preceding::",
                    AxisSpecifier::PrecedingSibling => "preceding-sibling::",
                    AxisSpecifier::SelfNode => "self::",
                }
            }
        )
    }
}
// ------------------------------------------------------------------------------------------------

impl ToAbbrString for AxisSpecifier {
    fn to_abbr_string(&self) -> String {
        match self {
            AxisSpecifier::Ancestor => "ancestor::",
            AxisSpecifier::AncestorOrSelf => "ancestor-or-self::",
            AxisSpecifier::Attribute => "@",
            AxisSpecifier::Child => "",
            AxisSpecifier::Descendant => "descendant::",
            AxisSpecifier::DescendantOrSelf => "descendant-or-self::",
            AxisSpecifier::Following => "following::",
            AxisSpecifier::FollowingSibling => "following-sibling::",
            AxisSpecifier::Namespace => "namespace::",
            AxisSpecifier::Parent => "parent::",
            AxisSpecifier::Preceding => "preceding::",
            AxisSpecifier::PrecedingSibling => "preceding-sibling::",
            AxisSpecifier::SelfNode => "self::",
        }
        .to_string()
    }
}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

impl Default for NodeTest {
    fn default() -> Self {
        NodeTest::All
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for NodeTest {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                NodeTest::All => "*".to_string(),
                NodeTest::Named(name) => name.to_string(),
                NodeTest::Comment => "comment()".to_string(),
                NodeTest::Text => "text()".to_string(),
                NodeTest::ProcessingInstruction(None) => "processing-instruction()".to_string(),
                NodeTest::ProcessingInstruction(Some(literal)) =>
                    format!("processing-instruction({})", literal),
                NodeTest::Node => "node()".to_string(),
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl ToAbbrString for NodeTest {}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

impl Default for Select {
    fn default() -> Self {
        Self {
            axis: Default::default(),
            node_test: Default::default(),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Select {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}{}", self.axis, self.node_test,)
    }
}

// ------------------------------------------------------------------------------------------------

impl ToAbbrString for Select {
    fn to_abbr_string(&self) -> String {
        if self.axis == AxisSpecifier::SelfNode && self.node_test == NodeTest::Node {
            ".".to_string()
        } else if self.axis == AxisSpecifier::Parent && self.node_test == NodeTest::Node {
            "..".to_string()
        } else if self.axis == AxisSpecifier::DescendantOrSelf && self.node_test == NodeTest::Node {
            "//".to_string()
        } else {
            format!("{}{}", self.axis.to_abbr_string(), self.node_test)
        }
    }
}

impl Select {
    pub fn new(axis: AxisSpecifier, node_test: NodeTest) -> Self {
        Self { axis, node_test }
    }
    select_fn!(all_ancestors, Ancestor, Node);
    select_fn!(all_ancestor_elements, Ancestor, All);
    select_fn!(all_ancestor_text, Ancestor, Text);
    select_fn!(all_ancestor_comments, Ancestor, Comment);
    select_fn!(ancestor_elements, Ancestor);

    select_fn!(all_ancestors_or_self, AncestorOrSelf, Node);
    select_fn!(all_ancestor_or_self_elements, AncestorOrSelf, All);
    select_fn!(all_ancestor_or_self_text, AncestorOrSelf, Text);
    select_fn!(all_ancestor_or_self_comments, AncestorOrSelf, Comment);
    select_fn!(ancestor_or_self_elements, AncestorOrSelf);

    select_fn!(all_attributes, Attribute, All);
    select_fn!(attributes, Attribute);

    select_fn!(all_children, Child, Node);
    select_fn!(all_child_elements, Child, All);
    select_fn!(all_child_text, Child, Text);
    select_fn!(all_child_comments, Child, Comment);
    select_fn!(child_elements, Child);

    select_fn!(all_descendants, Descendant, Node);
    select_fn!(all_descendant_elements, Descendant, All);
    select_fn!(all_descendant_text, Descendant, Text);
    select_fn!(all_descendant_comments, Descendant, Comment);
    select_fn!(descendant_elements, Descendant);

    select_fn!(all_descendants_or_self, DescendantOrSelf, Node);
    select_fn!(all_descendant_or_self_elements, DescendantOrSelf, All);
    select_fn!(all_descendant_or_self_text, DescendantOrSelf, Text);
    select_fn!(all_descendant_or_self_comments, DescendantOrSelf, Comment);
    select_fn!(descendant_or_self_elements, DescendantOrSelf);

    select_fn!(all_following, Following, Node);
    select_fn!(all_following_elements, Following, All);
    select_fn!(all_following_text, Following, Text);
    select_fn!(all_following_comments, Following, Comment);
    select_fn!(following_elements, Following);

    select_fn!(all_following_sibling, FollowingSibling, Node);
    select_fn!(all_following_sibling_elements, FollowingSibling, All);
    select_fn!(all_following_sibling_text, FollowingSibling, Text);
    select_fn!(all_following_sibling_comments, FollowingSibling, Comment);
    select_fn!(following_sibling_elements, FollowingSibling);

    // Namespace

    select_fn!(all_parent, Parent, Node);
    select_fn!(all_parent_elements, Parent, All);
    select_fn!(all_parent_text, Parent, Text);
    select_fn!(all_parent_comments, Parent, Comment);
    select_fn!(parent_elements, Parent);

    select_fn!(all_preceding, Preceding, Node);
    select_fn!(all_preceding_elements, Preceding, All);
    select_fn!(all_preceding_text, Preceding, Text);
    select_fn!(all_preceding_comments, Preceding, Comment);
    select_fn!(preceding_elements, Preceding);

    select_fn!(all_preceding_sibling, PrecedingSibling, Node);
    select_fn!(all_preceding_sibling_elements, PrecedingSibling, All);
    select_fn!(all_preceding_sibling_text, PrecedingSibling, Text);
    select_fn!(all_preceding_sibling_comments, PrecedingSibling, Comment);
    select_fn!(preceding_sibling_elements, PrecedingSibling);

    select_fn!(all_self, SelfNode, Node);
    select_fn!(all_self_elements, SelfNode, All);
    select_fn!(all_self_text, SelfNode, Text);
    select_fn!(all_self_comments, SelfNode, Comment);
    select_fn!(self_elements, SelfNode);
}
