/*!
This provides the evaluation implementation, it takes a `NodeSet` as the context and a parsed
XPath `LocationPath`.

# Example

*/

use crate::xpath1::model::{AxisSpecifier, LocationPath, Step};
use xml_dom::level2::NodeType;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub enum EvaluationError {
    NoDocumentNode,
    InvalidDocumentNode,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn evaluate_path(node_set: &NodeSet, xpath: &LocationPath) -> NodeSet {
    let mut next_set = if xpath.is_root() {
        node_set.document()
    } else {
        node_set.clone()
    };
    for step in xpath.steps() {
        next_set = filter_nodes(&select_nodes(node_set, step), step)
    }
    next_set
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn select_nodes(node_set: &NodeSet, step: &Step) -> NodeSet {
    let select_expr = step.select_expr();
    match select_expr.axis_specifier() {
        AxisSpecifier::Ancestor => node_set.ancestor(),
        AxisSpecifier::AncestorOrSelf => node_set.ancestor_or_self(),
        AxisSpecifier::Attribute => node_set.attribute(),
        AxisSpecifier::Child => node_set.child(),
        AxisSpecifier::Descendant => node_set.descendant(),
        AxisSpecifier::DescendantOrSelf => node_set.descendant_or_self(),
        AxisSpecifier::Following => node_set.following(),
        AxisSpecifier::FollowingSibling => node_set.following_sibling(),
        AxisSpecifier::Namespace => node_set.namespace(),
        AxisSpecifier::Parent => node_set.parent(),
        AxisSpecifier::Preceding => node_set.preceding(),
        AxisSpecifier::PrecedingSibling => node_set.preceding_sibling(),
        AxisSpecifier::SelfNode => node_set.self_node(),
    }
}

fn filter_nodes(node_set: &NodeSet, step: &Step) -> NodeSet {
    let mut filters: Vec<Box<dyn Filter>> = Vec::new();

    let select_expr = step.select_expr();
    let filter = Box::new(NodeTestFilter::new(
        match select_expr.axis_specifier() {
            AxisSpecifier::Attribute => NodeType::Attribute,
            _ => NodeType::Element,
        },
        select_expr.node_test(),
    ));
    filters.push(filter);

    for predicate in step.predicate_exprs() {
        let filter = Box::new(PredicateFilter::new(predicate.clone()));
        filters.push(filter);
    }

    node_set
        .iter()
        .filter(|node| filters.iter().all(|filter| filter.apply(node)))
        .cloned()
        .collect()
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod filters;
use filters::{Filter, NodeTestFilter, PredicateFilter};

mod node_set;
pub use node_set::NodeSet;

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use xml_dom::level2::{Node, RefNode};
    use xml_dom::parser::read_xml;

    fn make_test_document() -> RefNode {
        let xml = include_str!("../../../tests/example.xml");
        let document_node = read_xml(xml).unwrap();
        document_node.clone()
    }

    fn check_result_nodes(node_set: NodeSet, count: usize, node_type: NodeType) {
        assert_eq!(node_set.len(), count);
        assert!(node_set.iter().all(|node| node.node_type() == node_type));
        println!("{:#?}", node_set);
    }

    // --------------------------------------------------------------------------------------------

    #[test]
    fn test_document_all_self_nodes() {
        let document_node = make_test_document();
        let mut xpath = LocationPath::default();
        let xpath = xpath.all_self();

        let result = evaluate_path(&NodeSet::from(document_node), &xpath);
        check_result_nodes(result, 1, NodeType::Document);
    }

    #[test]
    fn test_document_all_self_elements() {
        let document_node = make_test_document();
        let mut xpath = LocationPath::default();
        let xpath = xpath.all_self_elements();

        let result = evaluate_path(&NodeSet::from(document_node), &xpath);
        check_result_nodes(result, 0, NodeType::Element);
    }

    #[test]
    fn test_document_all_child_elements() {
        let document_node = make_test_document();
        let mut xpath = LocationPath::default();
        let xpath = xpath.all_child_elements();

        let result = evaluate_path(&NodeSet::from(document_node), &xpath);
        check_result_nodes(result, 1, NodeType::Element);
    }

    #[test]
    fn test_document_all_child_elements_named_catalog() {
        let document_node = make_test_document();
        let mut xpath = LocationPath::default();
        let xpath = xpath.child_elements("catalog");

        let result = evaluate_path(&NodeSet::from(document_node), &xpath);
        check_result_nodes(result, 1, NodeType::Element);
    }

    #[test]
    fn test_document_all_child_elements_named_books() {
        let document_node = make_test_document();
        let mut xpath = LocationPath::default();
        let xpath = xpath.child_elements("books");

        let result = evaluate_path(&NodeSet::from(document_node), &xpath);
        check_result_nodes(result, 0, NodeType::Element);
    }
}
