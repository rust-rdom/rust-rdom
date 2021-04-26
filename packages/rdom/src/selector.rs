use crate::error::DomError;
use crate::node::{concrete::ElementNodeArc, AnyNodeArc};
use std::convert::{TryFrom, TryInto};

pub struct Selector(String);

impl TryFrom<String> for Selector {
    type Error = DomError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // validate string (only allow [A-Z] and [0-9])
        let value = value.to_uppercase();
        let valid = value.as_bytes().iter().all(|&v| {
            (v >= ('A' as u8) && v <= ('Z' as u8)) || (v >= ('0' as u8) && v <= ('9' as u8))
        });

        if valid {
            Ok(Selector(value))
        } else {
            Err(DomError::InvalidQuerySelector)
        }
    }
}

impl TryFrom<&str> for Selector {
    type Error = DomError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Selector::try_from(value.to_string())
    }
}

impl Selector {
    pub fn matches_selected_node(&self, node: &AnyNodeArc) -> Option<ElementNodeArc> {
        match TryInto<ElementNodeArc>::try_into(node.clone()) {
            Ok(element) => {
                if self.is_selected_element(element.clone()) {
                    Some(element)
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }

    pub fn is_selected_element(&self, element: ElementNodeArc) -> bool {
        element.contents.tag_name() == self.0
    }
}
