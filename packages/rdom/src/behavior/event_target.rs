use std::collections::HashMap;

use variadic_closure::Function;
use crate::{node::AnyNodeWeak};

struct MouseEventData {

}

enum EventKind {
    OnClick,
    // TODO someday: CustomEvent
}

struct EventListenerOptions {}

struct EventListenerEntry {
    data: EventKind,
    target: EventTarget,
    options: EventListenerOptions
}

pub struct EventTargetBehaviorStorage {
    target: EventTarget,
    listeners: HashMap<EventKind, EventListenerEntry>
}

#[derive(Clone)]
enum EventTarget {
    Node(AnyNodeWeak),
    XMLHttpRequest,
}

impl EventTargetBehaviorStorage {
    fn add_event_listener_with_callback(
        &mut self,
        kind: &str,
        listener: &Function
    ) -> Result<(), ()> {
        self.listeners.insert(EventKind::OnClick, EventListenerEntry {
            data: EventKind::OnClick,
            target: self.target.clone(),
            options: EventListenerOptions{}
        });
        Ok(())
    }
}
