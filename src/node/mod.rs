//! Core representation of a DOM node. See `nice` module for distinction from
//! nice representation.

use downcast_rs::DowncastSync;
use paste::paste;

use std::sync::{Arc, Weak};

use crate::behavior::NodeBehavior;
use crate::internal_prelude::*;
use crate::node_list::NodeList;
use crate::sandbox::Sandbox;
use crate::window::Window;

pub mod element;
pub(crate) mod private;

/// An input event
pub struct InputEvent {}

/// A base trait for all core node types
pub trait AnyNode: DowncastSync + PrivateAnyNode {
    /// Gives a weak reference to the sandbox the node was created in.
    fn get_context(&self) -> Weak<Sandbox>;

    /// Clones the node
    fn clone_node(&self) -> Arc<dyn AnyNode>;

    /// Returns the node's first child in the tree
    fn first_child(&self) -> Option<Arc<dyn AnyNode>>;

    /// Returns the node's last child in the tree
    fn last_child(&self) -> Option<Arc<dyn AnyNode>>;

    /// Appends a node as a child
    fn append_child(&self, other: Arc<dyn AnyNode>);

    /// Returns a live NodeList representing the children of the node
    fn child_nodes(&self) -> Arc<NodeList>;
}
impl_downcast!(sync AnyNode);

macro_rules! impl_nodes {
    ($((
        $ty: ty,
        storage: $storage: ty,
        blurb: $blurb: literal,
        link: $link: literal,
        impl { $( $rest:tt )* }
        $(, $postlude: literal)?
    ))*) => {
        $(
            paste! {
                #[doc =
                    "The ["
                    $blurb
                    "](https://developer.mozilla.org/en-US/docs/Web/API/"
                    $link
                    ") node type"
                    $(" " $postlude)?
                ]
                pub struct $ty {
                    /// Reference to the sandbox to which this node belongs
                    pub context: Weak<Sandbox>,

                    /// Node behavior (fields/methods associated with the DOM class called Node)
                    pub(crate) node_behavior: Arc<NodeBehavior>,

                    pub(crate) storage: $storage,
                }
            }

            paste! {
                impl $ty {
                    pub(crate) fn new(context: Weak<Sandbox>, storage: $storage) -> Arc<$ty> {
                        let construction: Arc<$ty> = Arc::new_cyclic(|construction_weak| -> $ty {
                            $ty {
                                context,
                                node_behavior: Arc::new(NodeBehavior::new(construction_weak.clone())),
                                storage,
                            }
                        });

                        construction
                    }

                    $($rest)*
                }

                impl AnyNode for $ty {
                    fn get_context(&self) -> Weak<Sandbox> {
                        self.context.clone()
                    }

                    fn clone_node(&self) -> Arc<dyn AnyNode> {
                        let mut construction = $ty::new(self.get_context(), Default::default());

                        let mut cons = Arc::get_mut(&mut construction).expect("Could not construct node");
                        (*cons).storage = self.storage.clone();

                        construction
                    }

                    fn first_child(&self) -> Option<Arc<dyn AnyNode>> {
                        self.node_behavior.first_child()
                    }

                    fn last_child(&self) -> Option<Arc<dyn AnyNode>> {
                        self.node_behavior.last_child()
                    }

                    fn append_child(&self, other: Arc<dyn AnyNode>) {
                        self.node_behavior.append_child(other)
                    }

                    fn child_nodes(&self) -> Arc<NodeList> {
                        self.node_behavior.child_nodes()
                    }
                }

                impl PrivateAnyNode for $ty {
                    fn get_node_behavior(&self) -> Arc<NodeBehavior> {
                        self.node_behavior.clone()
                    }
                }
            }
        )*
    }
}

#[derive(Default, Clone)]
pub(crate) struct DocumentStorage {
    /// Pointer back up to the window
    pub(crate) default_view: Weak<Window>,
}

#[derive(Default, Clone)]
pub(crate) struct TextNodeStorage {
    /// Text in the text node
    pub(crate) text: String,
}

impl_nodes! {
    (
        AttrNode,
        storage: (),
        blurb: "attr (attribute)",
        link: "Attr",
        impl {}
    )
    (
        TextNode,
        storage: TextNodeStorage,
        blurb: "text",
        link: "Text",
        impl {}
    )
    (
        Document,
        storage: DocumentStorage,
        blurb: "document",
        link: "Document",
        impl {
            /// Creates a text node.
            pub fn create_text_node(&self, text: String) -> Arc<TextNode> {
                TextNode::new(self.get_context(), TextNodeStorage { text })
            }
        }
    )
}
