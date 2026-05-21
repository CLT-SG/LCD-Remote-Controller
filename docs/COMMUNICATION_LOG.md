# Communication Log

The right-hand panel in the dashboard shows every byte sent and received on the RS-232 link. This document explains the log format and how to use it for troubleshooting.

## Log entry format

Each row represents one event:

| Field       | Description                                                               |
|-------------|---------------------------------------------------------------------------|
| `timestamp` | Time of day in `HH:MM:SS`.                                                |
| `direction` | `TX` (host → display), `RX` (display → host), or `ERR` (failure).         |
| `hex`       | Raw bytes as upper-case hex, or `—` when no bytes were transferred.      |
| `summary`   | Human-readable description, e.g. `Get Power`, `0x01`, or `No response`.   |

The buffer is ring-bound to **200 entries**; older rows are discarded automatically.

## Directions

### TX — Transmit

Every command the application sends to the display is logged as `TX`.

Example:

```
14:32:05  TX  B2 00 00 3A 01 01 30 0A  Get Power
```

### RX — Receive

Replies from the display are logged as `RX`.

```
14:32:05  RX  01  0x01
```

A successful reply is usually a single byte:

* `0x00` / `0x01` — boolean state (power, mute).
* `0x00` … `0x64` — percentage value (brightness, contrast, volume).
* `0xFF` — the display signalled failure (bad parameter, out of range, etc.).

### ERR — Error

`ERR` rows appear when:

1. **Timeout** — the display did not reply within `Read Timeout (ms)`.  
   This is the most common symptom when the display is overwhelmed by
   back-to-back commands or needs more time after the port is opened.

   ```
   14:32:06  ERR  —  No response (timeout)
   ```

2. **Serial I/O failure** — the port was unplugged, permission was revoked,
   or the OS reported a hardware error.

   ```
   14:32:07  ERR  —  Error reading reply: Broken pipe
   ```

## Diagnosing timeouts

If you see mostly `TX` and `ERR` rows with very few `RX` entries, try the
following in order:

1. **Check the cable** — confirm the RS-232 adapter is plugged in and the
   correct COM port is selected.
2. **Verify baud rate** — the BSD-E6 default is **115 200 8N1**.
3. **Increase Post-Open Delay** — open the **COM** panel and raise
   *Post-Open Delay (ms)* to `500` or `1000`. Some displays ignore the
   first command if it arrives too quickly after link establishment.
4. **Increase Inter-Command Delay** — raise *Inter-Command Delay (ms)* to
   `200` or `300`. This spaces out the six status-polling commands so the
   display has time to process each frame before the next one arrives.
5. **Check the log timestamps** — with the delays enabled you should see
   roughly `(post_open) + 6 × (inter_command + timeout)` seconds between
   poll cycles instead of rapid-fire `TX`/`ERR` bursts.

## Clearing the log

Click **Clear** above the log panel, or send `DELETE /api/logs` via the REST API.
