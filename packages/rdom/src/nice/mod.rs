//! This module contains a nicer, public representation of Nodes and Elements. This is
//! nice in comparison to what rdom calls the "core" representation of Nodes and
//! Elements, which is a bit more cumbersome to deal with in some cases.
//!
//! For most purposes, a nice element is what you want. Nice elements store
//! an `Arc` of the core element, which ensures that the underlying core element is retained
//! as long as you maintain that reference to it. (This is how all `Arc`s work.)
//!
//! For some DOM operations, ownership of said `Arc` (or nice element) is sufficient
//! to perform the operation. However, this `Arc` does not ensure that the whole sandbox
//! is retained, due to the possibility that the sandbox is dropped at an arbitrary time
//! while you hold this reference.
//!
//! As a result, you must be careful to not drop the sandbox until you are totally done
//! performing DOM operations, else you may find that those operations fail.
//!
//! Rdom opts for weak pointers in all but one direction (down), so if the sandbox is
//! dropped, most of the elements will be dropped with it. This design is
//! chosen to help with preventing memory leaks, but it has the side effect of causing some
//! operations (such as getting the parent node of an element) to fail at runtime.

use paste::paste;

use std::convert::TryFrom;
use std::result::Result;

crate::use_behaviors!(sandbox_member);
use crate::internal_prelude::*;

pub mod element;

pub struct Document(AnyNodeArc);
