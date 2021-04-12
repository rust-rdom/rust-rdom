//! This module provides functionality connected to query selectors
// Made it a separate module for the scenario if we would need
// something more than simple query selectors

use std::{str::FromStr, sync::Arc};

use crate::internal_prelude::*;

/// A parsed selector type
pub struct Selector(String);

impl FromStr for Selector {
    type Err = DomError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // I could use simd optimizations here if you
        // allow unsafe code, @philip-peterson

        // validate string (only allow [A-Z] and [0-9])
        let s = s.to_uppercase();
        let valid = s.as_bytes().iter().all(|&v| {
            (v >= ('A' as u8) && v <= ('Z' as u8)) || (v >= ('0' as u8) && v <= ('9' as u8))
        });

        if valid {
            Ok(Selector(s))
        } else {
            Err(DomError::InvalidQuerySelector)
        }
    }
}

fn is_selected(node: &dyn AnyNode, selector: &Selector) -> bool {
    match node.as_element() {
        Some(element) => element.tag_name() == selector.0,
        None => false,
    }
}

/// Query selector function, does not check root node.
/// Does not return element, returns node.
pub fn query_selector(
    root: &dyn AnyNode,
    selector: &str,
) -> Result<Option<Arc<dyn AnyNode>>, DomError> {
    let selector = selector.parse::<Selector>()?;
    Ok(query_selector_rec(root, &selector))
}

fn query_selector_rec(root: &dyn AnyNode, selector: &Selector) -> Option<Arc<dyn AnyNode>> {
    root.static_child_nodes().into_iter().find_map(|node| {
        if is_selected(&*node, selector) {
            Some(node)
        } else {
            query_selector_rec(&*node, selector)
        }
    })
}
