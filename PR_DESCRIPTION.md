# Fix Serial Timeouts, Select Dark Mode Styling, and Error Log Visibility

## Overview
This PR addresses three operational issues: the display consistently timing out on status polls because commands were sent too rapidly back-to-back; unreadable native select dropdowns in dark mode on Windows; and timeouts being logged ambiguously as empty RX entries.

## Problem

1. **RX timeout on every poll.** `Inner::poll()` sends six getter commands in rapid succession. The BSD-E6 display does not have enough time to process each frame before the next one arrives, so every reply times out. The log shows a burst of TX entries followed by RX entries with `No response (timeout)`. Increasing the read timeout alone does not help because the root cause is traffic pacing, not the display being slow.

2. **Native select dropdown unreadable in dark mode on Windows.** The Tauri WebView2 on Windows renders `<select>` option text with the OS default background, which becomes white-on-white when the application is in dark mode. No explicit CSS was styling `select option`, so operators could not read the port list or baud-rate dropdown.

3. **Timeout and errors logged as RX.** When the display did not reply, the code logged an empty byte array under direction `RX` with summary `No response (timeout)`. This made it hard to scan the log for actual failures because every empty RX looked like a successful (but empty) reply.

## Solution

### Serial timing enforcement
- Added `post_open_delay_ms` and `inter_command_delay_ms` fields to `ComConfig` in `src-tauri/src/config.rs`. Defaults are 200 ms and 100 ms respectively.
- Extended `SerialManager` in `src-tauri/src/serial_manager.rs` with two `Instant` trackers: `last_port_open` and `last_tx`.
- Rewrote `transact()` to:
  - Wait `post_open_delay_ms` after a fresh port open before the first command.
  - Wait `inter_command_delay_ms` since the previous command before sending the next one.
  - Reset timing state on port close or configuration change to avoid stale delays.
- This serialises traffic one command at a time with a configurable pause, matching the display's processing capacity.

### Select dropdown dark mode fix
- Added explicit `color-scheme: dark` / `color-scheme: light` on `select` elements.
- Added `background-color` and `color` rules for `select option` under both themes in `src/style.css`.
- This forces the WebView2 to render dark background text in dark mode and light background text in light mode, regardless of OS defaults.

### Error log separation
- Changed timeout and serial I/O error logging from direction `RX` to a new direction `ERR` in `src-tauri/src/serial_manager.rs`.
- Updated `SerialLogEntry.direction` in `src/types.ts` to include `"ERR"`.
- Added red legend and badge styling (`bg-red-500/20 text-red-300`) for `ERR` entries in `src/components/CommunicationLog.vue`.

### UI configuration panel
- Added two new numeric input fields to `src/components/ComConfigPanel.vue` for `post_open_delay_ms` and `inter_command_delay_ms`.
- Added a descriptive helper text under the inter-command delay field so operators know when to increase it.

### Documentation
- Updated `docs/ARCHITECTURE.md` concurrency model section to describe the new serial timing delays.
- Updated `docs/API.md` `GET /api/com` example and log format to include the new fields and `ERR` direction.
- Added a "Timing settings" subsection and timeout troubleshooting row to `docs/USAGE.md`.
- Created `docs/COMMUNICATION_LOG.md` — a new guide explaining log format, directions, and how to diagnose timeouts.

## Files changed

| File | Change |
|------|--------|
| `src-tauri/src/config.rs` | Added `post_open_delay_ms` and `inter_command_delay_ms` to `ComConfig` with defaults. |
| `src-tauri/src/serial_manager.rs` | Added `last_tx` / `last_port_open` timing; enforced delays in `transact()`; timeouts and errors now log as `ERR`. |
| `src/types.ts` | Added `post_open_delay_ms`, `inter_command_delay_ms` to `ComConfig`; added `"ERR"` to `SerialLogEntry.direction`. |
| `src/components/ComConfigPanel.vue` | Added inputs for post-open and inter-command delay; added helper description text. |
| `src/components/CommunicationLog.vue` | Added `ERR` legend and red badge styling. |
| `src/style.css` | Added `color-scheme` and `select option` colour overrides for dark/light mode. |
| `docs/API.md` | Updated `ComConfig` example and log format to include new fields and `ERR`. |
| `docs/ARCHITECTURE.md` | Updated concurrency model to describe serial timing delays. |
| `docs/USAGE.md` | Added timing settings section and timeout troubleshooting guidance. |
| `docs/COMMUNICATION_LOG.md` | New documentation explaining log format, directions, and timeout diagnosis. |

## Verification

- `cargo check` and `cargo test` pass on the Rust backend.
- `tsc --noEmit` passes with zero type errors in the Vue frontend.
- All existing unit tests continue to pass.

## Deployment notes

Existing `config.json` files will load safely because the two new fields have `Default` values (`post_open_delay_ms: 200`, `inter_command_delay_ms: 100`). After upgrading, operators can adjust the delays in the COM panel if the display still does not reply reliably.
