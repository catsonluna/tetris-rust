use once_cell::sync::Lazy;

use super::base::event::BaseEvent;

pub static UPDATE_EVENT: Lazy<BaseEvent> = Lazy::new(|| BaseEvent::new());
pub static TICK_EVENT:  Lazy<BaseEvent> = Lazy::new(|| BaseEvent::new());
pub static RENDER_EVENT: Lazy<BaseEvent> = Lazy::new(|| BaseEvent::new());