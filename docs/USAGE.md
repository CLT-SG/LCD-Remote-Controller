# User Guide

## 1. Connect the cable

Connect the BSD-E6 LCD's RS-232 port to the PC running this application
using a USB-to-Serial adapter if needed. Note the COM port name:

* Windows: `COM3`, `COM4`, …
* Linux: `/dev/ttyUSB0`, `/dev/ttyS0`, …
* macOS: `/dev/tty.usbserial-XXXX`

> Linux only: add your user to the `dialout` group so non-root processes
> can open the port: `sudo usermod -aG dialout $USER`, then log out and
> back in.

## 2. Launch the application

Double-click the bundled installer (or run `npm run tauri:dev` from a
clone). The dashboard opens.

## 3. Configure the COM port

1. Click **COM** in the top-right of the header.
2. Pick your port from the dropdown (use the ↻ button to rescan).
3. Verify the baud rate is **115 200**, data bits **8**, parity **none**,
   stop bits **1**.
4. Click **Test Connection**. A green banner confirms the port opens.
5. Click **Save**.

The settings are persisted to disk and reloaded on every launch.

### Timing settings

If the display does not reply reliably (all RX entries show *No response (timeout)*), adjust the two delay fields:

* **Post-Open Delay (ms)** — pause after the COM port is opened before the
  first command. Default is `200 ms`. Increase to `500–1000 ms` if the display
  needs more time to initialise the RS-232 link.
* **Inter-Command Delay (ms)** — minimum gap between consecutive commands.
  Default is `100 ms`. Increase to `200–300 ms` if the display is overwhelmed
  by rapid consecutive commands.

## 4. Control the display

* **Power** — toggle on / off.
* **Input Source** — select VGA, HDMI, DP, DVI or AV.
* **Volume** — drag the slider; releases the slider sends one command.
  Use the **Mute** toggle to silence without changing the level.
* **Brightness** and **Contrast** — same UX as Volume.

A connection indicator in the header turns red whenever a command fails
or the port is closed; the message bar below shows the underlying error.

Click the **Refresh** button (↻) in the header to manually sync the UI with
the display's current state. This is useful after physical adjustments or
when using multiple controllers. See [Status Refresh](STATUS_REFRESH.md) for
details.

## 5. Use a phone

While the desktop application is running:

1. Make sure the phone is on the **same Wi-Fi network** as the PC.
2. On the desktop, open the **COM** panel — the *Mobile Access* section
   lists every URL the phone can use, e.g. `http://192.168.1.42:8765`.
3. Open that URL on the phone's browser. The exact same dashboard
   loads and controls the same display.

You can have multiple phones connected at the same time; they all share
the same live state because the desktop is the single source of truth.

## Troubleshooting

| Symptom                                  | Likely cause                                       |
|------------------------------------------|----------------------------------------------------|
| `Permission denied` on `/dev/ttyUSB0`    | Add user to `dialout` group (Linux).               |
| `No such file or directory`              | Adapter not plugged in / wrong port name.          |
| `Disconnected` even though port opened   | Wrong baud rate or cable wiring (check pinout).    |
| Phone cannot reach the desktop URL       | Different Wi-Fi network, or firewall blocks 8765.  |
| Slider jumps back to old value           | Display rejected the command (`0xFF`); check power.|

To see verbose logs run with `RUST_LOG=debug`:

```bash
RUST_LOG=debug npm run tauri:dev
```
