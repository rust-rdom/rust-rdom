#![macro_use]

use crate::internal_prelude::*;
use crate::node::contents::NodeType;
use crate::node::NodeCommon;

pub trait ParentNodeBehavior {
    fn child_element_count(&self) -> Result<usize, DomError>;
}

pub struct ParentNodeBehaviorStorage {
    pub(crate) node: Weak<NodeCommon>,
}

impl ParentNodeBehaviorStorage {
    pub fn new(node: Weak<NodeCommon>) -> ParentNodeBehaviorStorage {
        ParentNodeBehaviorStorage { node }
    }

    pub fn child_element_count(&self) -> Result<usize, DomError> {
        let node = self.node.upgrade().ok_or(DomError::SandboxDropped)?;
        Ok(node
            .node_graph
            .static_child_nodes()
            .iter()
            .filter(|node| node.contents.to_node_type() == NodeType::Element)
            .count())
    }
}

#[macro_export]
/// Implements ParentBehavior
macro_rules! impl_parent_node {
    ($structname: ty, $($fieldname: ident).+) => {
        paste::paste! {
            impl ParentNodeBehavior for $structname {
                fn child_element_count(&self) -> Result<usize, DomError> {
                    self.$($fieldname).+.child_element_count()
                }
            }
        }
    };
}
