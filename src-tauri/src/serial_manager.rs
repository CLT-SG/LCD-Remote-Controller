//! Thin wrapper around `serialport` that owns the open handle and serialises
//! request / response cycles. The BSD-E6 protocol is half-duplex: send 8 bytes,
//! read 1 reply byte. The manager keeps the port closed when there is no
//! configured port name so the rest of the application can still be inspected
//! and configured before any hardware is connected.

use std::{
    collections::VecDeque,
    io::{Read, Write},
    time::Duration,
};

use anyhow::{Context, Result};
use serde::Serialize;
use serialport::SerialPort;

use crate::config::ComConfig;
use crate::protocol::{self, cmd, InputSource, Op};

/// Single entry in the serial communication log.
#[derive(Debug, Clone, Serialize)]
pub struct SerialLogEntry {
    pub timestamp: String,
    pub direction: String,
    pub hex: String,
    pub summary: String,
}

/// Bounded ring buffer for TX / RX frames.
pub struct SerialLog {
    entries: VecDeque<SerialLogEntry>,
    max_entries: usize,
}

impl SerialLog {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: VecDeque::new(),
            max_entries,
        }
    }

    pub fn push(&mut self, direction: &str, bytes: &[u8], summary: &str) {
        let timestamp = format_timestamp();
        let hex = bytes
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(" ");
        self.entries.push_back(SerialLogEntry {
            timestamp,
            direction: direction.to_string(),
            hex: if hex.is_empty() {
                "—".to_string()
            } else {
                hex
            },
            summary: summary.to_string(),
        });
        while self.entries.len() > self.max_entries {
            self.entries.pop_front();
        }
    }

    pub fn entries(&self) -> Vec<SerialLogEntry> {
        self.entries.iter().cloned().collect()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

fn format_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let secs = now.as_secs();
    let hh = (secs / 3600) % 24;
    let mm = (secs / 60) % 60;
    let ss = secs % 60;
    format!("{:02}:{:02}:{:02}", hh, mm, ss)
}

pub struct SerialManager {
    config: ComConfig,
    port: Option<Box<dyn SerialPort>>,
    log: SerialLog,
}

impl SerialManager {
    pub fn new(config: ComConfig) -> Self {
        Self {
            config,
            port: None,
            log: SerialLog::new(200),
        }
    }

    /// Read-only view of the active serial configuration. Currently used
    /// only by tests and integration probes; kept on the public API so
    /// callers do not have to clone the config to inspect it.
    #[allow(dead_code)]
    pub fn config(&self) -> &ComConfig {
        &self.config
    }

    /// Replace the configuration. Any open handle is closed; the next command
    /// will lazily re-open with the new settings.
    pub fn update_config(&mut self, config: ComConfig) {
        if config != self.config {
            self.port = None;
        }
        self.config = config;
    }

    pub fn list_ports() -> Vec<String> {
        serialport::available_ports()
            .map(|ports| ports.into_iter().map(|p| p.port_name).collect())
            .unwrap_or_default()
    }

    /// Open the configured port, returning an error if it cannot be opened.
    pub fn open(&mut self) -> Result<()> {
        if self.port.is_some() {
            return Ok(());
        }
        if self.config.port.trim().is_empty() {
            anyhow::bail!("no COM port configured");
        }
        let stop_bits = match self.config.stop_bits {
            2 => serialport::StopBits::Two,
            _ => serialport::StopBits::One,
        };
        let data_bits = match self.config.data_bits {
            7 => serialport::DataBits::Seven,
            _ => serialport::DataBits::Eight,
        };
        let port = serialport::new(&self.config.port, self.config.baud_rate)
            .data_bits(data_bits)
            .stop_bits(stop_bits)
            .parity(self.config.parity.as_serial())
            .flow_control(self.config.flow_control.as_serial())
            .timeout(Duration::from_millis(self.config.timeout_ms))
            .open()
            .with_context(|| format!("opening {}", self.config.port))?;
        self.port = Some(port);
        Ok(())
    }

    pub fn close(&mut self) {
        self.port = None;
    }

    pub fn is_connected(&self) -> bool {
        self.port.is_some()
    }

    pub fn logs(&self) -> Vec<SerialLogEntry> {
        self.log.entries()
    }

    pub fn clear_logs(&mut self) {
        self.log.clear();
    }

