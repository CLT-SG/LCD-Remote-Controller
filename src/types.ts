// Shared TypeScript types between the Vue frontend and the Rust HTTP API.
// Keep this in sync with `src-tauri/src/http_server.rs` request/response shapes.

export type InputSource = "vga" | "hdmi" | "dp" | "dvi" | "av";

export interface DeviceStatus {
  power: boolean | null;
  brightness: number | null;
  contrast: number | null;
  volume: number | null;
  muted: boolean | null;
  input: InputSource | null;
  connected: boolean;
  last_error: string | null;
}

export interface ComConfig {
  port: string;
  baud_rate: number;
  data_bits: number;
  stop_bits: number;
  parity: "none" | "odd" | "even";
  flow_control: "none" | "software" | "hardware";
  timeout_ms: number;
  post_open_delay_ms: number;
  inter_command_delay_ms: number;
}

export interface ComInfo {
  config: ComConfig;
  available_ports: string[];
}

export interface ServerInfo {
  hostname: string;
  addresses: string[];
  port: number;
  version: string;
}

export interface SerialLogEntry {
  timestamp: string;
  direction: "TX" | "RX" | "ERR";
  hex: string;
  summary: string;
}

export interface ApiError {
  error: string;
}
