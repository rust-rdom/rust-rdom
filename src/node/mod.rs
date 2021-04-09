//! Core representation of a DOM node. See `nice` module for distinction from
//! nice representation.

use downcast_rs::DowncastSync;
use paste::paste;

use crate::internal_prelude::*;

crate::use_behaviors!(node, sandbox_member);
use crate::window::Window;

use std::sync::{Arc, Weak};

pub mod element;

// I have to abandon this private interface for now - maksimil
// pub(crate) mod private;

/// An input event
pub struct InputEvent {}

/// A base trait for all common node types
pub trait AnyNode: DowncastSync + SandboxMemberBehavior + NodeBehavior {
    /// Clones node according to Node.cloneNode()
    fn clone_node(&self) -> Arc<dyn AnyNode>;

    /// Returns the node type, as defuned in https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeType
    fn get_node_type(&self) -> isize;
}
impl_downcast!(sync AnyNode);

macro_rules! impl_nodes {
    ($((
        $ty: ty,
        storage: $storage: ty,
        blurb: $blurb: literal,
        link: $link: literal,
        node_type: $node_type: expr,
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
                    /// implementation for SandboxMemberBehavior
                    pub member_storage: SandboxMemberBehaviorStorage,

                    /// implementation for NodeBehavior
                    pub(crate) node_storage: NodeBehaviorStorage,

                    pub(crate) storage: $storage,

                    node_type: isize,
                }
            }

            paste! {
                impl $ty {
                    pub(crate) fn new(context: Weak<Sandbox>, storage: $storage) -> Arc<$ty> {
                        let construction: Arc<$ty> = Arc::new_cyclic(|construction_weak| -> $ty {
                            $ty {
                                storage,
                                node_storage: NodeBehaviorStorage::new(construction_weak.clone()),
                                member_storage: SandboxMemberBehaviorStorage::new(context),
                                node_type: $node_type,
                            }
                        });

                        construction
                    }

                    $($rest)*
                }

                impl_sandbox_member!($ty, member_storage);
                impl_node!($ty, node_storage);

                impl AnyNode for $ty {
                    fn clone_node(&self) -> Arc<dyn AnyNode> {
                        let mut construction = $ty::new(self.get_context(), Default::default());

                        let mut cons = Arc::get_mut(&mut construction).expect("Could not construct node");
                        (*cons).storage = self.storage.clone();

                        construction
                    }

                    fn get_node_type(&self) -> isize {
                        self.node_type
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
        ElementNode,
        storage: (),
        blurb: "Element",
        link: "Element",
        node_type: 1,
        impl {}
    )
    (
        AttrNode,
        storage: (),
        blurb: "attr (attribute)",
        link: "Attr",
        node_type: 2,
        impl {}
    )
    (
        TextNode,
        storage: TextNodeStorage,
        blurb: "text",
        link: "Text",
        node_type: 3,
        impl {}
    )
    (
        CDATASectionNode,
        storage: (),
        blurb: "CDATASection",
        link: "CDATASection",
        node_type: 4,
        impl {}
    )
    (
        ProcessingInstructionNode,
        storage: () /* or ProcessingInstructiNodeStorage */,
        blurb: "ProcessingInstruction",
        link: "ProcessingInstruction",
        node_type: 7,
        impl {}
    )
    (
        CommentNode,
        storage: () /* or CommentNodeStorage */,
        blurb: "Comment",
        link: "Comment",
        node_type: 8,
        impl {}
    )
    (
        Document,
        storage: DocumentStorage,
        blurb: "document",
        link: "Document",
        node_type: 9,
        impl {
            /// Creates a text node.
            pub fn create_text_node(&self, text: String) -> Arc<TextNode> {
                TextNode::new(self.get_context(), TextNodeStorage { text })
            }
        }
    )
    (
        DocumentTypeNode,
        storage: () /* or DocumentTypeNodeStorage */,
        blurb: "DocumentType",
        link: "DocumentType",
        node_type: 10,
        impl {}
    )
    (
        DocumentFragmentNode,
        storage: () /* or DocumentFragmentNodeStorage */,
        blurb: "DocumentFragment",
        link: "DocumentFragment",
        node_type: 11,
        impl {}
    )
}
