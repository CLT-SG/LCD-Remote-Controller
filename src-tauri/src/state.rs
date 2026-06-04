//! Shared application state. Wraps the config + serial manager + the most
//! recent device snapshot in a tokio `Mutex` so the axum handlers can serve
//! requests concurrently without blocking the runtime.

use std::sync::Arc;

use serde::Serialize;
use tokio::sync::Mutex;

use crate::config::AppConfig;
use crate::protocol::InputSource;
use crate::serial_manager::{SerialLogEntry, SerialManager};

#[derive(Debug, Clone, Serialize, Default)]
pub struct DeviceSnapshot {
    pub power: Option<bool>,
    pub brightness: Option<u8>,
    pub contrast: Option<u8>,
    pub volume: Option<u8>,
    pub muted: Option<bool>,
    pub input: Option<InputSource>,
    pub connected: bool,
    pub last_error: Option<String>,
}

pub struct Inner {
    pub config: AppConfig,
    pub serial: SerialManager,
    pub snapshot: DeviceSnapshot,
}

pub type AppState = Arc<Mutex<Inner>>;

impl Inner {
    pub fn new(config: AppConfig) -> Self {
        let serial = SerialManager::new(config.com.clone());
        Self {
            config,
            serial,
            snapshot: DeviceSnapshot::default(),
        }
    }

    /// Refresh the cached snapshot by polling all getters. Errors are stored
    /// on the snapshot rather than propagated so the UI can keep rendering.
    pub fn refresh(&mut self) {
        if self.config.com.port.trim().is_empty() {
            self.snapshot.connected = false;
            self.snapshot.last_error = Some("No COM port configured.".into());
            return;
        }
        match self.poll() {
            Ok(()) => {
                self.snapshot.connected = self.serial.is_connected();
                self.snapshot.last_error = None;
            }
            Err(e) => {
                self.snapshot.connected = false;
                self.snapshot.last_error = Some(format!("{e:#}"));
            }
        }
    }

    fn poll(&mut self) -> anyhow::Result<()> {
        if let Some(v) = self.serial.get_power()? {
            self.snapshot.power = Some(v);
        }
        if let Some(v) = self.serial.get_brightness()? {
            if v <= 100 {
                self.snapshot.brightness = Some(v);
            }
        }
        if let Some(v) = self.serial.get_contrast()? {
            self.snapshot.contrast = Some(v);
        }
        if let Some(v) = self.serial.get_volume()? {
            self.snapshot.volume = Some(v);
        }
        if let Some(v) = self.serial.get_mute()? {
            self.snapshot.muted = Some(v);
        }
        if let Some(v) = self.serial.get_input()? {
            if matches!(v, InputSource::Vga | InputSource::Hdmi | InputSource::Dp) {
                self.snapshot.input = Some(v);
            }
        }
        Ok(())
    }

    pub fn logs(&self) -> Vec<SerialLogEntry> {
        self.serial.logs()
    }

    pub fn clear_logs(&mut self) {
        self.serial.clear_logs();
    }
}
