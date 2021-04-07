//! Representation of a [NamedNodeMap](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap)
//! and associated metadata.

use crate::behavior::sandbox_member_prelude::*;
use crate::internal_prelude::*;
use crate::node::AttrNode;

/// A [NamedNodeMap](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap) structure
pub struct NamedNodeMap {
    /// SandboxMember implementation
    pub context: SandboxMemberBehaviorStorage,

    /// Reference back up to the core element
    pub element: Weak<dyn element::AnyElement>,

    /// The attribute nodes
    pub attribute_list: Vec<Arc<AttrNode>>,
}

impl NamedNodeMap {
    fn new(context: Weak<Sandbox>, element: Weak<dyn element::AnyElement>) -> Arc<NamedNodeMap> {
        Arc::new(NamedNodeMap {
            context: SandboxMemberBehaviorStorage::new(context),
            element,
            attribute_list: Vec::new(),
        })
    }

    fn length(&self) -> usize {
        self.attribute_list.len()
    }

    fn item(&self, index: usize) -> Option<Arc<AttrNode>> {
        self.attribute_list.get(index).cloned()
    }
}

impl_sandbox_member!(NamedNodeMap, context);
