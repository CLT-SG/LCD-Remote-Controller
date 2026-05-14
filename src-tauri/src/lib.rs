//! Application entry point invoked from `main.rs`. The Tauri runtime owns the
//! desktop window; we additionally spawn an axum HTTP server on a tokio
//! runtime so phones on the same network can reach the same UI through their
//! browser. The desktop WebView itself loads the embedded server URL in
//! production builds, so there is exactly one frontend code path.

mod config;
mod http_server;
mod protocol;
mod serial_manager;
mod state;

use std::sync::Arc;

use tokio::sync::Mutex;

use crate::config::AppConfig;
use crate::http_server::{serve, DEFAULT_PORT};
use crate::state::Inner;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let config = AppConfig::load();
    let state = Arc::new(Mutex::new(Inner::new(config)));

    // Spawn the HTTP server on a dedicated tokio runtime so it lives
    // independently of any Tauri command invocation.
    let server_state = state.clone();
    std::thread::Builder::new()
        .name("http-server".into())
        .spawn(move || {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("tokio runtime");
            rt.block_on(async move {
                if let Err(e) = serve(server_state, DEFAULT_PORT).await {
                    tracing::error!(?e, "HTTP server crashed");
                }
            });
        })
        .expect("spawn http-server thread");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|_app| Ok(()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
