# Changelog

All notable changes to this project are documented in this file. The
format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Cross-platform Tauri 2 desktop application shell for Windows, Linux
  and macOS.
- Vue 3 + TypeScript + Tailwind frontend with a Fluent / Windows 11
  inspired dark theme.
- Dashboard controls:
  - Power toggle (Fluent switch).
  - Volume, brightness and contrast sliders with a drag flyout
    tooltip; commands are dispatched on pointer release to avoid
    flooding the serial line.
  - Mute toggle paired with the volume slider.
  - Input source selector for VGA, HDMI, DP, DVI and AV.
- COM configuration panel with port discovery, baud rate, data bits,
  stop bits, parity, flow control and read timeout; settings are
  persisted to the platform config directory as JSON.
- Non-destructive "Test Connection" probe that opens the configured
  port without sending any frame.
- Embedded Axum HTTP server bound to `0.0.0.0:8765`, serving both the
  REST/JSON API and the compiled Vue bundle (embedded into the Rust
  binary via `rust-embed`).
- Mobile access: any phone on the same network can open
  `http://<host>:8765` to use the identical dashboard; the COM panel
  surfaces the reachable URLs automatically.
- BSD-E6 RS-232 protocol module in Rust with:
  - Strongly typed op codes and input sources.
  - XOR checksum helper.
  - Unit tests pinned against the vendor's worked examples
    (Power on / Brightness 100 / Volume 50).
- Serial port manager that owns the handle, serialises half-duplex
  transactions and lazily reopens on configuration change.
- REST API endpoints: `/api/health`, `/api/info`, `/api/status`,
  `/api/power`, `/api/brightness`, `/api/contrast`, `/api/volume`,
  `/api/mute`, `/api/input`, `/api/com` (GET/POST) and
  `/api/com/test`.
- Origin-aware API client that targets the embedded server from the
  Tauri WebView and the same origin from mobile browsers.
- Documentation set under `docs/`:
  - `ARCHITECTURE.md` - design, threading model, persistence.
  - `PROTOCOL.md` - RS-232 wire protocol reference and examples.
  - `API.md` - REST endpoint reference.
  - `DEVELOPMENT.md` - prerequisites, build and test instructions.
  - `USAGE.md` - operator guide and troubleshooting.
  - `README.md` - documentation index.
- Project scaffolding: `package.json`, `vite.config.ts`,
  `tsconfig.json`, `tailwind.config.js`, `postcss.config.js`,
  `index.html`, Tauri configuration and capability manifest, Cargo
  manifest and `build.rs`.

### Changed

- Replaced the original two-line `README.md` with a full project
  overview covering features, tech stack, quick start, project layout
  and links to the new documentation set.
- Renamed the vendor protocol reference file to
  `BSD-E6-Programming .txt` for editor compatibility.

### Notes

- Tauri icons (`src-tauri/icons/*`) are required for `tauri build` and
  must be generated locally via `npx @tauri-apps/cli icon`.
- `dist/index.html` is a placeholder; the real bundle is produced by
  `npm run build` and embedded into the Rust binary at compile time.
