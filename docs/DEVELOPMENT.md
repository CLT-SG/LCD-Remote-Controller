# Development Guide

## Prerequisites

| Tool         | Version |
|--------------|---------|
| Node.js      | ≥ 18    |
| npm          | ≥ 9     |
| Rust toolchain | ≥ 1.77 (`rustup default stable`) |
| Tauri prerequisites | Per platform — see [https://v2.tauri.app/start/prerequisites/](https://v2.tauri.app/start/prerequisites/) |

### Linux extras (Debian/Ubuntu)

```bash
sudo apt install -y libwebkit2gtk-4.1-dev build-essential curl wget \
                    libssl-dev libgtk-3-dev libayatana-appindicator3-dev \
                    librsvg2-dev libudev-dev pkg-config
```

`libudev-dev` is required by `serialport`.

### Windows

* Install **WebView2 runtime** (preinstalled on Windows 11).
* Install **Visual Studio Build Tools** (C++ workload).
* The Rust serial driver uses the Windows API directly, no extra setup
  needed.

### macOS

* Install Xcode Command Line Tools.

## Install

```bash
npm install
```

The first `tauri:dev` will fetch and compile the Rust crates; allow
~5 minutes on a clean checkout.

## Run (development)

```bash
npm run tauri:dev
```

This:

1. starts Vite at `http://localhost:5173`,
2. compiles the Rust binary in debug mode,
3. opens the Tauri window pointing at the embedded HTTP server, and
4. exposes the API + UI on `http://0.0.0.0:8765` so a phone on the same
   Wi-Fi network can connect.

### Without Tauri

You can also run only the headless backend + Vite for browser-only
development:

```bash
# terminal 1
cargo run --manifest-path src-tauri/Cargo.toml
# terminal 2
npm run dev
```

## Build (production)

```bash
npm run tauri:build
```

Artefacts land in `src-tauri/target/release/bundle/`:

* Windows: `.msi`, `.exe`
* Linux: `.AppImage`, `.deb`
* macOS: `.dmg`, `.app`

## Generating icons

Tauri requires platform icons. Generate them once from a 1024×1024 source PNG:

```bash
npx @tauri-apps/cli icon assets/source-icon.png
```

The CLI populates `src-tauri/icons/`.

## Tests

```bash
cargo test --manifest-path src-tauri/Cargo.toml
```

The `protocol` module ships with checksum tests pinned against the
spec examples.

## Code layout

See `docs/ARCHITECTURE.md`. In short:

```
src/                  # Vue frontend
src-tauri/src/        # Rust desktop binary
docs/                 # This documentation
```
