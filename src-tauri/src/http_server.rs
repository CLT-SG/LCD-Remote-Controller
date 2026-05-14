//! Embedded HTTP server. This is the single source of truth for the UI: the
//! desktop WebView and any phone browser on the same network both consume the
//! REST API exposed here, and serve the same Vue bundle (embedded at compile
//! time via `rust-embed`).

use std::net::SocketAddr;

use axum::{
    extract::State,
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;

use crate::config::ComConfig;
use crate::protocol::InputSource;
use crate::serial_manager::SerialManager;
use crate::state::{AppState, DeviceSnapshot};

/// Frontend bundle is embedded at compile time. This makes the binary fully
/// self-contained — no `dist/` folder needs to ship alongside the app.
#[derive(RustEmbed)]
#[folder = "../dist/"]
struct FrontendAssets;

pub const DEFAULT_PORT: u16 = 8765;

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(health))
        .route("/api/info", get(info))
        .route("/api/status", get(status))
        .route("/api/power", post(set_power))
        .route("/api/volume", post(set_volume))
        .route("/api/brightness", post(set_brightness))
        .route("/api/contrast", post(set_contrast))
        .route("/api/mute", post(set_mute))
        .route("/api/input", post(set_input))
        .route("/api/com", get(get_com).post(set_com))
        .route("/api/com/test", post(test_com))
        .fallback(get(static_handler))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

pub async fn serve(state: AppState, port: u16) -> anyhow::Result<()> {
    let app = build_router(state);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!(%addr, "HTTP server listening");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

// ---------------- Helpers ----------------

#[derive(Debug, Serialize)]
struct ApiError {
    error: String,
}

fn err(status: StatusCode, msg: impl Into<String>) -> Response {
    (status, Json(ApiError { error: msg.into() })).into_response()
}

async fn refresh_and_return(state: AppState) -> Json<DeviceSnapshot> {
    let mut inner = state.lock().await;
    inner.refresh();
    Json(inner.snapshot.clone())
}

// ---------------- Handlers ----------------

async fn health() -> &'static str {
    "ok"
}

#[derive(Serialize)]
struct ServerInfo {
    hostname: String,
    addresses: Vec<String>,
    port: u16,
    version: &'static str,
}

async fn info() -> Json<ServerInfo> {
    let hostname = hostname::get()
        .ok()
        .and_then(|h| h.into_string().ok())
        .unwrap_or_else(|| "unknown".to_string());
    let mut addresses = Vec::new();
    if let Ok(list) = local_ip_address::list_afinet_netifas() {
        for (_name, ip) in list {
            if !ip.is_loopback() && ip.is_ipv4() {
                addresses.push(ip.to_string());
            }
        }
    }
    addresses.push(hostname.clone());
    Json(ServerInfo {
        hostname,
        addresses,
        port: DEFAULT_PORT,
        version: env!("CARGO_PKG_VERSION"),
    })
}

async fn status(State(state): State<AppState>) -> Json<DeviceSnapshot> {
    refresh_and_return(state).await
}

#[derive(Deserialize)]
struct PowerReq {
    on: bool,
}

async fn set_power(State(state): State<AppState>, Json(req): Json<PowerReq>) -> Response {
    let mut inner = state.lock().await;
    if let Err(e) = inner.serial.set_power(req.on) {
        return err(StatusCode::BAD_GATEWAY, format!("{e:#}"));
    }
    drop(inner);
    refresh_and_return(state).await.into_response()
}

#[derive(Deserialize)]
struct PercentReq {
    value: u8,
}

async fn set_volume(State(state): State<AppState>, Json(req): Json<PercentReq>) -> Response {
    set_percent(state, req.value, |s, v| s.set_volume(v)).await
}
async fn set_brightness(State(state): State<AppState>, Json(req): Json<PercentReq>) -> Response {
    set_percent(state, req.value, |s, v| s.set_brightness(v)).await
}
async fn set_contrast(State(state): State<AppState>, Json(req): Json<PercentReq>) -> Response {
    set_percent(state, req.value, |s, v| s.set_contrast(v)).await
}

async fn set_percent<F>(state: AppState, value: u8, f: F) -> Response
where
    F: FnOnce(&mut SerialManager, u8) -> anyhow::Result<()>,
{
    if value > 100 {
        return err(StatusCode::BAD_REQUEST, "value must be 0..=100");
    }
    let mut inner = state.lock().await;
    if let Err(e) = f(&mut inner.serial, value) {
        return err(StatusCode::BAD_GATEWAY, format!("{e:#}"));
    }
    drop(inner);
    refresh_and_return(state).await.into_response()
}

#[derive(Deserialize)]
struct MuteReq {
    muted: bool,
}

async fn set_mute(State(state): State<AppState>, Json(req): Json<MuteReq>) -> Response {
    let mut inner = state.lock().await;
    if let Err(e) = inner.serial.set_mute(req.muted) {
        return err(StatusCode::BAD_GATEWAY, format!("{e:#}"));
    }
    drop(inner);
    refresh_and_return(state).await.into_response()
}

#[derive(Deserialize)]
struct InputReq {
    source: InputSource,
}

async fn set_input(State(state): State<AppState>, Json(req): Json<InputReq>) -> Response {
    let mut inner = state.lock().await;
    if let Err(e) = inner.serial.set_input(req.source) {
        return err(StatusCode::BAD_GATEWAY, format!("{e:#}"));
    }
    drop(inner);
    refresh_and_return(state).await.into_response()
}

#[derive(Serialize)]
struct ComInfoResponse {
    config: ComConfig,
    available_ports: Vec<String>,
}

async fn get_com(State(state): State<AppState>) -> Json<ComInfoResponse> {
    let inner = state.lock().await;
    Json(ComInfoResponse {
        config: inner.config.com.clone(),
        available_ports: SerialManager::list_ports(),
    })
}

async fn set_com(State(state): State<AppState>, Json(cfg): Json<ComConfig>) -> Response {
    let mut inner = state.lock().await;
    inner.config.com = cfg.clone();
    inner.serial.update_config(cfg);
    if let Err(e) = inner.config.save() {
        tracing::warn!(?e, "failed to persist config");
    }
    inner.refresh();
    Json(ComInfoResponse {
        config: inner.config.com.clone(),
        available_ports: SerialManager::list_ports(),
    })
    .into_response()
}

#[derive(Serialize)]
struct TestResponse {
    ok: bool,
    error: Option<String>,
}

async fn test_com(State(state): State<AppState>) -> Json<TestResponse> {
    let mut inner = state.lock().await;
    inner.serial.close();
    let res = inner.serial.open();
    match res {
        Ok(()) => Json(TestResponse {
            ok: true,
            error: None,
        }),
        Err(e) => Json(TestResponse {
            ok: false,
            error: Some(format!("{e:#}")),
        }),
    }
}

// ---------------- Static asset serving ----------------

async fn static_handler(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');
    let candidate = if path.is_empty() { "index.html" } else { path };
    if let Some(asset) = FrontendAssets::get(candidate) {
        let mime = mime_guess::from_path(candidate)
            .first_or_octet_stream()
            .to_string();
        return ([(header::CONTENT_TYPE, mime)], asset.data.into_owned()).into_response();
    }
    // SPA fallback: any unknown path serves index.html so client-side routing
    // works.
    if let Some(index) = FrontendAssets::get("index.html") {
        return (
            [(header::CONTENT_TYPE, "text/html".to_string())],
            index.data.into_owned(),
        )
            .into_response();
    }
    (
        StatusCode::NOT_FOUND,
        "frontend bundle missing — run `npm run build` before `cargo build`",
    )
        .into_response()
}
