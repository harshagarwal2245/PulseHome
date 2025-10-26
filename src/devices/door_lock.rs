//! # Door Lock Device
//!
//! This module defines the `DoorLock` struct, a concrete implementation of the
//! [`Device`] trait for smart locks in the PulseHome system.
//!
//! The `DoorLock` can be locked or unlocked, and generates an [`Event`] whenever its state changes.

use crate::models::{
    device::Device,
    event::{Event, EventType},
};

/// Represents a smart door lock device.
#[derive(Debug)]
pub struct DoorLock {
    name: String,
    locked: bool, // true = locked, false = unlocked
}

impl DoorLock {
    /// Creates a new `DoorLock` with the given name, initially unlocked.
    ///
    /// # Example
    /// ```
    /// use pulsehome::devices::door_lock::DoorLock;
    ///
    /// let lock = DoorLock::new("Front Door");
    /// assert_eq!(lock.get_name(), "Front Door");
    /// assert_eq!(lock.get_state(), "unlocked");
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            locked: false,
        }
    }
}

impl Device for DoorLock {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_type(&self) -> &str {
        "DoorLock"
    }

    fn execute_command(&mut self, command: EventType) -> Result<Event, Box<dyn std::error::Error>> {
        match command {
            EventType::Lock => self.locked = true,
            EventType::Unlock => self.locked = false,
            _ => return Err("DoorLock only supports Lock or Unlock commands".into()),
        }

        Ok(Event::new(
            self.name.clone(),
            self.get_type().to_string(),
            command,
            Some(self.get_state()),
        ))
    }

    fn get_state(&self) -> String {
        if self.locked {
            "locked".to_string()
        } else {
            "unlocked".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::event::EventType;

    #[test]
    fn door_initial_state_unlocked() {
        let lock = DoorLock::new("Front Door");
        assert_eq!(lock.get_name(), "Front Door");
        assert_eq!(lock.get_type(), "DoorLock");
        assert_eq!(lock.get_state(), "unlocked");
    }

    #[test]
    fn door_lock_unlock() {
        let mut lock = DoorLock::new("Back Door");

        let lock_event = lock.execute_command(EventType::Lock).unwrap();
        assert_eq!(lock.get_state(), "locked");
        assert_eq!(lock_event.payload.unwrap(), "locked");

        let unlock_event = lock.execute_command(EventType::Unlock).unwrap();
        assert_eq!(lock.get_state(), "unlocked");
        assert_eq!(unlock_event.payload.unwrap(), "unlocked");
    }

    #[test]
    fn door_invalid_command() {
        let mut lock = DoorLock::new("Test Door");
        let result = lock.execute_command(EventType::SetTemp);
        assert!(result.is_err());
    }
}
