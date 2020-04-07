/*!
The type `NodeSet` represents both the input to, and output from, each step in a path.



*/

use std::collections::vec_deque::Iter;
use std::collections::VecDeque;
use std::iter::FromIterator;
use xml_dom::level2::{Node, NodeType, RefNode};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A container of DOM nodes. This is not truly a set, there may be duplicates and the order of
/// items is defined by the individual axis functions, by default it is document order. The name
/// reflects the type introduced in the XPath 1.0 specification, §3.3
/// [Node Sets](https://www.w3.org/TR/xpath-10/#node-sets).
///
#[derive(Clone, Debug, PartialEq)]
pub struct NodeSet(VecDeque<RefNode>);

// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

macro_rules! veq {
    () => {
        VecDeque::new()
    };
    ($node:expr) => {{
        let mut new_vec: VecDeque<RefNode> = Default::default();
        new_vec.push_back($node);
        new_vec
    }};
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for NodeSet {
    fn default() -> Self {
        Self(Default::default())
    }
}

// ------------------------------------------------------------------------------------------------

impl From<RefNode> for NodeSet {
    fn from(node: RefNode) -> Self {
        Self(veq![node])
    }
}

// ------------------------------------------------------------------------------------------------

impl From<&RefNode> for NodeSet {
    fn from(node: &RefNode) -> Self {
        Self(veq![node.clone()])
    }
}

// ------------------------------------------------------------------------------------------------

impl FromIterator<RefNode> for NodeSet {
    fn from_iter<T: IntoIterator<Item = RefNode>>(iter: T) -> Self {
        Self(VecDeque::from_iter(iter))
    }
}

// ------------------------------------------------------------------------------------------------

impl NodeSet {
    pub fn iter(&self) -> Iter<'_, RefNode> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    ///
    /// The `ancestor` axis contains the ancestors of the context node; the ancestors of the context
    /// node consist of the parent of context node and the parent's parent and so on; thus, the
    /// ancestor axis will always include the root node, unless the context node is the root node.
    ///
    /// # Result
    ///
    /// ```text
    ///                        [2]
    ///                         |
    ///       ,-----------,-----'-----,-----------,
    ///      [ ]         [1]         [ ]         [ ]
    ///       |           |           |
    ///    ,--'--,     ,--'--,     ,--'--,
    ///   [ ]   [ ]   [ ]   (X)   [ ]   [ ]
    ///                      |
    ///                ,-----|-----,
    ///               [ ]   [ ]   [ ]
    /// ```
    ///
    pub fn ancestor(&self) -> Self {
        fn parents(node: &RefNode) -> VecDeque<RefNode> {
            let mut result = NodeSet::inner_new();
            let mut next = node.parent_node();
            while let Some(node) = next {
                result.push_back(node.clone());
                next = node.parent_node();
            }
            result
        }
        Self(self.iter().map(parents).flatten().collect())
    }

    ///
    /// The `ancestor-or-self` axis contains the context node and the ancestors of the context node;
    /// thus, the ancestor axis will always include the root node.
    ///
    /// # Result
    ///
    /// ```text
    ///                        [3]
    ///                         |
    ///       ,-----------,-----'-----,-----------,
    ///      [ ]         [2]         [ ]         [ ]
    ///       |           |           |
    ///    ,--'--,     ,--'--,     ,--'--,
    ///   [ ]   [ ]   [ ]   (1)   [ ]   [ ]
    ///                      |
    ///                ,-----|-----,
    ///               [ ]   [ ]   [ ]
    /// ```
    ///
    pub fn ancestor_or_self(&self) -> Self {
        fn parents(node: &RefNode) -> VecDeque<RefNode> {
            let mut result = NodeSet::inner_new();
            result.push_back(node.clone());
            let mut next = node.parent_node();
            while let Some(node) = next {
                result.push_back(node.clone());
                next = node.parent_node();
            }
            result
        }
        Self(self.iter().map(parents).flatten().collect())
    }

    ///
    /// The `attribute` axis contains the attributes of the context node; the axis will be empty
    /// unless the context node is an element.
    ///
    /// The result excludes any attributes on the context element where `local_name`, or `prefix`,
    /// is 'xmlns'.
    ///
    pub fn attribute(&self) -> Self {
        Self(
            self.iter()
                .filter_map(|node| {
                    if node.node_type() == NodeType::Element {
                        let attribute_hash = node.attributes();
                        let attribute_nodes: VecDeque<RefNode> = attribute_hash
                            .iter()
                            .filter_map(|(name, node)| {
                                if name.is_namespace_attribute() {
                                    None
                                } else {
                                    Some(node.clone())
                                }
                            })
                            .collect();
                        Some(attribute_nodes)
                    } else {
                        None
                    }
                })
                .flatten()
                .collect(),
        )
    }

    ///
    /// The `child` axis contains the children of the context node.
    ///
    /// # Result
    ///
    /// ```text
    ///                        [ ]
    ///                         |
    ///       ,-----------,-----'-----,-----------,
    ///      [ ]         (X)         [ ]         [ ]
    ///       |           |           |
    ///    ,--'--,     ,--'--,     ,--'--,
    ///   [ ]   [ ]   [1]   [2]   [ ]   [ ]
    ///                      |
    ///                ,-----|-----,
    ///               [ ]   [ ]   [ ]
    /// ```
    ///
    pub fn child(&self) -> Self {
        Self(
            self.iter()
                .map(|node| {
                    node.child_nodes()
                        .iter()
                        .cloned()
                        .collect::<VecDeque<RefNode>>()
                })
                .flatten()
                .collect(),
        )
    }

    ///
    /// The `descendant` axis contains the descendants of the context node; a descendant is a child
    /// or a child of a child and so on; thus the descendant axis never contains attribute or
    /// namespace nodes.
    ///
    /// # Result
    ///
    /// ```text
    ///                        [ ]
    ///                         |
    ///       ,-----------,-----'-----,-----------,
    ///      [ ]         (X)         [ ]         [ ]
    ///       |           |           |
    ///    ,--'--,     ,--'--,     ,--'--,
    ///   [ ]   [ ]   [1]   [2]   [ ]   [ ]
    ///                      |
    ///                ,-----|-----,
    ///               [3]   [4]   [5]
    /// ```
    ///
    pub fn descendant(&self) -> Self {
        let mut descendants = self.child();
        let mut next = descendants
            .iter()
            .map(|node| NodeSet::from(node).descendant().into_inner())
            .flatten()
            .collect::<VecDeque<RefNode>>();
        descendants.append(&mut next);
        descendants
    }

    ///
    /// The `descendant-or-self` axis contains the context node and the descendants of the context
    /// node.
    ///
    /// # Result
    ///
    /// ```text
    ///                        [ ]
    ///                         |
    ///       ,-----------,-----'-----,-----------,
    ///      [ ]         (1)         [ ]         [ ]
    ///       |           |           |
    ///    ,--'--,     ,--'--,     ,--'--,
    ///   [ ]   [ ]   [2]   [3]   [ ]   [ ]
    ///                      |
    ///                ,-----|-----,
    ///               [4]   [5]   [6]
    /// ```
    ///
    pub fn descendant_or_self(&self) -> Self {
        let mut descendants = self.self_node();
        let mut next = descendants
            .iter()
            .map(|node| NodeSet::from(node).descendant().into_inner())
            .flatten()
            .collect::<VecDeque<RefNode>>();
        descendants.append(&mut next);
        descendants
    }

    ///
    /// Non-standard axis, used to select the document node (if present) for all input nodes.
    ///
    /// # Result
    ///
    /// ```text
    ///                        [1] <-- IFF node_type == Document
    ///                         |
    ///       ,-----------,-----'-----,-----------,
    ///      [ ]         [ ]         [ ]         [ ]
    ///       |           |           |
    ///    ,--'--,     ,--'--,     ,--'--,
    ///   [ ]   [ ]   [ ]   (X)   [ ]   [ ]
    ///                      |
    ///                ,-----|-----,
    ///               [ ]   [ ]   [ ]
    /// ```
    ///
    pub fn document(&self) -> Self {
        Self(
            self.iter()
                .filter_map(|node| node.owner_document())
                .collect(),
        )
    }

    ///
    /// The `following` axis contains all nodes in the same document as the context node that are
    /// after the context node in document order, excluding any descendants and excluding attribute
    /// nodes and namespace nodes.
    ///
    /// # Result
    ///
    /// ```text
    ///                        [ ]
    ///                         |
    ///       ,-----------,-----'-----,-----------,
    ///      [ ]         (X)         [1]         [4]
    ///       |           |           |
    ///    ,--'--,     ,--'--,     ,--'--,
    ///   [ ]   [ ]   [ ]   [ ]   [2]   [3]
    ///                      |
    ///                ,-----|-----,
    ///               [ ]   [ ]   [ ]
    /// ```
    ///
    pub fn following(&self) -> Self {
        Self(
            self.following_sibling()
                .iter()
                .map(|node| NodeSet::from(node).descendant_or_self().into_inner())
                .flatten()
                .collect(),
        )
    }

    ///
    /// The `following-sibling` axis contains all the following siblings of the context node; if
    /// the context node is an attribute node or namespace node, the `following-sibling` axis is
    /// empty.
    ///
    ///  # Result
    ///
    /// ```text
    ///                        [ ]
    ///                         |
    ///       ,-----------,-----'-----,-----------,
    ///      [ ]         (X)         [1]         [2]
    ///       |           |           |
    ///    ,--'--,     ,--'--,     ,--'--,
    ///   [ ]   [ ]   [ ]   [ ]   [ ]   [ ]
    ///                      |
    ///                ,-----|-----,
    ///               [ ]   [ ]   [ ]
    /// ```
    ///
    pub fn following_sibling(&self) -> Self {
        fn siblings(node: &RefNode) -> VecDeque<RefNode> {
            let mut result = NodeSet::inner_new();
            let mut next = node.next_sibling();
            while let Some(sibling) = next {
                result.push_back(sibling.clone());
                next = sibling.next_sibling();
            }
            result
        }
        Self(self.iter().map(siblings).flatten().collect())
    }

    ///
    /// The `namespace` axis contains the namespace nodes of the context node; the axis will be
    /// empty unless the context node is an element.
    ///
    /// The result contains only the attributes on the context element where `local_name`, or
    /// `prefix`, is 'xmlns'.
    ///
    pub fn namespace(&self) -> Self {
        Self(
            self.iter()
                .filter_map(|node| {
                    if node.node_type() == NodeType::Element {
                        let attribute_hash = node.attributes();
                        let attribute_nodes: VecDeque<RefNode> = attribute_hash
                            .iter()
                            .filter_map(|(name, node)| {
                                if name.is_namespace_attribute() {
                                    Some(node.clone())
                                } else {
                                    None
                                }
                            })
                            .collect();
                        Some(attribute_nodes)
                    } else {
                        None
                    }
                })
                .flatten()
                .collect(),
        )
    }

    ///
    /// The parent axis contains the parent of the context node, if there is one.
    ///
    /// # Result
    ///
    /// ```text
    ///                        [1]
    ///                         |
    ///       ,-----------,-----'-----,-----------,
    ///      [ ]         (X)         [ ]         [ ]
    ///       |           |           |
    ///    ,--'--,     ,--'--,     ,--'--,
    ///   [ ]   [ ]   [ ]   [ ]   [ ]   [ ]
    ///                      |
    ///                ,-----|-----,
    ///               [ ]   [ ]   [ ]
    /// ```
    ///
    pub fn parent(&self) -> Self {
        Self(self.iter().filter_map(|node| node.parent_node()).collect())
    }

    ///
    /// The `preceding` axis contains all nodes in the same document as the context node that are
    /// before the context node in document order, excluding any ancestors and excluding attribute
    /// nodes and namespace nodes.
    ///
    /// **Note**: the order of nodes is based on adjacency, in effect reverse document order.
    ///
    /// # Result
    ///
    /// ```text
    ///                        [ ]
    ///                         |
    ///       ,-----------,-----'-----,-----------,
    ///      [3]         (X)         [ ]         [ ]
    ///       |           |           |
    ///    ,--'--,     ,--'--,     ,--'--,
    ///   [2]   [1]   [ ]   [ ]   [ ]   [ ]
    ///                      |
    ///                ,-----|-----,
    ///               [ ]   [ ]   [ ]
    /// ```
    ///
    pub fn preceding(&self) -> Self {
        fn reverse_descendant(node_set: &NodeSet) -> VecDeque<RefNode> {
            let mut children: VecDeque<RefNode> = node_set.child().iter().rev().cloned().collect();
            let mut next: VecDeque<RefNode> = children
                .iter()
                .map(|node| reverse_descendant(&NodeSet::from(node)))
                .flatten()
                .collect::<VecDeque<RefNode>>();
            children.append(&mut next);
            children
        }

        let mut previous: VecDeque<RefNode> =
            self.preceding_sibling().iter().rev().cloned().collect();
        let mut next = previous
            .iter()
            .map(|node| reverse_descendant(&NodeSet::from(node)))
            .flatten()
            .collect::<VecDeque<RefNode>>();
        previous.append(&mut next);
        Self(previous)
    }

    ///
    /// The `preceding-sibling` axis contains all the preceding siblings of the context node; if the
    /// context node is an attribute node or namespace node, the `preceding-sibling` axis is empty.
    ///
    /// # Result
    ///
    /// ```text
    ///                        [ ]
    ///                         |
    ///       ,-----------,-----'-----,-----------,
    ///      [1]         (X)         [ ]         [ ]
    ///       |           |           |
    ///    ,--'--,     ,--'--,     ,--'--,
    ///   [ ]   [ ]   [ ]   [ ]   [ ]   [ ]
    ///                      |
    ///                ,-----|-----,
    ///               [ ]   [ ]   [ ]
    /// ```
    ///
    pub fn preceding_sibling(&self) -> Self {
        Self(
            self.iter()
                .map(|node| match node.parent_node() {
                    None => veq!(),
                    Some(parent) => {
                        let mut result = NodeSet::inner_new();
                        let siblings = parent.child_nodes();
                        let mut child_iter = siblings.iter();
                        while let Some(child) = child_iter.next() {
                            if child == node {
                                break;
                            }
                            result.push_back(child.clone());
                        }
                        result
                    }
                })
                .flatten()
                .collect(),
        )
    }

    ///
    /// The `self` axis contains just the context node itself.
    ///
    /// # Result
    ///
    /// ```text
    ///                        [ ]
    ///                         |
    ///       ,-----------,-----'-----,-----------,
    ///      [ ]         (1)         [ ]         [ ]
    ///       |           |           |
    ///    ,--'--,     ,--'--,     ,--'--,
    ///   [ ]   [ ]   [ ]   [ ]   [ ]   [ ]
    ///                      |
    ///                ,-----|-----,
    ///               [ ]   [ ]   [ ]
    /// ```
    ///
    pub fn self_node(&self) -> Self {
        Self(self.iter().cloned().collect())
    }

    fn into_inner(self) -> VecDeque<RefNode> {
        self.0
    }

    fn inner_new() -> VecDeque<RefNode> {
        Default::default()
    }

    fn append(&mut self, vector: &mut VecDeque<RefNode>) {
        self.0.append(vector)
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use xml_dom::level2::convert::as_document;
    use xml_dom::level2::Element;
    use xml_dom::parser::read_xml;

    //
    // ```text
    //                        [A]
    //                         |
    //       ,-----------,-----'-----,-----------,
    //      [B]         [E]         [K]         [N]
    //       |           |           |
    //    ,--'--,     ,--'--,     ,--'--,
    //   [C]   [D]   [F]   [G]   [L]   [M]
    //                      |
    //                ,-----|-----,
    //               [H]   [I]   [J]
    // ```
    //
    fn make_test_document() -> RefNode {
        const TEST_XML: &str = r##"<?xml version="1.0"?>
<book xml:id="A">
  <chapter xml:id="B">
    <section xml:id="C">
      This is the first section of chapter 1.
    </section>
    <section xml:id="D" incomplete="true">
    </section>
  </chapter>
  <chapter xml:id="E">
    <section xml:id="F">
    </section>
    <section xml:id="G">
      <sub-section xml:id="H">
      </sub-section>
      <sub-section xml:id="I">
      </sub-section>
      <sub-section xml:id="J">
      </sub-section>
    </section>
  </chapter>
  <chapter xml:id="K">
    <section xml:id="L">
    </section>
    <section xml:id="M" incomplete="true">
    </section>
  </chapter>
  <chapter xml:id="N" incomplete="true">
  </chapter>
</book>"##;
        let document_node = read_xml(TEST_XML).unwrap();
        document_node.clone()
    }

    fn assert_equal_ids(node_set: NodeSet, ids: &[&str]) {
        assert_eq!(
            node_set
                .iter()
                .filter_map(|node| node.get_attribute("xml:id"))
                .collect::<Vec<String>>(),
            ids.iter().map(|s| s.to_string()).collect::<Vec<String>>()
        );
    }

    fn print_node_set(node_set: &NodeSet, label: Option<&str>) {
        println!(
            "=========={}==========",
            if let Some(label) = label {
                format!(" {} ", label)
            } else {
                String::new()
            }
        );
        node_set.iter().for_each(|node| println!(":: {}", node));
    }

    #[test]
    fn test_ancester() {
        let document_node = make_test_document();
        let document = as_document(&document_node).unwrap();
        let context_node = document.get_element_by_id("G").unwrap();
        let node_set = NodeSet::from(context_node);

        let node_set = node_set.ancestor();

        assert_eq!(node_set.len(), 3);
        assert_equal_ids(node_set, &["E", "A"]);
    }

    #[test]
    fn test_ancester_or_self() {
        let document_node = make_test_document();
        let document = as_document(&document_node).unwrap();
        let context_node = document.get_element_by_id("G").unwrap();
        let node_set = NodeSet::from(context_node);

        let node_set = node_set.ancestor_or_self();

        assert_eq!(node_set.len(), 4);
        assert_equal_ids(node_set, &["G", "E", "A"]);
    }

    #[test]
    fn test_child() {
        let document_node = make_test_document();
        let document = as_document(&document_node).unwrap();
        let context_node = document.get_element_by_id("E").unwrap();
        let node_set = NodeSet::from(context_node);

        let node_set = node_set.child();

        assert_eq!(node_set.len(), 2);
        assert_equal_ids(node_set, &["F", "G"]);
    }

    #[test]
    fn test_descendant() {
        let document_node = make_test_document();
        let document = as_document(&document_node).unwrap();
        let context_node = document.get_element_by_id("E").unwrap();
        let node_set = NodeSet::from(context_node);

        let node_set = node_set.descendant();

        print_node_set(&node_set, Some("descendant"));
        assert_eq!(node_set.len(), 5);
        assert_equal_ids(node_set, &["F", "G", "H", "I", "J"]);
    }

    #[test]
    fn test_descendant_or_self() {
        let document_node = make_test_document();
        let document = as_document(&document_node).unwrap();
        let context_node = document.get_element_by_id("E").unwrap();
        let node_set = NodeSet::from(context_node);

        let node_set = node_set.descendant_or_self();

        print_node_set(&node_set, Some("descendant_or_self"));
        assert_eq!(node_set.len(), 6);
        assert_equal_ids(node_set, &["E", "F", "G", "H", "I", "J"]);
    }

    #[test]
    fn test_following() {
        let document_node = make_test_document();
        let document = as_document(&document_node).unwrap();
        let context_node = document.get_element_by_id("E").unwrap();
        let node_set = NodeSet::from(context_node);

        let node_set = node_set.following();

        print_node_set(&node_set, Some("following"));
        assert_eq!(node_set.len(), 4);
        assert_equal_ids(node_set, &["K", "L", "M", "N"]);
    }

    #[test]
    fn test_following_sibling() {
        let document_node = make_test_document();
        let document = as_document(&document_node).unwrap();
        let context_node = document.get_element_by_id("E").unwrap();
        let node_set = NodeSet::from(context_node);

        let node_set = node_set.following_sibling();

        print_node_set(&node_set, Some("following_sibling"));
        assert_eq!(node_set.len(), 2);
        assert_equal_ids(node_set, &["K", "N"]);
    }

    #[test]
    fn test_parent() {
        let document_node = make_test_document();
        let document = as_document(&document_node).unwrap();
        let context_node = document.get_element_by_id("E").unwrap();
        let node_set = NodeSet::from(context_node);

        let node_set = node_set.parent();

        print_node_set(&node_set, Some("parent"));
        assert_eq!(node_set.len(), 1);
        assert_equal_ids(node_set, &["A"]);
    }

    #[test]
    fn test_preceding() {
        let document_node = make_test_document();
        let document = as_document(&document_node).unwrap();
        let context_node = document.get_element_by_id("E").unwrap();
        let node_set = NodeSet::from(context_node);

        let node_set = node_set.preceding();

        print_node_set(&node_set, Some("preceding"));
        assert_eq!(node_set.len(), 4);
        assert_equal_ids(node_set, &["B", "D", "C"]);
    }

    #[test]
    fn test_preceding_sibling() {
        let document_node = make_test_document();
        let document = as_document(&document_node).unwrap();
        let context_node = document.get_element_by_id("E").unwrap();
        let node_set = NodeSet::from(context_node);

        let node_set = node_set.preceding_sibling();

        print_node_set(&node_set, Some("preceding_sibling"));
        assert_eq!(node_set.len(), 1);
        assert_equal_ids(node_set, &["B"]);
    }
}
