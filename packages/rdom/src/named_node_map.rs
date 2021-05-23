//! Representation of a [NamedNodeMap](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap)
//! and associated metadata.

use crate::internal_prelude::*;
use crate::node::concrete::{AttributeNodeArc, ElementNodeWeak};
use std::convert::TryInto;

/// A [NamedNodeMap](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap) structure
#[sourcegen::sourcegen(generator = "behave", script = "SandboxMember context")]
// Generated. All manual edits to the block annotated with #[sourcegen...] will be discarded.
pub struct NamedNodeMap {
    /// SandboxMember implementation
    pub(crate) context: Weak<Sandbox>,
    /// The attribute nodes
    pub(crate) attribute_list: Vec<AttributeNodeArc>,
    pub(crate) owning_element: ElementNodeWeak,
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
    pub(crate) fn new(
        context: Weak<Sandbox>,
        owning_element: ElementNodeWeak,
    ) -> Arc<NamedNodeMap> {
        Arc::new(NamedNodeMap {
            context,
            attribute_list: Vec::new(),
            owning_element,
        })
    }

    /// Gets an attribute node given its name.
    pub fn get_named_item(&self, name: String) -> Option<AttributeNodeArc> {
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

    /// Replaces or appends an attribute node. Returns the old attribute node (if applicable).
    pub fn set_named_item(
        &mut self,
        item: AttributeNodeArc,
    ) -> Result<Option<AttributeNodeArc>, DomError> {
        match item.contents.element.clone() {
            Some(element) if element != self.owning_element => Err(DomError::InUseAttribute),
            _ => {
                let name = item.contents.get_name();
                let name = name.to_ascii_lowercase();
                let existing_index = self.get_attribute_idx(name);
                Ok(if let Some(existing_index) = existing_index {
                    let existing_attr = self.attribute_list[existing_index].clone();
                    if existing_attr == item {
                        Some(item)
                    } else {
                        self.replace_attribute(existing_index, item);
                        Some(existing_attr)
                    }
                } else {
                    self.attribute_list.push(item);
                    None
                })
            }
        }
    }

    /// Removes an attribute
    pub fn remove_named_item(
        &mut self,
        qualified_name: String,
    ) -> Result<AttributeNodeArc, DomError> {
        let name = qualified_name.to_ascii_lowercase();
        let existing_index = self.get_attribute_idx(name);
        match existing_index {
            None => Err(DomError::NotFound),
            Some(existing_index) => Ok(self.attribute_list.remove(existing_index)),
        }
    }

    // TODO: namespaces
    fn get_attribute_idx(&self, local_name: String) -> Option<usize> {
        self.attribute_list.iter().position(|attr| {
            let attr: AttributeNodeArc = attr
                .clone()
                .try_into()
                .expect("Node in NamedNodeMap was not an Attr node");
            attr.contents.get_name() == local_name
        })
    }

    fn replace_attribute(&mut self, index: usize, item: AttributeNodeArc) -> AttributeNodeArc {
        let old = self.attribute_list[index].clone();
        self.attribute_list[index] = item;
        old
    }

    /// Gives the number of attributes
    fn length(&self) -> usize {
        self.attribute_list.len()
    }

    /// Gives the attribute item at `index`
    pub fn item(&self, index: usize) -> Option<AttributeNodeArc> {
        self.attribute_list.get(index).cloned()
    }
}
