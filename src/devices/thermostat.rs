//! # Thermostat Device
//!
//! This module defines the `Thermostat` struct, a concrete implementation of the
//! [`Device`] trait for smart thermostats in the PulseHome system.
//!
//! The `Thermostat` can set a temperature and generate an [`Event`] for each change.

use crate::models::{
    device::Device,
    event::{Event, EventType},
};

/// Represents a smart thermostat device.
#[derive(Debug)]
pub struct Thermostat {
    name: String,
    temperature: i32, // current temperature in Celsius
}

impl Thermostat {
    /// Creates a new `Thermostat` with the given name and initial temperature.
    ///
    /// # Example
    /// ```
    /// use pulsehome::devices::thermostat::Thermostat;
    ///
    /// let thermo = Thermostat::new("Bedroom Thermostat", 22);
    /// assert_eq!(thermo.get_name(), "Bedroom Thermostat");
    /// assert_eq!(thermo.get_state(), "22°C");
    /// ```
    pub fn new(name: impl Into<String>, initial_temp: i32) -> Self {
        Self {
            name: name.into(),
            temperature: initial_temp,
        }
    }
}

impl Device for Thermostat {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_type(&self) -> &str {
        "Thermostat"
    }

    fn execute_command(&mut self, command: EventType) -> Result<Event, Box<dyn std::error::Error>> {
        match command {
            EventType::SetTemp => {
                self.temperature += 1;
            }
            _ => return Err("Thermostat only supports SetTemp commands".into()),
        }

        Ok(Event::new(
            self.name.clone(),
            self.get_type().to_string(),
            command,
            Some(self.get_state()),
        ))
    }

    fn get_state(&self) -> String {
        format!("{}°C", self.temperature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::event::EventType;

    #[test]
    fn thermostat_initial_temperature() {
        let thermo = Thermostat::new("Bedroom Thermostat", 22);
        assert_eq!(thermo.get_name(), "Bedroom Thermostat");
        assert_eq!(thermo.get_type(), "Thermostat");
        assert_eq!(thermo.get_state(), "22°C");
    }

    #[test]
    fn thermostat_set_temperature() {
        let mut thermo = Thermostat::new("Living Room Thermostat", 20);
        let event = thermo.execute_command(EventType::SetTemp).unwrap();
        assert_eq!(thermo.get_state(), "21°C");
        assert_eq!(event.device_name, "Living Room Thermostat");
        assert_eq!(event.event_type, EventType::SetTemp);
        assert_eq!(event.payload.unwrap(), "21°C");
    }

    #[test]
    fn thermostat_invalid_command() {
        let mut thermo = Thermostat::new("Test Thermo", 20);
        let result = thermo.execute_command(EventType::TurnOn);
        assert!(result.is_err());
    }
}
