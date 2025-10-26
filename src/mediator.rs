//! # HomeHub (Mediator)
//!
//! The `HomeHub` mediates between devices and observers in the PulseHome system.
//! It allows the CLI or other clients to execute commands on devices, and automatically
//! notifies observers about device events.

use crate::models::{device::Device, event::Event};
use crate::observer::Observer;

/// The HomeHub struct acts as a Mediator for devices and observers.
pub struct HomeHub {
    devices: Vec<Box<dyn Device>>,
    observers: Vec<Box<dyn Observer>>,
}

impl HomeHub {
    /// Creates a new empty HomeHub.
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            observers: Vec::new(),
        }
    }

    /// Registers a new device with the hub.
    pub fn register_device(&mut self, device: Box<dyn Device>) {
        self.devices.push(device);
    }

    /// Registers a new observer with the hub.
    pub fn register_observer(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    /// Executes a command on a device by name.
    ///
    /// Notifies all observers of the resulting event.
    pub fn execute_device_command(
        &mut self,
        device_name: &str,
        command: crate::models::event::EventType,
    ) -> Result<Event, Box<dyn std::error::Error>> {
        let device = self
            .devices
            .iter_mut()
            .find(|d| d.get_name() == device_name)
            .ok_or_else(|| format!("Device '{}' not found", device_name))?;

        let event = device.execute_command(command)?;

        // Notify observers
        for obs in &mut self.observers {
            obs.on_event(&event);
        }

        Ok(event)
    }

    /// Returns a list of registered device names.
    pub fn list_devices(&self) -> Vec<String> {
        self.devices
            .iter()
            .map(|d| d.get_name().to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::devices::light::Light;
    use crate::models::event::EventType;
    use crate::observer::Observer;

    /// Dummy observer for testing
    struct DummyObserver {
        pub events: Vec<String>,
    }

    impl DummyObserver {
        fn new() -> Self {
            Self { events: Vec::new() }
        }
    }

    impl Observer for DummyObserver {
        fn on_event(&mut self, event: &Event) {
            self.events.push(format!(
                "{}:{}",
                event.device_name,
                event.payload.clone().unwrap()
            ));
        }
    }

    #[test]
    fn homehub_execute_device_command() {
        let mut hub = HomeHub::new();
        let mut light = Light::new("Living Room Light");
        hub.register_device(Box::new(light));
        let mut observer = DummyObserver::new();
        hub.register_observer(Box::new(observer));

        let event = hub
            .execute_device_command("Living Room Light", EventType::TurnOn)
            .unwrap();
        assert_eq!(event.device_name, "Living Room Light");
        assert_eq!(event.payload.unwrap(), "on");
    }

    #[test]
    fn homehub_device_not_found() {
        let mut hub = HomeHub::new();
        let result = hub.execute_device_command("NonExistent", EventType::TurnOn);
        assert!(result.is_err());
    }
}
