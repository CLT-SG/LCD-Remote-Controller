# LCD Remote Controller

Modern, cross-platform dashboard for remote-controlling **BPLRT-BSD-E6 series LCD
displays** over RS-232. The desktop application also exposes the same UI
to phones on the same Wi-Fi network through an embedded HTTP server, so
operators can use whichever device is closer at hand.

> Originally built for BPLRT (Bukit Panjang LRT) operations.

## Features

- **Power** toggle (Fluent UI switch).
- **Volume**, **Brightness**, **Contrast** sliders with a Windows 11-style
  flyout tooltip that updates while dragging.
- **Mute** toggle, paired with the volume slider.
- **Input source** selector — VGA / HDMI / DP / DVI / AV.
- **COM configuration** panel with port discovery, baud / framing options
  and a "Test Connection" probe; settings are persisted to disk.
- **Mobile access**: open `http://<host>:8765` on any phone on the LAN
  to get the same dashboard.

## Tech stack

| Layer        | Technology                                 |
|--------------|---------------------------------------------|
| Desktop shell| [Tauri 2](https://v2.tauri.app/) (Rust)     |
| HTTP server  | [Axum](https://github.com/tokio-rs/axum) on Tokio |
| Serial       | [`serialport`](https://crates.io/crates/serialport) crate |
| Frontend     | [Vue 3](https://vuejs.org/) + TypeScript    |
| Styling      | [Tailwind CSS](https://tailwindcss.com/) (Fluent palette) |
| Icons        | [`lucide-vue-next`](https://lucide.dev/)    |
| Bundling     | Vite, embedded into the Rust binary via `rust-embed` |

## Quick start

```bash
# 1. install JavaScript & Rust deps
npm install

# 2. run in development (opens Tauri window + serves the API on :8765)
npm run tauri:dev

# 3. on a phone on the same network, open one of the URLs printed in the
#    "COM" → "Mobile Access" panel, e.g.  http://192.168.1.42:8765
```

To produce installers/packages for Windows / Linux / macOS:

```bash
npm run tauri:build
```

See [`docs/DEVELOPMENT.md`](docs/DEVELOPMENT.md) for full prerequisites.

## Project layout

```
.
├── src/                 # Vue 3 frontend (used by both desktop & mobile)
├── src-tauri/           # Rust desktop binary, serial driver, HTTP API
├── docs/                # Architecture, protocol, API & user docs
├── BSD-E6-Programming   # Original vendor protocol reference
├── package.json
└── README.md
```

## Documentation

| Topic                                | Link                                   |
|--------------------------------------|----------------------------------------|
| High-level architecture              | [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) |
| BSD-E6 RS-232 wire protocol          | [docs/PROTOCOL.md](docs/PROTOCOL.md)   |
| HTTP API reference                   | [docs/API.md](docs/API.md)             |
| Building & contributing              | [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md) |
| Operator guide                       | [docs/USAGE.md](docs/USAGE.md)         |

## License

MIT — see `LICENSE` (add per organisation policy).
