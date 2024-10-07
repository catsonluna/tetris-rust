use std::sync::Arc;

use event_listener_primitives::{Bag, BagOnce, HandlerId};

#[derive(Default)]
pub struct Handlers {
    pub action: Bag<Arc<dyn Fn(String) + Send + Sync + 'static>>,
    pub closed: BagOnce<Box<dyn FnOnce(String) + Send + 'static>>,
}

pub struct StringEvent {
    pub handlers: Handlers,
}

impl Drop for StringEvent {
    fn drop(&mut self) {
        self.handlers.closed.call(Box::new(|_| {}));
    }
}

impl StringEvent {
    pub fn new() -> Self {
        let handlers = Handlers::default();

        Self { handlers }
    }

    pub fn call(&self, arg: String) {
        self.handlers.action.call(|callback| {
            callback(arg.clone());
        });
    }

    pub fn on_event<F: Fn(String) + Send + Sync + 'static>(&self, callback: F) -> HandlerId {
        self.handlers.action.add(Arc::new(callback))
    }

    pub fn on_closed<F: FnOnce(String) + Send + 'static>(&self, callback: F) -> HandlerId {
        self.handlers.closed.add(Box::new(callback))
    }
}
