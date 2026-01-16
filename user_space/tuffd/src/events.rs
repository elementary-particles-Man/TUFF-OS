use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::state_machine::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct TuffLogEntry {
    pub timestamp: u64,
    pub level: LogLevel,
    pub event: TuffEvent,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Audit,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "details")]
pub enum TuffEvent {
    SystemBoot { version: String },
    StateTransition { from: State, to: State, reason: String },
    KeySearch { status: String },
    KeyDetected { device: String, key_uuid: String },
    KeyRejected { device: String, reason: String },
    MountSuccess { path: String },
    MountFailure { path: String, error: String },
    IoError { context: String, error: String },
}

impl TuffLogEntry {
    pub fn new(level: LogLevel, event: TuffEvent) -> Self {
        let start = SystemTime::now();
        let timestamp = start.duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        Self {
            timestamp,
            level,
            event,
        }
    }

    pub fn log(&self) {
        // In production, this would go to a secured audit file.
        // For now, we print JSON to stdout/stderr which systemd/docker can capture.
        if let Ok(json) = serde_json::to_string(self) {
            println!("{}", json);
        }
    }
}
