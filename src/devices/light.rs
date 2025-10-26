//! # Light Device
//!
//! This module defines the `Light` struct, a concrete implementation of the
//! [`Device`] trait for smart lights in the PulseHome system.
//!
//! The `Light` device can be turned on or off and generates an [`Event`]
//! whenever its state changes.

use crate::models::{
    device::Device,
    event::{Event, EventType},
};

/// Represents a smart light device.
#[derive(Debug)]
pub struct Light {
    name: String,
    state: bool, // true = on, false = off
}

impl Light {
    /// Creates a new `Light` with the given name, initially off.
    ///
    /// # Example
    /// ```
    /// use pulsehome::devices::light::Light;
    ///
    /// let light = Light::new("Living Room Light");
    /// assert_eq!(light.get_name(), "Living Room Light");
    /// assert_eq!(light.get_state(), "off");
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            state: false,
        }
    }
}

impl Device for Light {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_type(&self) -> &str {
        "Light"
    }

    fn execute_command(&mut self, command: EventType) -> Result<Event, Box<dyn std::error::Error>> {
        match command {
            EventType::TurnOn => self.state = true,
            EventType::TurnOff => self.state = false,
            _ => return Err("Light only supports TurnOn or TurnOff commands".into()),
        }

        Ok(Event::new(
            self.name.clone(),
            self.get_type().to_string(),
            command,
            Some(self.get_state()),
        ))
    }

    fn get_state(&self) -> String {
        if self.state {
            "on".to_string()
        } else {
            "off".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::event::EventType;

    #[test]
    fn light_initial_state_off() {
        let light = Light::new("Bedroom Light");
        assert_eq!(light.get_name(), "Bedroom Light");
        assert_eq!(light.get_type(), "Light");
        assert_eq!(light.get_state(), "off");
    }

    #[test]
    fn light_turn_on() {
        let mut light = Light::new("Living Room Light");
        let event = light.execute_command(EventType::TurnOn).unwrap();
        assert_eq!(light.get_state(), "on");
        assert_eq!(event.device_name, "Living Room Light");
        assert_eq!(event.event_type, EventType::TurnOn);
        assert_eq!(event.payload.unwrap(), "on");
    }

    #[test]
    fn light_turn_off() {
        let mut light = Light::new("Living Room Light");
        light.execute_command(EventType::TurnOn).unwrap(); // turn on first
        let event = light.execute_command(EventType::TurnOff).unwrap();
        assert_eq!(light.get_state(), "off");
        assert_eq!(event.event_type, EventType::TurnOff);
        assert_eq!(event.payload.unwrap(), "off");
    }

    #[test]
    fn light_invalid_command() {
        let mut light = Light::new("Test Light");
        let result = light.execute_command(EventType::SetTemp);
        assert!(result.is_err());
    }
}
