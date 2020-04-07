use crate::xpath1::model::ToAbbrString;
use std::fmt::{Display, Formatter, Result};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This models the different axis specifiers described in XPath, the default is `Child`.
///
/// Corresponds to the BNF production `AxisSpecifier` (5).
///
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AxisSpecifier {
    /// the `ancestor` axis contains the ancestors of the context node; the ancestors of the context
    /// node consist of the parent of context node and the parent's parent and so on; thus, the
    /// ancestor axis will always include the root node, unless the context node is the root node
    Ancestor,
    /// the `ancestor-or-self` axis contains the context node and the ancestors of the context node;
    /// thus, the ancestor axis will always include the root node
    AncestorOrSelf,
    /// the `attribute` axis contains the attributes of the context node; the axis will be empty
    /// unless the context node is an element
    Attribute,
    /// the `child` axis contains the children of the context node
    Child,
    /// the `descendant` axis contains the descendants of the context node; a descendant is a child
    /// or a child of a child and so on; thus the descendant axis never contains attribute or
    /// namespace nodes
    Descendant,
    /// the `descendant-or-self` axis contains the context node and the descendants of the context
    /// node
    DescendantOrSelf,
    /// the `following` axis contains all nodes in the same document as the context node that are
    /// after the context node in document order, excluding any descendants and excluding attribute
    /// nodes and namespace nodes
    Following,
    /// the `following-sibling` axis contains all the following siblings of the context node; if the
    /// context node is an attribute node or namespace node, the `following-sibling` axis is empty
    FollowingSibling,
    /// the `namespace` axis contains the namespace nodes of the context node; the axis will be
    /// empty unless the context node is an element
    Namespace,
    /// the `parent` axis contains the parent of the context node, if there is one
    Parent,
    /// the `preceding` axis contains all nodes in the same document as the context node that are
    /// before the context node in document order, excluding any ancestors and excluding attribute
    /// nodes and namespace nodes
    Preceding,
    /// the `preceding-sibling` axis contains all the preceding siblings of the context node; if the
    /// context node is an attribute node or namespace node, the `preceding-sibling` axis is empty
    PrecedingSibling,
    /// the `self` axis contains just the context node itself
    SelfNode,
}

///
/// This models the different node tests described in XPath, the default is `All`.
///
/// Corresponds to the BNF production `NodeTest` (7).
///
#[derive(Clone, Debug, PartialEq)]
pub enum NodeTest {
    /// All Nodes of the principal type.
    All,
    /// All Nodes of the principal type where `node_name` matches.
    Named(String),
    /// All `Comment` nodes.
    Comment,
    /// All `Text` nodes.
    Text,
    /// All `ProcessingInstruction` nodes, optionally where `target` matches.
    ProcessingInstruction(Option<String>),
    /// Nodes of any type.
    Node,
}

///
/// A container for an `AxisSpecifier` and a `NodeTest`.
///
#[derive(Clone, Debug)]
pub struct Select {
    axis: AxisSpecifier,
    test: NodeTest,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

macro_rules! select_fn {
    ($fn_name:ident, $axis:ident, $node_test:ident) => {
        /// Create a new `Select` using the corresponding axis specifier and node test.
        pub fn $fn_name() -> Self {
            Self::with(AxisSpecifier::$axis, NodeTest::$node_test)
        }
    };
    ($fn_name:ident, $axis:ident) => {
        /// Create a new `Select` using the corresponding axis specifier and node test.
        pub fn $fn_name(named: &str) -> Self {
            Self::with(AxisSpecifier::$axis, NodeTest::Named(named.to_string()))
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
            test: Default::default(),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Select {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}{}", self.axis, self.test,)
    }
}

// ------------------------------------------------------------------------------------------------

impl ToAbbrString for Select {
    fn to_abbr_string(&self) -> String {
        if self.axis == AxisSpecifier::SelfNode && self.test == NodeTest::Node {
            ".".to_string()
        } else if self.axis == AxisSpecifier::Parent && self.test == NodeTest::Node {
            "..".to_string()
        } else if self.axis == AxisSpecifier::DescendantOrSelf && self.test == NodeTest::Node {
            "//".to_string()
        } else {
            format!("{}{}", self.axis.to_abbr_string(), self.test)
        }
    }
}

impl Select {
    ///
    /// Construct a new `Select` component from the provided `axis` and `node_test`.
    ///
    pub fn with(axis: AxisSpecifier, node_test: NodeTest) -> Self {
        Self {
            axis,
            test: node_test,
        }
    }

    ///
    /// Return the axis specifier part of this Select component.
    ///
    pub fn axis_specifier(&self) -> AxisSpecifier {
        self.axis
    }

    ///
    /// Return the node test part of this Select component.
    ///
    pub fn node_test(&self) -> NodeTest {
        self.test.clone()
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
