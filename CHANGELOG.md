# Changelog

All notable changes to this project are documented in this file. The
format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v1.0.6 (21 May 2026)

### Added

- **RS-232 Communication Log** displayed in a dedicated dashboard card on
  the right-hand column. Every TX and RX frame is timestamped and shown in
  hex, making it trivial to verify that commands actually reach the display
  and to inspect replies. The log is a bounded ring buffer (200 entries)
  stored in `SerialManager`; operators can clear it with the **Clear**
  button inside the card.
- `GET /api/logs` and `DELETE /api/logs` endpoints so the Vue frontend can
  read and clear the serial traffic log.
- `src/components/CommunicationLog.vue` — scrollable, colour-coded log
  panel (sky for TX, emerald for RX) with a responsive empty state.

### Fixed

- **GET command frame construction** in `src-tauri/src/protocol.rs`. The
  BSD-E6 datasheet requires status-read frames to use op-code `0x01` with the
  function code (`0x30..0x35`) in the parameter byte. Previously the code
  swapped these two bytes, producing an unrecognised frame (e.g.
  `B2 00 00 3A 01 30 01 0A` instead of the correct
  `B2 00 00 3A 01 01 30 0A`). This caused the LCD panel to ignore every
  status request: the app reported "Connected" because the serial port
  opened, but every polled value stayed `null`. Hercules and similar
  terminals worked because they sent the correct byte sequence manually.
  The fix adds `Op::Get = 0x01` and changes `cmd::get` to
  `build_frame(Op::Get, op as u8)`, verified against the vendor examples.

### Changed

- Dashboard layout upgraded to `lg:grid-cols-3`. The left two columns
  (`lg:col-span-2`) contain all existing control cards; the right column
  (`lg:col-span-1`) hosts the new **Communication Log** card. On smaller
  screens the log stacks below the controls as before.
- Polling interval now fetches both device status and logs in parallel
  (`refreshAll`) so the traffic view stays in sync with the controls.
- Applied `cargo fmt` formatting to `src-tauri/src/http_server.rs` and
  `src-tauri/src/serial_manager.rs` so the CI lint pipeline passes cleanly.

## v1.0.5 (14 May 2026)

### Added

- **CLT brand mark** in the dashboard header. The component
  (`src/components/CltLogo.vue`) renders the canonical application
  icon (`src-tauri/icons/512x512.png`, mirrored at
  `src/assets/logo.png` for Vite bundling).
- **Light / dark theme toggle** placed beside the refresh button.
  Implemented in `src/composables/useTheme.ts`; the selection is
  persisted in `localStorage` under `clt.theme` and falls back to the
  OS-level `prefers-color-scheme` preference on first run.
- **Mobile Access** promoted to its own dashboard card, rendered
  directly below the Contrast container so operators can see the LAN
  URLs without opening the COM settings panel.
- `docs/UI.md` describing the dashboard layout, theming model and
  component responsibilities; linked from `docs/README.md`.
- `*.png` and `*.svg` module declarations in `src/shims-vue.d.ts` so
  TypeScript accepts asset imports.
- Light-theme overrides scoped under `html.light` in `src/style.css`,
  including a tuned scrollbar and a smooth background transition.

### Changed

- Renamed the application from "BSD-E6 LCD Remote Controller" to
  **"CLT LCD Remote Controller"** in the dashboard header and footer.
- Reorganised the dashboard grid:
  - **Power**, **Input Source** and **Volume** now share one row
    (`md:grid-cols-3`).
  - **Brightness** and **Contrast** share a second row
    (`md:grid-cols-2`).
  - **Mobile Access** occupies a full-width row underneath.
- Updated the hardware reference from `BSD-E6` to **`BPLRT-BSD-E6`**
  in `README.md`, `package.json`, `src-tauri/Cargo.toml`,
  `src-tauri/tauri.conf.json` (`longDescription`) and the module-level
  doc comment of `src-tauri/src/protocol.rs`.
- Bumped the application version to **`1.0.5`** in `package.json` and
  `src-tauri/Cargo.toml`.
- Increased the default Tauri window size from `1100x760` to
  **`1350x900`** to comfortably fit the new three-column primary row.

## v0.1.0

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
