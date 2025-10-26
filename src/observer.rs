//! # Observer Trait
//!
//! Defines the `Observer` trait for the PulseHome system.
//! Observers get notified by the `HomeHub` whenever a device emits an event.

use crate::models::event::Event;

pub mod display_observer;
pub mod logger_observer;

/// Trait representing an observer that reacts to device events.
pub trait Observer {
    /// Called by HomeHub whenever a device generates an event.
    fn on_event(&mut self, event: &Event);
}
