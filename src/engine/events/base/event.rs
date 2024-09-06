use std::sync::Arc;

use event_listener_primitives::{Bag, BagOnce, HandlerId};

#[derive(Default)]
pub struct Handlers {
    pub action: Bag<Arc<dyn Fn() + Send + Sync + 'static>>,
    pub closed: BagOnce<Box<dyn FnOnce() + Send + 'static>>,
}

pub struct BaseEvent {
    pub handlers: Handlers,
}

impl Drop for BaseEvent {
    fn drop(&mut self) {
        self.handlers.closed.call_simple();
    }
}

impl BaseEvent {
    pub fn new() -> Self {
        let handlers = Handlers::default();

        Self { handlers }
    }

    pub fn call(&self) {
        self.handlers.action.call(|callback| {
            callback();
        });
    }

    pub fn on_event<F: Fn() + Send + Sync + 'static>(&self, callback: F) -> HandlerId {
        self.handlers.action.add(Arc::new(callback))
    }

    pub fn on_closed<F: FnOnce() + Send + 'static>(&self, callback: F) -> HandlerId {
        self.handlers.closed.add(Box::new(callback))
    }
}
