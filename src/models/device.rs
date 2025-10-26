//! # Device Module
//!
//! This module defines the [`Device`] trait, which represents any smart device
//! in the PulseHome system. Concrete devices (e.g., `Light`, `Thermostat`) implement
//! this trait.  
//!
//! The trait provides a common interface for executing commands and retrieving state,
//! allowing the **HomeHub mediator** to interact uniformly with all devices.

use crate::models::event::{Event, EventType};

/// Represents a generic smart device.
///
/// Implementors must provide methods to execute commands
/// and retrieve the current state of the device.
pub trait Device {
    /// Returns the unique name of the device.
    fn get_name(&self) -> &str;

    /// Returns a human-readable type of the device (e.g., `"Light"`).
    fn get_type(&self) -> &str;

    /// Executes a command on the device.
    ///
    /// Returns an [`Event`] representing the result or an error message.
    fn execute_command(&mut self, command: EventType) -> Result<Event, Box<dyn std::error::Error>>;

    /// Returns the current state of the device as a string.
    fn get_state(&self) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::event::EventType;

    /// Dummy device for testing the Device trait
    struct DummyDevice {
        name: String,
        device_type: String,
        state: String,
    }

    impl DummyDevice {
        fn new(name: &str, device_type: &str) -> Self {
            Self {
                name: name.to_string(),
                device_type: device_type.to_string(),
                state: "off".to_string(),
            }
        }
    }

    impl Device for DummyDevice {
        fn get_name(&self) -> &str {
            &self.name
        }

        fn get_type(&self) -> &str {
            &self.device_type
        }

        fn execute_command(
            &mut self,
            command: EventType,
        ) -> Result<Event, Box<dyn std::error::Error>> {
            match command {
                EventType::TurnOn => self.state = "on".to_string(),
                EventType::TurnOff => self.state = "off".to_string(),
                _ => return Err("Unsupported command for DummyDevice".into()),
            }

            Ok(Event::new(
                self.name.clone(),
                self.device_type.clone(),
                command,
                Some(self.state.clone()),
            ))
        }

        fn get_state(&self) -> String {
            self.state.clone()
        }
    }

    #[test]
    fn dummy_device_turn_on() {
        let mut device = DummyDevice::new("TestLight", "Light");
        let event = device.execute_command(EventType::TurnOn).unwrap();
        assert_eq!(device.get_state(), "on");
        assert_eq!(event.device_name, "TestLight");
        assert_eq!(event.event_type, EventType::TurnOn);
        assert_eq!(event.payload.unwrap(), "on");
    }

    #[test]
    fn dummy_device_invalid_command() {
        let mut device = DummyDevice::new("TestDevice", "Generic");
        let result = device.execute_command(EventType::SetTemp);
        assert!(result.is_err());
    }
}
