// Thin fetch wrapper around the embedded Rust HTTP API. The same code path is
// used by the desktop WebView and by mobile browsers, so all requests are
// relative URLs against the page origin.
import type {
  ComConfig,
  ComInfo,
  DeviceStatus,
  InputSource,
  SerialLogEntry,
  ServerInfo,
} from "./types";

// When the page is loaded inside the Tauri WebView the origin is the custom
// `tauri://` (or `http://tauri.localhost` on Windows) protocol; in that case
// requests need to target the embedded server explicitly. In every other case
// (vite dev with proxy, or a phone hitting the LAN IP) the API lives on the
// same origin.
function apiBase(): string {
  if (typeof window === "undefined") return "";
  const { protocol, hostname } = window.location;
  if (protocol === "tauri:" || hostname === "tauri.localhost") {
    return "http://127.0.0.1:8765";
  }
  return "";
}

async function request<T>(
  method: string,
  path: string,
  body?: unknown,
): Promise<T> {
  const res = await fetch(apiBase() + path, {
    method,
    headers: body ? { "Content-Type": "application/json" } : undefined,
    body: body ? JSON.stringify(body) : undefined,
  });
const text = await res.text();

let data: unknown = null;

try {
  data = text ? JSON.parse(text) : null;
} catch {
  data = text;
}

if (!res.ok) {
  const msg =
    typeof data === "object" &&
    data !== null &&
    "error" in data
      ? String((data as { error?: unknown }).error)
      : res.statusText;

  throw new Error(msg);
}

return data as T;
}

export const api = {
  status: () => request<DeviceStatus>("GET", "/api/status"),
  setPower: (on: boolean) => request<DeviceStatus>("POST", "/api/power", { on }),
  setBrightness: (value: number) =>
    request<DeviceStatus>("POST", "/api/brightness", { value }),
  setContrast: (value: number) =>
    request<DeviceStatus>("POST", "/api/contrast", { value }),
  setVolume: (value: number) =>
    request<DeviceStatus>("POST", "/api/volume", { value }),
  setMute: (muted: boolean) =>
    request<DeviceStatus>("POST", "/api/mute", { muted }),
  setInput: (source: InputSource) =>
    request<DeviceStatus>("POST", "/api/input", { source }),

  getCom: () => request<ComInfo>("GET", "/api/com"),
  setCom: (config: ComConfig) => request<ComInfo>("POST", "/api/com", config),
  testCom: () => request<{ ok: boolean; error: string | null }>(
    "POST",
    "/api/com/test",
  ),

  serverInfo: () => request<ServerInfo>("GET", "/api/info"),

  logs: () => request<SerialLogEntry[]>("GET", "/api/logs"),
  clearLogs: () => request<void>("DELETE", "/api/logs"),
};
