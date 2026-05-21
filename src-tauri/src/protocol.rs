//! BPLRT-BSD-E6 series RS-232 protocol.
//!
//! All commands have the form:
//!
//! ```text
//! B2 00 00 3A 01 <op> <param> <checksum>
//! ```
//!
//! where `<checksum>` is the XOR of every byte from `0x3A` up to and
//! including `<param>` (i.e. `3A ^ 01 ^ op ^ param`).
//!
//! Responses returned by the display are a single byte:
//!
//! * `0..=100` for percent-style getters (brightness/contrast/volume),
//! * `0` / `1` for boolean getters (power / mute),
//! * `0..=N` for input source getters,
//! * `255` (`0xFF`) on failure.
//!
//! See `docs/PROTOCOL.md` for the full reference.

use serde::{Deserialize, Serialize};

const HEADER: [u8; 5] = [0xB2, 0x00, 0x00, 0x3A, 0x01];

/// Op-codes recognised by the BSD-E6 firmware.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Op {
    Get = 0x01,
    GetPower = 0x30,
    GetBrightness = 0x31,
    GetContrast = 0x32,
    GetVolume = 0x33,
    GetMute = 0x34,
    GetInput = 0x35,
    SetPower = 0x40,
    SetBrightness = 0x41,
    SetContrast = 0x42,
    SetVolume = 0x43,
    SetMute = 0x44,
    SetInput = 0x45,
}

/// Logical input sources. The numeric value matches the wire encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InputSource {
    Vga = 0,
    Hdmi = 1,
    Dp = 2,
    Dvi = 3,
    Av = 4,
}

impl InputSource {
    pub fn from_byte(b: u8) -> Option<Self> {
        Some(match b {
            0 => Self::Vga,
            1 => Self::Hdmi,
            2 => Self::Dp,
            3 => Self::Dvi,
            4 => Self::Av,
            _ => return None,
        })
    }
}

/// XOR checksum over `0x3A`, `0x01`, op-code and parameter.
pub fn checksum(op: u8, param: u8) -> u8 {
    0x3A ^ 0x01 ^ op ^ param
}

/// Build the 8-byte command frame for the given op / parameter.
pub fn build_frame(op: Op, param: u8) -> [u8; 8] {
    let mut out = [0u8; 8];
    out[..5].copy_from_slice(&HEADER);
    out[5] = op as u8;
    out[6] = param;
    out[7] = checksum(op as u8, param);
    out
}

/// Convenience helpers for each command. Param value ranges are clamped where
/// needed by the caller; `set_percent` does not clamp so the UI layer can
/// surface validation errors.
pub mod cmd {
    use super::*;
    pub fn get(op: Op) -> [u8; 8] {
        build_frame(Op::Get, op as u8)
    }
    pub fn set_bool(op: Op, on: bool) -> [u8; 8] {
        build_frame(op, if on { 1 } else { 0 })
    }
    pub fn set_percent(op: Op, value: u8) -> [u8; 8] {
        build_frame(op, value)
    }
    pub fn set_input(src: InputSource) -> [u8; 8] {
        build_frame(Op::SetInput, src as u8)
    }
}

/// Parse a single response byte into a percentage value (0..=100), or `None`
/// when the device returned the failure sentinel `0xFF`.
pub fn parse_percent(byte: u8) -> Option<u8> {
    if byte == 0xFF || byte > 100 {
        None
    } else {
        Some(byte)
    }
}

/// Parse a single response byte into a boolean (0 / 1), or `None` on failure.
pub fn parse_bool(byte: u8) -> Option<bool> {
    match byte {
        0 => Some(false),
        1 => Some(true),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksum_matches_spec_examples() {
        // Set Brightness = 100  ->  B2 00 00 3A 01 41 64 1E
        assert_eq!(build_frame(Op::SetBrightness, 0x64)[7], 0x1E);
        // Set Power = On        ->  B2 00 00 3A 01 40 01 7A
        assert_eq!(build_frame(Op::SetPower, 0x01)[7], 0x7A);
        // Set Volume = 50       ->  B2 00 00 3A 01 43 32 4A
        assert_eq!(build_frame(Op::SetVolume, 0x32)[7], 0x4A);
        // Get Power status      ->  B2 00 00 3A 01 01 30 0A
        let g = cmd::get(Op::GetPower);
        assert_eq!(&g[..], &[0xB2, 0x00, 0x00, 0x3A, 0x01, 0x01, 0x30, 0x0A]);
    }

    #[test]
    fn percent_failure_byte_is_none() {
        assert_eq!(parse_percent(0xFF), None);
        assert_eq!(parse_percent(101), None);
        assert_eq!(parse_percent(0), Some(0));
        assert_eq!(parse_percent(100), Some(100));
    }
}
