use downcast_rs::DowncastSync;

use crate::node::raw::AnyRawNode;

pub use wrapped::*;

mod raw;
mod wrapped;

#[derive(Debug)]
pub struct Node {}
impl AnyRawNode for Node {}