//! Representation of a [NamedNodeMap](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap)
//! and associated metadata.

use crate::internal_prelude::*;
use crate::node::concrete::AttributeNodeArc;
use std::convert::TryInto;

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
    pub(crate) fn new(context: Weak<Sandbox>) -> Arc<NamedNodeMap> {
        Arc::new(NamedNodeMap {
            context,
            attribute_list: Vec::new(),
        })
    }

    fn get_named_item(&self, name: String) -> Option<AttributeNodeArc> {
        let name = name.to_ascii_lowercase();
        self.attribute_list.iter().find_map(|attr| {
            let attr: AttributeNodeArc = attr
                .clone()
                .try_into()
                .expect("Node in NamedNodeMap was not an Attr node");
            if attr.contents.get_name() == name {
                Some(attr)
            } else {
                None
            }
        })
    }

    fn length(&self) -> usize {
        self.attribute_list.len()
    }

    fn item(&self, index: usize) -> Option<AnyNodeArc> {
        self.attribute_list.get(index).cloned()
    }
}
