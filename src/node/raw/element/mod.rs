use downcast_rs::DowncastSync;

use super::AnyRawNode;

pub mod body;
pub mod document;

/// A base trait for all raw element types
pub trait AnyRawElement: DowncastSync + AnyRawNode {}
impl_downcast!(sync AnyRawElement);

#[derive(Debug)]
pub struct Element {}
impl AnyRawElement for Element {}
impl AnyRawNode for Element {}