    /// Send a frame and read up to `expect_reply` bytes (typically 1).
    fn transact(&mut self, frame: &[u8], expect_reply: usize, op_name: &str) -> Result<Vec<u8>> {
        self.open()?;
        let port = self
            .port
            .as_mut()
            .expect("port should be open after open()");
        // Best-effort flush of stale bytes from the receive buffer.
        let _ = port.clear(serialport::ClearBuffer::Input);

        self.log.push("TX", frame, op_name);

        port.write_all(frame).context("writing serial frame")?;
        port.flush().context("flushing serial frame")?;
        let mut buf = vec![0u8; expect_reply];
        let mut filled = 0;
        while filled < expect_reply {
            match port.read(&mut buf[filled..]) {
                Ok(0) => break,
                Ok(n) => filled += n,
                Err(e) if e.kind() == std::io::ErrorKind::TimedOut => break,
                Err(e) => {
                    // Drop the handle so the next call retries from scratch.
                    self.port = None;
                    self.log
                        .push("RX", &[], &format!("Error reading reply: {}", e));
                    return Err(anyhow::Error::from(e).context("reading serial reply"));
                }
            }
        }
        buf.truncate(filled);

        let rx_summary = if buf.is_empty() {
            "No response (timeout)".to_string()
        } else {
            describe_reply(&buf)
        };
        self.log.push("RX", &buf, &rx_summary);

        Ok(buf)
    }

    // ---- Getters --------------------------------------------------------

    pub fn get_power(&mut self) -> Result<Option<bool>> {
        let r = self.transact(&cmd::get(Op::GetPower), 1, "Get Power")?;
        Ok(r.first().and_then(|b| protocol::parse_bool(*b)))
    }
    pub fn get_brightness(&mut self) -> Result<Option<u8>> {
        let r = self.transact(&cmd::get(Op::GetBrightness), 1, "Get Brightness")?;
        Ok(r.first().and_then(|b| protocol::parse_percent(*b)))
    }
    pub fn get_contrast(&mut self) -> Result<Option<u8>> {
        let r = self.transact(&cmd::get(Op::GetContrast), 1, "Get Contrast")?;
        Ok(r.first().and_then(|b| protocol::parse_percent(*b)))
    }
    pub fn get_volume(&mut self) -> Result<Option<u8>> {
        let r = self.transact(&cmd::get(Op::GetVolume), 1, "Get Volume")?;
        Ok(r.first().and_then(|b| protocol::parse_percent(*b)))
    }
    pub fn get_mute(&mut self) -> Result<Option<bool>> {
        let r = self.transact(&cmd::get(Op::GetMute), 1, "Get Mute")?;
        Ok(r.first().and_then(|b| protocol::parse_bool(*b)))
    }
    pub fn get_input(&mut self) -> Result<Option<InputSource>> {
        let r = self.transact(&cmd::get(Op::GetInput), 1, "Get Input")?;
        Ok(r.first().and_then(|b| InputSource::from_byte(*b)))
    }

    // ---- Setters --------------------------------------------------------

    pub fn set_power(&mut self, on: bool) -> Result<()> {
        self.transact(
            &cmd::set_bool(Op::SetPower, on),
            1,
            &format!("Set Power = {}", if on { "On" } else { "Off" }),
        )?;
        Ok(())
    }
    pub fn set_brightness(&mut self, value: u8) -> Result<()> {
        self.transact(
            &cmd::set_percent(Op::SetBrightness, value.min(100)),
            1,
            &format!("Set Brightness = {}%", value.min(100)),
        )?;
        Ok(())
    }
    pub fn set_contrast(&mut self, value: u8) -> Result<()> {
        self.transact(
            &cmd::set_percent(Op::SetContrast, value.min(100)),
            1,
            &format!("Set Contrast = {}%", value.min(100)),
        )?;
        Ok(())
    }
    pub fn set_volume(&mut self, value: u8) -> Result<()> {
        self.transact(
            &cmd::set_percent(Op::SetVolume, value.min(100)),
            1,
            &format!("Set Volume = {}%", value.min(100)),
        )?;
        Ok(())
    }
    pub fn set_mute(&mut self, muted: bool) -> Result<()> {
        self.transact(
            &cmd::set_bool(Op::SetMute, muted),
            1,
            &format!("Set Mute = {}", if muted { "On" } else { "Off" }),
        )?;
        Ok(())
    }
    pub fn set_input(&mut self, source: InputSource) -> Result<()> {
        self.transact(
            &cmd::set_input(source),
            1,
            &format!("Set Input = {:?}", source),
        )?;
        Ok(())
    }
}

fn describe_reply(bytes: &[u8]) -> String {
    if bytes.len() == 1 {
        let b = bytes[0];
        match b {
            0xFF => "0xFF (Failure)".to_string(),
            0x00 => "0x00".to_string(),
            0x01 => "0x01".to_string(),
            2..=100 => format!("0x{:02X} ({})", b, b),
            _ => format!("0x{:02X}", b),
        }
    } else {
        bytes
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(" ")
    }
}
