use std::{collections::HashMap};

use variadic_closure::Function;
use crate::{node::AnyNodeWeak};

struct MouseEventData {

}

enum EventKind {
    OnClick,
    // TODO someday: CustomEvent
}

struct EventListenerOptions {}

struct EventListenerEntry<'a> {
    // data: EventKind,
    listener: &'a Function,
    // target: EventTarget,
    options: EventListenerOptions
}

pub struct EventTargetBehaviorStorage<'a> {
    // target: EventTarget,
    listeners: &'a mut HashMap<String, EventListenerEntry<'a>>
}

impl<'a> EventTargetBehaviorStorage<'a> {
    fn add_event_listener_with_callback(
        &mut self,
        kind: String,
        listener: &'a Function
    ) -> Result<(), ()> {
        self.listeners.insert(kind, EventListenerEntry {
            listener,
            // data: EventKind::OnClick,
            // target: self.target.clone(),
            options: EventListenerOptions{}
        });
        Ok(())
    }
}
