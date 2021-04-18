use super::graph_storage::NodeGraphStorage;
use super::{AnyStorage, NodeCommon};
use crate::internal_prelude::*;

crate::use_behaviors!(sandbox_member);

// A strongly-typed handle to a node with a weak reference.
// T may be the underlying storage
// type of any node.
