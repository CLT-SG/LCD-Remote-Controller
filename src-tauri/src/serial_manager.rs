//! Thin wrapper around `serialport` that owns the open handle and serialises
//! request / response cycles. The BSD-E6 protocol is half-duplex: send 8 bytes,
//! read 1 reply byte. The manager keeps the port closed when there is no
//! configured port name so the rest of the application can still be inspected
//! and configured before any hardware is connected.

use std::{io::{Read, Write}, time::Duration};

use anyhow::{Context, Result};
use serialport::SerialPort;

use crate::config::ComConfig;
use crate::protocol::{self, cmd, InputSource, Op};

pub struct SerialManager {
    config: ComConfig,
    port: Option<Box<dyn SerialPort>>,
}

impl SerialManager {
    pub fn new(config: ComConfig) -> Self {
        Self { config, port: None }
    }

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

    /// Send a frame and read up to `expect_reply` bytes (typically 1).
    fn transact(&mut self, frame: &[u8], expect_reply: usize) -> Result<Vec<u8>> {
        self.open()?;
        let port = self
            .port
            .as_mut()
            .expect("port should be open after open()");
        // Best-effort flush of stale bytes from the receive buffer.
        let _ = port.clear(serialport::ClearBuffer::Input);
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
                    return Err(anyhow::Error::from(e).context("reading serial reply"));
                }
            }
        }
        buf.truncate(filled);
        Ok(buf)
    }

    // ---- Getters --------------------------------------------------------

    pub fn get_power(&mut self) -> Result<Option<bool>> {
        let r = self.transact(&cmd::get(Op::GetPower), 1)?;
        Ok(r.first().and_then(|b| protocol::parse_bool(*b)))
    }
    pub fn get_brightness(&mut self) -> Result<Option<u8>> {
        let r = self.transact(&cmd::get(Op::GetBrightness), 1)?;
        Ok(r.first().and_then(|b| protocol::parse_percent(*b)))
    }
    pub fn get_contrast(&mut self) -> Result<Option<u8>> {
        let r = self.transact(&cmd::get(Op::GetContrast), 1)?;
        Ok(r.first().and_then(|b| protocol::parse_percent(*b)))
    }
    pub fn get_volume(&mut self) -> Result<Option<u8>> {
        let r = self.transact(&cmd::get(Op::GetVolume), 1)?;
        Ok(r.first().and_then(|b| protocol::parse_percent(*b)))
    }
    pub fn get_mute(&mut self) -> Result<Option<bool>> {
        let r = self.transact(&cmd::get(Op::GetMute), 1)?;
        Ok(r.first().and_then(|b| protocol::parse_bool(*b)))
    }
    pub fn get_input(&mut self) -> Result<Option<InputSource>> {
        let r = self.transact(&cmd::get(Op::GetInput), 1)?;
        Ok(r.first().and_then(|b| InputSource::from_byte(*b)))
    }

    // ---- Setters --------------------------------------------------------

    pub fn set_power(&mut self, on: bool) -> Result<()> {
        self.transact(&cmd::set_bool(Op::SetPower, on), 1)?;
        Ok(())
    }
    pub fn set_brightness(&mut self, value: u8) -> Result<()> {
        self.transact(&cmd::set_percent(Op::SetBrightness, value.min(100)), 1)?;
        Ok(())
    }
    pub fn set_contrast(&mut self, value: u8) -> Result<()> {
        self.transact(&cmd::set_percent(Op::SetContrast, value.min(100)), 1)?;
        Ok(())
    }
    pub fn set_volume(&mut self, value: u8) -> Result<()> {
        self.transact(&cmd::set_percent(Op::SetVolume, value.min(100)), 1)?;
        Ok(())
    }
    pub fn set_mute(&mut self, muted: bool) -> Result<()> {
        self.transact(&cmd::set_bool(Op::SetMute, muted), 1)?;
        Ok(())
    }
    pub fn set_input(&mut self, source: InputSource) -> Result<()> {
        self.transact(&cmd::set_input(source), 1)?;
        Ok(())
    }
}
