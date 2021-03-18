use downcast_rs::DowncastSync;

use super::AnyNode;

pub mod body;
pub mod document;

pub trait AnyElement: DowncastSync + AnyNode {}
impl_downcast!(sync AnyElement);

#[derive(Debug)]
pub struct Element {}
impl AnyElement for Element {}
impl AnyNode for Element {}