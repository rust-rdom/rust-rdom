//! This module contains various representations of Nodes and Elements. There are
//! two variants:
//!   - raw
//!   - wrapped
//!
//! For most purposes, a wrapped element is what you want. Wrapped elements store
//! an `Arc` of the raw element, which ensures that the underlying raw element is retained
//! as long as you maintain that reference to it. (This is how all `Arc`s work.)
//!
//! For some DOM operations, ownership of said `Arc` (or wrapped element) is sufficient
//! to perform the operation. However, this `Arc` does not ensure that the whole sandbox
//! is retained, due to the possibility that the sandbox is dropped at an arbitrary time
//! while you hold this reference.
//! 
//! Rdom opts for weak pointers in all but one direction (down), so if the sandbox is
//! dropped, most of the elements will be drop with it. This design is
//! chosen to help with preventing memory leaks, but it has the side effect of causing some
//! operations (such as getting the parent node of an element) to fail at runtime.
//! 
//! As a result, you must be careful to not drop the sandbox until you are totally done
//! performing DOM operations, else you may find that those operations fail.

use downcast_rs::DowncastSync;

use crate::node::raw::AnyRawNode;

pub use wrapped::*;

pub mod raw;
pub mod wrapped;

#[derive(Debug)]
pub struct Node {}
impl AnyRawNode for Node {}