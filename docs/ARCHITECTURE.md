# Architecture

## High level

```
            ┌─────────────────────────────────────────────┐
            │             LCD Remote Controller           │
            │            (Tauri desktop binary)           │
            │                                             │
   USB/RS-232│  ┌────────────┐    ┌──────────────────┐   │
LCD ◄────────┼─►│  Serial     │◄──►│  Axum HTTP/JSON  │◄──┼── Wi-Fi ── 📱 phone
            │  │  Manager    │    │     server       │   │              browser
            │  └────────────┘    └────────▲─────────┘   │
            │         ▲                   │             │
            │         │                   │             │
            │  ┌──────┴────────┐  ┌───────┴─────────┐   │
            │  │  AppConfig    │  │  Tauri WebView  │   │
            │  │  (JSON file)  │  │  (Vue + Tailwind│   │
            │  └───────────────┘  │   loaded from   │   │
            │                      │   embedded HTTP)│   │
            │                      └─────────────────┘   │
            └─────────────────────────────────────────────┘
```

The desktop binary owns three concerns:

1. **Serial transport** (`src-tauri/src/serial_manager.rs`) — opens the
   configured COM port and performs synchronous request / response cycles
   following the BSD-E6 protocol.
2. **REST/JSON API** (`src-tauri/src/http_server.rs`) — an Axum server bound
   to `0.0.0.0:8765`. It is the single point of access for all clients.
3. **Embedded UI** — the compiled Vue bundle is embedded into the binary at
   build time (via `rust-embed`) and served by the same HTTP server, so
   phones on the same network can reach `http://<host>:8765/` and use the
   exact same UI as the desktop.

## Why a unified HTTP server?

Using a single API layer keeps the codebase simple and consistent:

* The Vue frontend only knows REST. It does not differentiate desktop vs.
  mobile.
* The Tauri WebView is pointed at the embedded server (`127.0.0.1:8765`)
  so behaviour is identical to a phone hitting the LAN IP.
* No `tauri::command` boilerplate is required for the device API surface.

## Concurrency model

* The `tokio` runtime drives Axum.
* Shared state is `Arc<tokio::sync::Mutex<Inner>>`. The mutex guards the
  serial port handle and the cached device snapshot. RS-232 is half-duplex,
  so serialising access also avoids interleaved frames on the wire.
* The Tauri runtime runs on the main thread. The HTTP server is spawned on
  a dedicated worker thread with its own multi-threaded tokio runtime so
  it cannot deadlock UI callbacks.

## Configuration persistence

`AppConfig` (`src-tauri/src/config.rs`) is serialised to JSON inside the
platform config dir:

* Linux: `~/.config/lcd-remote-controller/config.json`
* macOS: `~/Library/Application Support/com.bplrt.lcd-remote-controller/config.json`
* Windows: `%APPDATA%\bplrt\lcd-remote-controller\config\config.json`

It is loaded once on startup and re-saved every time `POST /api/com`
succeeds.

## Frontend layout

```
src/
├── App.vue                    # Top-level dashboard
├── api.ts                     # fetch() wrapper, origin-aware base URL
├── types.ts                   # Shared TypeScript types
└── components/
    ├── DashboardCard.vue      # Translucent Fluent surface card
    ├── FluentSlider.vue       # Windows 11-style slider with flyout
    ├── PowerToggle.vue        # Fluent toggle switch
    ├── InputSourceSelect.vue  # Segmented input source picker
    └── ComConfigPanel.vue     # Serial port configuration form
```

The frontend polls `GET /api/status` every 5 seconds, and immediately after
any user action (each `POST /api/*` returns the refreshed snapshot).
