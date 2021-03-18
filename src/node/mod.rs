use downcast_rs::DowncastSync;

use crate::node::raw::AnyNode;

pub use wrapped::*;

mod raw;
mod wrapped;

#[derive(Debug)]
pub struct Node {}
impl AnyNode for Node {}