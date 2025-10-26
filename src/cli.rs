//! # CLI Module
//!
//! Provides a command-line interface for the PulseHome Smart Home system.
//! Allows the user to interact with the `HomeHub`, execute commands on devices,
//! and receive feedback from observers.
//!
//! ## Example
//! ```no_run
//! use pulsehome::cli::CLI;
//! use pulsehome::mediator::HomeHub;
//!
//! let hub = HomeHub::new();
//! let mut cli = CLI::new(hub);
//! cli.start();
//! ```

use crate::devices::{door_lock::*, light::*, thermostat::*};
use crate::mediator::HomeHub;
use crate::models::event::EventType;
use std::io::{self, Write};

/// Represents the command-line interface for interacting with the smart home system.
pub struct CLI {
    hub: HomeHub,
}

impl CLI {
    /// Creates a new CLI instance with the given `HomeHub`.
    ///
    /// # Arguments
    /// * `hub` - The `HomeHub` mediator used to manage devices and observers.
    ///
    /// # Example
    /// ```
    /// use pulsehome::cli::CLI;
    /// use pulsehome::mediator::HomeHub;
    ///
    /// let hub = HomeHub::new();
    /// let cli = CLI::new(hub);
    /// ```
    pub fn new(hub: HomeHub) -> Self {
        Self { hub }
    }

    /// Starts the interactive CLI loop.
    ///
    /// The user can type commands to control devices or type `exit` to quit.
    pub fn start(&mut self) {
        println!("Welcome to PulseHome Smart Home CLI!");
        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                eprintln!("Failed to read input");
                continue;
            }

            if input.eq_ignore_ascii_case("help") {
                Self::print_help();
                continue;
            }

            let input = input.trim();
            if input.eq_ignore_ascii_case("exit") {
                println!("Exiting CLI. Goodbye!");
                break;
            }

