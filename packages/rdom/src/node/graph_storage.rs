use crate::behavior::sandbox_member::SandboxMemberBehavior;
use crate::internal_prelude::*;
use crate::node::concrete::ElementNodeArc;
use crate::node_list::{NodeList, NodeListStorage, Query};
use std::{
    convert::{TryFrom, TryInto},
    sync::RwLock,
};

/// NodeGraphStorage contains all the data connected
/// to graph of the nodes
pub struct NodeGraphStorage {
    /// Reference back up to the common Node
    node: AnyNodeWeak,

    parent_node: Option<AnyNodeWeak>,
    left_sibling: Option<AnyNodeWeak>,
    right_sibling: Option<AnyNodeWeak>,
    child_nodes: RwLock<Vec<AnyNodeArc>>,
}

impl NodeGraphStorage {
    /// Constructs a new NodeGraphStorage
    pub fn new(node: AnyNodeWeak) -> NodeGraphStorage {
        NodeGraphStorage {
            node,
            parent_node: None,
            left_sibling: None,
            right_sibling: None,
            child_nodes: RwLock::new(Vec::new()),
        }
    }

    pub(crate) fn first_child(&self) -> Option<AnyNodeArc> {
        let lock = self.child_nodes.read().unwrap();
        (*lock).first().cloned()
    }

    pub(crate) fn last_child(&self) -> Option<AnyNodeArc> {
        let lock = self.child_nodes.read().unwrap();
        (*lock).last().cloned()
    }

    pub(crate) fn append_child(&self, other: AnyNodeArc) {
        let mut lock = self.child_nodes.write().unwrap();
        (*lock).push(other);
    }

    pub(crate) fn static_child_nodes(&self) -> Vec<AnyNodeArc> {
        self.child_nodes.read().unwrap().clone()
    }

    pub(crate) fn child_nodes(&self) -> Arc<NodeList> {
        let strong_ref = self.node.upgrade().expect("Sandbox dropped");

        NodeList::new(
            strong_ref.get_context(),
            NodeListStorage::Live(Query::ChildNodes {
                children_of: strong_ref,
            }),
        )
    }

    // IMPORTANT: this function does not check the element itself, it only checks children
    pub(crate) fn query_selector_rec(&self, selector: &Selector) -> Option<ElementNodeArc> {
        self.static_child_nodes().into_iter().find_map(|child| {
            match selector.filter_selected_node(child) {
                Ok(element) => Some(element),
                Err(node) => node.common.node_graph.query_selector_rec(selector),
            }
        })
    }
}

pub struct Selector(String);

impl TryFrom<String> for Selector {
    type Error = DomError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // validate string (only allow [A-Z] and [0-9])
        let value = value.to_uppercase();
        let valid = value.as_bytes().iter().all(|&v| {
            (v >= ('A' as u8) && v <= ('Z' as u8)) || (v >= ('0' as u8) && v <= ('9' as u8))
        });

        if valid {
            Ok(Selector(value))
        } else {
            Err(DomError::InvalidQuerySelector)
        }
    }
}

impl TryFrom<&str> for Selector {
    type Error = DomError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Selector::try_from(value.to_string())
    }
}

impl Selector {
    pub fn filter_selected_node(&self, node: AnyNodeArc) -> Result<ElementNodeArc, AnyNodeArc> {
        match <_ as TryInto<ElementNodeArc>>::try_into(node.clone()) {
            Ok(element) => {
                if self.is_selected_element(element.clone()) {
                    Ok(element)
                } else {
                    Err(node)
                }
            }
            Err(_) => Err(node),
        }
    }

    pub fn is_selected_element(&self, element: ElementNodeArc) -> bool {
        element.contents.tag_name() == self.0
    }
}
