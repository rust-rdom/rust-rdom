//! Representation of a [NamedNodeMap](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap)
//! and associated metadata.

use crate::internal_prelude::*;
use crate::node::AnyNodeWeak;

/// A [NamedNodeMap](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap) structure
#[sourcegen::sourcegen(generator = "behave", script = "SandboxMember context")]
// Generated. All manual edits to the block annotated with #[sourcegen...] will be discarded.
pub struct NamedNodeMap {
    /// SandboxMember implementation
    pub context: Weak<Sandbox>,
    /// The attribute nodes. There is currently no static guarantee
    /// that they are actually attribute nodes as opposed to another kind
    /// of node. This is where it would be nice to have a `nice` representation.
    pub attribute_list: Vec<AnyNodeArc>,
    /// Reference back up to the core element
    /// would be nice to know this is actually an Element (same as above)
    pub element: AnyNodeWeak,
}

#[sourcegen::generated]
impl NamedNodeMap {
    /// gets `Weak<Sandbox>` to the `Sandbox` that it is in
    pub fn get_context(&self) -> Weak<Sandbox> {
        self.context.clone()
    }
}

#[sourcegen::generated]
impl SandboxMemberBehavior for NamedNodeMap {
    fn get_context(&self) -> Weak<Sandbox> {
        self.get_context()
    }
}

impl NamedNodeMap {
    fn new(context: Weak<Sandbox>, element: AnyNodeWeak) -> Arc<NamedNodeMap> {
        Arc::new(NamedNodeMap {
            context,
            element,
            attribute_list: Vec::new(),
        })
    }

    fn length(&self) -> usize {
        self.attribute_list.len()
    }

    fn item(&self, index: usize) -> Option<AnyNodeArc> {
        self.attribute_list.get(index).cloned()
    }
}
