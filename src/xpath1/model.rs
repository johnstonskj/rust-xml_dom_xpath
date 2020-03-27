/*!
One-line description.

More detailed description, with

# Example

*/

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
pub enum Predicate {
    Expr(ExprNode),
    Terminal(Terminal),
    Function(FunctionCall),
}

#[derive(Clone, Debug)]
pub enum ExprNode {
    And {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    Or {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    Equals {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    NotEquals {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    LessThan {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    LessThanOrEqual {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    GreaterThan {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    GreaterThanOrEqual {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    Add {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    Subtract {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    Multiply {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    Divide {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    Modulus {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    FPDiv {
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
    UnaryMinus {
        value: Box<Predicate>,
    },
}

#[derive(Clone, Debug)]
pub struct SelectExpr {
    axis: AxisSpecifier,
    node_test: NodeTest,
}

#[derive(Clone, Debug)]
pub enum Terminal {
    Variable(String),
    Literal(String),
    Number(i64),
    Float(f64),
    Select(SelectExpr),
}

#[derive(Clone, Debug)]
pub struct FunctionCall {
    name: String,
    arguments: Vec<Predicate>,
}

#[derive(Clone, Debug)]
pub struct Step {
    axis: AxisSpecifier,
    node_test: NodeTest,
    predicates: Vec<Predicate>,
}

#[derive(Clone, Debug)]
pub struct LocationPath {
    root: bool,
    steps: Vec<Step>,
}

// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

macro_rules! step_fn {
    ($fn_name:ident, $axis:ident, $node_test:ident) => {
        pub fn $fn_name() -> Self {
            Self {
                axis: AxisSpecifier::$axis,
                node_test: NodeTest::$node_test,
                predicates: Vec::default(),
            }
        }
    };
    ($fn_name:ident, $axis:ident, $node_test:ident, $name_name:ident) => {
        pub fn $fn_name($name_name: &str) -> Self {
            Self {
                axis: AxisSpecifier::$axis,
                node_test: NodeTest::$node_test($name_name.to_string()),
                predicates: Vec::default(),
            }
        }
    };
}

macro_rules! path_fn {
    ($fn_name:ident) => {
        pub fn $fn_name(&mut self) -> &mut Self {
            self.append(Step::$fn_name());
            self
        }
    };
    ($fn_name:ident, $name_name:ident) => {
        pub fn $fn_name(&mut self, $name_name: &str) -> &mut Self {
            self.append(Step::$fn_name($name_name));
            self
        }
    };
}
macro_rules! predicate_fn {
    ($fn_name:ident, $expr_t:ident) => {
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

impl Predicate {
    pub fn literal(value: &str) -> Self {
        Predicate::Terminal(Terminal::Literal(value.to_string()))
    }
    pub fn integer(value: i64) -> Self {
        Predicate::Terminal(Terminal::Number(value))
    }
    pub fn float(value: f64) -> Self {
        Predicate::Terminal(Terminal::Float(value))
    }
    pub fn variable(named: &str) -> Self {
        Predicate::Terminal(Terminal::Variable(named.to_string()))
    }
    pub fn function(named: &str) -> Self {
        Predicate::Function(FunctionCall {
            name: named.to_string(),
            arguments: Vec::default(),
        })
    }
    pub fn function_with(named: &str, args: &[Predicate]) -> Self {
        Predicate::Function(FunctionCall {
            name: named.to_string(),
            arguments: args.to_vec(),
        })
    }
    pub fn select(axis: AxisSpecifier, node_test: NodeTest) -> Self {
        Predicate::Terminal(Terminal::Select(SelectExpr { axis, node_test }))
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
        write!(
            f,
            "{}",
            match self {
                ExprNode::And { left, right } => format!("{} and {}", left, right),
                ExprNode::Or { left, right } => format!("{} or {}", left, right),
                ExprNode::Equals { left, right } => format!("{} = {}", left, right),
                ExprNode::NotEquals { left, right } => format!("{} != {}", left, right),
                ExprNode::LessThan { left, right } => format!("{} < {}", left, right),
                ExprNode::LessThanOrEqual { left, right } => format!("{} <= {}", left, right),
                ExprNode::GreaterThan { left, right } => format!("{} > {}", left, right),
                ExprNode::GreaterThanOrEqual { left, right } => format!("{} >= {}", left, right),
                ExprNode::Add { left, right } => format!("{} + {}", left, right),
                ExprNode::Subtract { left, right } => format!("{} - {}", left, right),
                ExprNode::Multiply { left, right } => format!("{} * {}", left, right),
                ExprNode::Divide { left, right } => format!("{} / {}", left, right),
                ExprNode::Modulus { left, right } => format!("{} mod {}", left, right),
                ExprNode::FPDiv { left, right } => format!("{} div {}", left, right),
                ExprNode::UnaryMinus { value } => format!("- {}", value),
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

impl Display for SelectExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}{}", self.axis, self.node_test,)
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

impl FunctionCall {
    pub fn append(&mut self, argument: Predicate) {
        self.arguments.push(argument);
    }
}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

impl Default for Step {
    fn default() -> Self {
        Self {
            axis: Default::default(),
            node_test: Default::default(),
            predicates: Default::default(),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Step {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}{}{}",
            self.axis,
            self.node_test,
            self.predicates
                .iter()
                .map(|p| format!("[{}]", p))
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Step {
    pub fn new(axis: AxisSpecifier, node_test: NodeTest) -> Self {
        Self {
            axis,
            node_test,
            predicates: Default::default(),
        }
    }

    pub fn new_with_predicate(
        axis: AxisSpecifier,
        node_test: NodeTest,
        predicate: Predicate,
    ) -> Self {
        Self {
            axis,
            node_test,
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
    step_fn!(ancestor_elements, Ancestor, Named, named);

    step_fn!(all_ancestors_or_self, AncestorOrSelf, Node);
    step_fn!(all_ancestor_or_self_elements, AncestorOrSelf, All);
    step_fn!(all_ancestor_or_self_text, AncestorOrSelf, Text);
    step_fn!(all_ancestor_or_self_comments, AncestorOrSelf, Comment);
    step_fn!(ancestor_or_self_elements, AncestorOrSelf, Named, named);

    step_fn!(all_attributes, Attribute, All);
    step_fn!(attributes, Attribute, Named, named);

    step_fn!(all_children, Child, Node);
    step_fn!(all_child_elements, Child, All);
    step_fn!(all_child_text, Child, Text);
    step_fn!(all_child_comments, Child, Comment);
    step_fn!(child_elements, Child, Named, named);

    step_fn!(all_descendants, Descendant, Node);
    step_fn!(all_descendant_elements, Descendant, All);
    step_fn!(all_descendant_text, Descendant, Text);
    step_fn!(all_descendant_comments, Descendant, Comment);
    step_fn!(descendant_elements, Descendant, Named, named);

    step_fn!(all_descendants_or_self, DescendantOrSelf, Node);
    step_fn!(all_descendant_or_self_elements, DescendantOrSelf, All);
    step_fn!(all_descendant_or_self_text, DescendantOrSelf, Text);
    step_fn!(all_descendant_or_self_comments, DescendantOrSelf, Comment);
    step_fn!(descendant_or_self_elements, DescendantOrSelf, Named, named);

    step_fn!(all_following, Following, Node);
    step_fn!(all_following_elements, Following, All);
    step_fn!(all_following_text, Following, Text);
    step_fn!(all_following_comments, Following, Comment);
    step_fn!(following_elements, Following, Named, named);

    step_fn!(all_self, SelfNode, Node);
    step_fn!(all_self_elements, SelfNode, All);
    step_fn!(all_self_text, SelfNode, Text);
    step_fn!(all_self_comments, SelfNode, Comment);
    step_fn!(self_elements, SelfNode, Named, named);
}

// ------------------------------------------------------------------------------------------------
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

    path_fn!(all_children);
    path_fn!(all_child_elements);
    path_fn!(all_child_text);
    path_fn!(all_child_comments);
    path_fn!(child_elements, named);

    path_fn!(all_attributes);
    path_fn!(attributes, named);

    path_fn!(all_descendants);
    path_fn!(all_descendant_elements);
    path_fn!(descendant_elements, named);

    pub fn append(&mut self, step: Step) -> &mut Self {
        self.steps.push(step);
        self
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------