            self.parse_command(input);
        }
    }

    fn print_help() {
        println!("Available commands:");
        println!("  add <device_type> <device_name> [initial_value] - Add a new device");
        println!("     device_type: light | thermostat | doorlock");
        println!("  turn_on <device_name>       - Turn on a light");
        println!("  turn_off <device_name>      - Turn off a light");
        println!("  lock <device_name>          - Lock a door");
        println!("  unlock <device_name>        - Unlock a door");
        println!("  set_temp <device_name> <value> - Set thermostat temperature");
        println!("  list                        - List all registered devices");
        println!("  help                        - Show this help message");
        println!("  exit                        - Exit the CLI");
    }

    /// Parses a user command and executes it via the `HomeHub`.
    ///
    /// Supported commands:
    /// - `add <device_type> <device_name> [initial_value]`
    /// - `turn_on <device_name>`
    /// - `turn_off <device_name>`
    /// - `lock <device_name>`
    /// - `unlock <device_name>`
    /// - `set_temp <device_name> <value>`
    /// - `list`
    ///
    /// # Examples
    /// ```no_run
    /// cli.parse_command("add light Living Room Light");
    /// cli.parse_command("turn_on Living Room Light");
    /// cli.parse_command("set_temp Bedroom Thermostat 24");
    /// cli.parse_command("list");
    /// ```
    fn parse_command(&mut self, command: &str) {
        let command = command.trim();
        if command.is_empty() {
            self.display_message("Empty command");
            return;
        }

        let mut parts = command.split_whitespace();
        let action = match parts.next() {
            Some(a) => a.to_lowercase(),
            None => return,
        };
        let rest: Vec<&str> = parts.collect();

        match action.as_str() {
            "add" => {
                if rest.len() < 2 {
                    eprintln!("Usage: add <device_type> <device_name> [initial_value]");
                    return;
                }
                let device_type = rest[0];
                let device_name = rest[1..].join(" ");
                match device_type.to_lowercase().as_str() {
                    "light" => self.hub.register_device(Box::new(Light::new(&device_name))),
                    "doorlock" => self
                        .hub
                        .register_device(Box::new(DoorLock::new(&device_name))),
                    "thermostat" => {
                        let temp = rest
                            .get(2)
                            .and_then(|v| v.parse::<i32>().ok())
                            .unwrap_or(22);
                        self.hub
                            .register_device(Box::new(Thermostat::new(&device_name, temp)));
                    }
                    _ => {
                        eprintln!("Unknown device type '{}'", device_type);
                        return;
                    }
                }
                self.display_message(&format!(
                    "Device '{}' of type '{}' added.",
                    device_name, device_type
                ));
            }
            "turn_on" | "turn_off" | "lock" | "unlock" => {
                if rest.is_empty() {
                    eprintln!("Usage: {} <device_name>", action);
                    return;
                }
                let device_name = rest.join(" ");
                let event_type = match action.as_str() {
                    "turn_on" => EventType::TurnOn,
                    "turn_off" => EventType::TurnOff,
                    "lock" => EventType::Lock,
                    "unlock" => EventType::Unlock,
                    _ => unreachable!(),
                };
                match self.hub.execute_device_command(&device_name, event_type) {
                    Ok(event) => self.display_message(&format!(
                        "Executed command: {} on '{}'. New state: {}",
                        action,
                        device_name,
                        event.payload.unwrap_or("unknown".to_string())
                    )),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            "set_temp" => {
                if rest.len() < 2 {
                    eprintln!("Usage: set_temp <device_name> <temperature>");
                    return;
                }
                let temp_str = rest.last().unwrap();
                let device_name = rest[..rest.len() - 1].join(" ");
                match temp_str.parse::<i32>() {
                    Ok(_temp) => {
                        match self
                            .hub
                            .execute_device_command(&device_name, EventType::SetTemp)
                        {
                            Ok(event) => self.display_message(&format!(
                                "Set temperature for '{}' to {}",
                                device_name,
                                event.payload.unwrap_or("unknown".to_string())
                            )),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                    Err(_) => eprintln!("Invalid temperature '{}'", temp_str),
                }
            }
            "list" => {
                let devices = self.hub.list_devices();
                if devices.is_empty() {
                    self.display_message("No devices registered.");
                } else {
                    self.display_message(&format!("Registered devices: {:?}", devices));
                }
            }
            _ => eprintln!("Unknown command '{}'", action),
        }
    }

    /// Displays a message to the user.
    ///
    /// # Arguments
    /// * `msg` - The message string to display.
    fn display_message(&self, msg: &str) {
        println!("{}", msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::devices::{door_lock::DoorLock, light::Light, thermostat::Thermostat};
    use crate::models::event::EventType;

    #[test]
    fn test_add_command() {
        let hub = HomeHub::new();
        let mut cli = CLI::new(hub);

        // Add devices
        cli.parse_command("add light Living Room Light");
        cli.parse_command("add thermostat Bedroom Thermostat 24");
        cli.parse_command("add doorlock Front Door");

        let devices = cli.hub.list_devices();
        assert!(devices.contains(&"Living Room Light".to_string()));
        assert!(devices.contains(&"Front Door".to_string()));
    }

    #[test]
    fn test_list_command() {
        let mut hub = HomeHub::new();
        hub.register_device(Box::new(Light::new("Living Room Light")));
        let mut cli = CLI::new(hub);

        // Just call list and ensure no panic
        cli.parse_command("list");
    }

    #[test]
    fn test_turn_on_off_commands() {
        let mut hub = HomeHub::new();
        hub.register_device(Box::new(Light::new("Living Room Light")));
        let mut cli = CLI::new(hub);

        cli.parse_command("turn_on Living Room Light");
        let device = cli
            .hub
            .list_devices()
            .iter()
            .find(|d| d.as_str() == "Living Room Light")
            .unwrap();
    }

    #[test]
    fn test_invalid_command() {
        let hub = HomeHub::new();
        let mut cli = CLI::new(hub);

        // Unknown command should print error but not panic
        cli.parse_command("fly Living Room Light");
    }

    #[test]
    fn test_missing_arguments() {
        let hub = HomeHub::new();
        let mut cli = CLI::new(hub);

        cli.parse_command("add"); // missing device_type and name
        cli.parse_command("turn_on"); // missing device name
        cli.parse_command("set_temp Bedroom Thermostat"); // missing value
    }
}
