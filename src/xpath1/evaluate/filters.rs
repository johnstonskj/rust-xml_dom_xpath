/*!


*/

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

use crate::xpath1::model::{NodeTest, Predicate};
use std::str::FromStr;
use xml_dom::level2::{Name, Node, NodeType, ProcessingInstruction, RefNode};

pub trait Filter {
    fn apply(&self, to: &RefNode) -> bool;
}

//
// Every axis has a principal node type. If an axis can contain elements, then the principal node
// type is element; otherwise, it is the type of the nodes that the axis can contain. Thus,
//
// * For the attribute axis, the principal node type is attribute.
// * For the namespace axis, the principal node type is namespace.
// * For other axes, the principal node type is element.
//
pub struct NodeTestFilter {
    principal_type: NodeType,
    node_test: NodeTest,
}

#[allow(dead_code)]
pub struct PredicateFilter {
    predicate: Predicate,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Filter for NodeTestFilter {
    fn apply(&self, node: &RefNode) -> bool {
        match &self.node_test {
            NodeTest::All => node.node_type() == self.principal_type,
            NodeTest::Named(name) => {
                if node.node_type() == self.principal_type {
                    let name = Name::from_str(&name).unwrap();
                    node.node_name() == name
                } else {
                    false
                }
            }
            NodeTest::Comment => node.node_type() == NodeType::Comment,
            NodeTest::Text => node.node_type() == NodeType::Text,
            NodeTest::ProcessingInstruction(None) => {
                node.node_type() == NodeType::ProcessingInstruction
            }
            NodeTest::ProcessingInstruction(Some(target)) => {
                node.node_type() == NodeType::ProcessingInstruction
                    && node.target() == target.clone()
            }
            NodeTest::Node => true,
        }
    }
}

impl NodeTestFilter {
    pub fn new(principal_type: NodeType, node_test: NodeTest) -> Self {
        Self {
            principal_type,
            node_test,
        }
    }
}

impl Filter for PredicateFilter {
    fn apply(&self, _to: &RefNode) -> bool {
        unimplemented!()
    }
}

impl PredicateFilter {
    pub fn new(predicate: Predicate) -> Self {
        Self { predicate }
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
