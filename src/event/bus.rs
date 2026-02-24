use tokio::sync::broadcast;
use crate::event::event::AxonEvent;

pub type EventSender = broadcast::Sender<AxonEvent>;
pub type EventReceiver = broadcast::Receiver<AxonEvent>;

pub fn create_event_bus(capacity: usize) -> (EventSender, EventReceiver) {
    broadcast::channel(capacity)
}

// === AXON_COMPAT: EventBus alias ===
/// Compatibility alias used by older code.
pub type EventBus = EventSender;

