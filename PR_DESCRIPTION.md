# Fix LCD Status GET Commands and Add RS-232 Communication Log

## Overview
This PR resolves a critical protocol bug that prevented the application from reading any LCD panel status, and adds a live RS-232 communication log to the dashboard so operators can inspect every frame sent and received.

## Problem

1. **GET commands were malformed.** The BSD-E6 datasheet specifies that status-read frames use op-code `0x01` with the function code (`0x30..0x35`) placed in the parameter byte. The existing implementation swapped these two bytes (e.g. `0x30 0x01` instead of `0x01 0x30`). The display therefore did not recognise any GET request. Because the serial port itself opened successfully, the UI reported "Connected", yet every polled value remained `null` and no error was surfaced.

2. **No traffic visibility.** When a command failed or the display did not respond, there was no way for an operator or maintainer to see the raw hex traffic. Troubleshooting required external tools such as Hercules, which confirmed the wiring was fine but did not help debug the application itself.

## Solution

### Protocol fix
- Added `Op::Get = 0x01` to the `Op` enum in `src-tauri/src/protocol.rs`.
- Changed `cmd::get` to call `build_frame(Op::Get, op as u8)` so every GET frame now matches the vendor spec.
- Updated the unit test to assert the exact byte sequence documented in `BSD-E6-Programming .txt`.

### Communication log
- Introduced `SerialLogEntry` and `SerialLog` (bounded ring buffer, 200 entries) inside `src-tauri/src/serial_manager.rs`.
- Every `transact` call now records:
  - **TX** — timestamp, raw hex bytes, human-readable operation name.
  - **RX** — timestamp, raw hex bytes, summary such as `0x01`, `0xFF (Failure)`, or `No response (timeout)`.
- Added `logs()` and `clear_logs()` methods to `Inner` in `src-tauri/src/state.rs`.
- Exposed two new HTTP endpoints in `src-tauri/src/http_server.rs`:
  - `GET /api/logs` — returns the current log array.
  - `DELETE /api/logs` — clears the buffer.

### Dashboard integration
- Created `src/components/CommunicationLog.vue`: a scrollable, colour-coded panel (sky for TX, emerald for RX) with a Clear button.
- Restructured `src/App.vue` into a two-column `lg:grid-cols-3` layout:
  - Left column (`lg:col-span-2`) holds all existing control cards.
  - Right column (`lg:col-span-1`) hosts the new **Communication Log** card.
- Status polling now fetches both device status and logs every 5 seconds; every user command also refreshes the log immediately.

### Documentation updates
- Updated `docs/API.md` with the new `/api/logs` endpoints.
- Updated `docs/UI.md` layout diagram and component table to include `CommunicationLog.vue`.
- Added `v1.0.6` entry to `CHANGELOG.md`.

## Files changed

| File | Change |
|------|--------|
| `src-tauri/src/protocol.rs` | Added `Op::Get = 0x01`; fixed `cmd::get`; updated unit test. |
| `src-tauri/src/serial_manager.rs` | Added `SerialLogEntry`, `SerialLog`; instrumented `transact` with TX/RX logging. |
| `src-tauri/src/state.rs` | Added `logs()` and `clear_logs()` delegation methods. |
| `src-tauri/src/http_server.rs` | Added `GET /api/logs` and `DELETE /api/logs` handlers. |
| `src/types.ts` | Added `SerialLogEntry` TypeScript interface. |
| `src/api.ts` | Added `logs()` and `clearLogs()` API client methods. |
| `src/App.vue` | Integrated `CommunicationLog`; restructured layout; added `loadLogs` / `onClearLogs`. |
| `src/components/CommunicationLog.vue` | New component for displaying RS-232 traffic. |
| `docs/API.md` | Documented `/api/logs` endpoints. |
| `docs/UI.md` | Updated layout diagram and component table. |
| `CHANGELOG.md` | Added `v1.0.6` release notes. |

## Verification

- `cargo test` in `src-tauri` passes, including the corrected GET checksum test.
- `npm run build` in the Vue frontend completes without type or lint errors.
- The GET Power frame now produces `B2 00 00 3A 01 01 30 0A`, matching the vendor datasheet exactly.

## Deployment notes

No database migrations or configuration changes are required. Existing `config.json` files remain valid. After upgrading, operators should see live RS-232 traffic appear in the right-hand dashboard panel as soon as a COM port is configured and the display is polled.
