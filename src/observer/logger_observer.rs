//! # Logger Observer
//!
//! Logs device events to a file.

use crate::models::event::Event;
use crate::observer::Observer;
use std::{fs::OpenOptions, io::Write};

/// Observer that logs events to a file.
pub struct LoggerObserver {
    file_path: String,
}

impl LoggerObserver {
    /// Creates a new LoggerObserver writing to the given file path.
    pub fn new(file_path: impl Into<String>) -> Self {
        Self {
            file_path: file_path.into(),
        }
    }
}

impl Observer for LoggerObserver {
    fn on_event(&mut self, event: &Event) {
        if let Err(e) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
            .and_then(|mut f| {
                writeln!(
                    f,
                    "Device '{}' ({}) state: {}",
                    event.device_name,
                    event.device_type,
                    event.payload.as_deref().unwrap_or("unknown")
                )
            })
        {
            eprintln!("[LoggerObserver] Failed to write log: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::event::{Event, EventType};
    use std::fs;

    #[test]
    fn logger_observer_writes_file() {
        let log_file = "test_log.txt";
        let mut observer = LoggerObserver::new(log_file);
        let event = Event::new("Light", "Light", EventType::TurnOn, Some("on".to_string()));
        observer.on_event(&event);

        let content = fs::read_to_string(log_file).unwrap();
        assert!(content.contains("Device 'Light' (Light) state: on"));

        // Clean up
        let _ = fs::remove_file(log_file);
    }
}
