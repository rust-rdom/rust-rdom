use downcast_rs::DowncastSync;

#[derive(Debug)]
pub(crate) struct EventBehaviorStorage {
    r#type: String,
    bubbles: bool,
    cancelable: bool,
}

pub(crate) trait Event: DowncastSync {
    fn get_storage(&self) -> &EventBehaviorStorage;
    fn get_cancelable(&self) -> bool {
        self.get_storage().cancelable
    }
    fn get_bubbles(&self) -> bool {
        self.get_storage().bubbles
    }
    fn get_type(&self) -> String {
        self.get_storage().r#type.clone()
    }
}
impl_downcast!(sync Event);

#[derive(Debug)]
struct MouseEvent {
    storage: EventBehaviorStorage
}
impl Event for MouseEvent {
    fn get_storage(&self) -> &EventBehaviorStorage {
        &self.storage
    }
}

#[derive(Clone)]
enum EventTarget {
    Node(AnyNodeWeak),
    XMLHttpRequest,
}

fn main() {
      let base: Box<dyn Event> = Box::new(MouseEvent {
        storage: EventBehaviorStorage {
          r#type: "onclick".into(),
          bubbles: true,
          cancelable: true
        }
      });

      if let Some(foo) = base.downcast_ref::<MouseEvent>() {
        assert_eq!(foo.storage.r#type, "onclick");
      }
}
