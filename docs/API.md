# HTTP API Reference

Base URL: `http://<host>:8765`

All endpoints respond with JSON. Errors have shape `{ "error": "<message>" }`.

## Status

### `GET /api/health`
Returns `ok`. Used as a readiness probe.

### `GET /api/status`
Polls the device and returns a `DeviceSnapshot`:

```json
{
  "power": true,
  "brightness": 80,
  "contrast": 50,
  "volume": 30,
  "muted": false,
  "input": "hdmi",
  "connected": true,
  "last_error": null
}
```

Any field may be `null` if the corresponding getter failed (the device
returned `0xFF`) or no port is configured.

### `GET /api/info`
Returns hostname, LAN IPv4 addresses and the listening port. Used by the
UI to suggest mobile-access URLs.

## Device control

Each setter returns the refreshed `DeviceSnapshot` so the UI can update
without an additional roundtrip.

| Method | Path             | Body                              | Notes                      |
|--------|------------------|-----------------------------------|----------------------------|
| POST   | `/api/power`     | `{ "on": true }`                  |                            |
| POST   | `/api/brightness`| `{ "value": 0..100 }`             | Validated server-side      |
| POST   | `/api/contrast`  | `{ "value": 0..100 }`             |                            |
| POST   | `/api/volume`    | `{ "value": 0..100 }`             |                            |
| POST   | `/api/mute`      | `{ "muted": true }`               |                            |
| POST   | `/api/input`     | `{ "source": "vga\|hdmi\|dp\|dvi\|av" }` |                       |

## COM (serial) configuration

### `GET /api/com`
```json
{
  "config": {
    "port": "/dev/ttyUSB0",
    "baud_rate": 115200,
    "data_bits": 8,
    "stop_bits": 1,
    "parity": "none",
    "flow_control": "none",
    "timeout_ms": 500,
    "post_open_delay_ms": 200,
    "inter_command_delay_ms": 100
  },
  "available_ports": ["/dev/ttyUSB0", "/dev/ttyS0"]
}
```

### `POST /api/com`
Persists a new configuration. Body is the same `config` shape as above.
Returns the updated `ComInfo` object.

### `POST /api/com/test`
Attempts to open the configured port without sending any frame. Returns:
```json
{ "ok": true,  "error": null }
{ "ok": false, "error": "opening /dev/ttyUSB0: Permission denied" }
```

## Communication log

### `GET /api/logs`
Returns the most recent 200 serial traffic entries:

```json
[
  {
    "timestamp": "14:32:05",
    "direction": "TX",
    "hex": "B2 00 00 3A 01 01 30 0A",
    "summary": "Get Power"
  },
  {
    "timestamp": "14:32:05",
    "direction": "RX",
    "hex": "01",
    "summary": "0x01"
  },
  {
    "timestamp": "14:32:06",
    "direction": "ERR",
    "hex": "—",
    "summary": "No response (timeout)"
  }
]
```

**Directions:**
- `TX` — command sent to the display.
- `RX` — reply received from the display.
- `ERR` — timeout or serial I/O error. These entries help diagnose timing or wiring issues without scrolling through empty RX rows.

### `DELETE /api/logs`
Clears the in-memory log buffer. Returns `ok`.

## Error codes

| Status | Meaning                                          |
|--------|--------------------------------------------------|
| 400    | Invalid request (e.g. value out of range).       |
| 502    | The serial transaction failed (`Bad Gateway`).   |
| 404    | Asset not found (only for static assets).        |

CORS is permissive by default so the UI can be loaded from any origin on
the LAN. Tighten this in `http_server.rs` if you require stricter access
control.
