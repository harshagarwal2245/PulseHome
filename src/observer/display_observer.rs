//! # Display Observer
//!
//! Prints device events to the console.

use crate::models::event::Event;
use crate::observer::Observer;

/// Observer that displays events to the console.
pub struct DisplayObserver;

impl DisplayObserver {
    /// Creates a new DisplayObserver.
    pub fn new() -> Self {
        Self
    }
}

impl Observer for DisplayObserver {
    fn on_event(&mut self, event: &Event) {
        println!(
            "[DisplayObserver] Device '{}' ({}) state: {}",
            event.device_name,
            event.device_type,
            event.payload.as_deref().unwrap_or("unknown")
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::event::Event;
    use crate::models::event::EventType;

    #[test]
    fn display_observer_receives_event() {
        let mut observer = DisplayObserver::new();
        let event = Event::new("Light", "Light", EventType::TurnOn, Some("on".to_string()));
        observer.on_event(&event);
    }
}
