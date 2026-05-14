# BSD-E6 RS-232 Protocol

## Link layer

| Setting          | Value           |
|------------------|-----------------|
| Communication    | RS-232          |
| Baud rate        | 115 200 bps     |
| Data bits        | 8               |
| Parity           | None            |
| Stop bits        | 1               |
| Flow control     | None            |

## Frame format

All commands use the same 8-byte structure:

```
B2 00 00 3A 01 <op> <param> <checksum>
```

* `B2 00 00` — preamble.
* `3A` — start of payload (XOR seed).
* `01` — device address.
* `<op>` — operation code (see table).
* `<param>` — operation parameter (0/1, 0..100, or input id).
* `<checksum>` — `0x3A XOR 0x01 XOR op XOR param`.

The display always replies with **one byte**:

* For getters: the requested value (`0/1` for booleans, `0..100` for
  percentages, `0..N` for the input source).
* For setters: typically the echo of the parameter, or `0` on success.
* `0xFF` (`255`) signals failure — invalid parameter, out of range, etc.

## Operation codes

| Op   | Direction | Description       | Parameter        |
|------|-----------|-------------------|------------------|
| `30` | Get       | Power status      | `0x01`           |
| `31` | Get       | Brightness        | `0x01`           |
| `32` | Get       | Contrast          | `0x01`           |
| `33` | Get       | Volume            | `0x01`           |
| `34` | Get       | Mute              | `0x01`           |
| `35` | Get       | Input source      | `0x01`           |
| `40` | Set       | Power             | `0` off / `1` on |
| `41` | Set       | Brightness        | `0..100`         |
| `42` | Set       | Contrast          | `0..100`         |
| `43` | Set       | Volume            | `0..100`         |
| `44` | Set       | Mute              | `0` / `1`        |
| `45` | Set       | Input             | `0` VGA, `1` HDMI, `2` DP, `3` DVI, `4` AV |

> Note: in the BSD-E6 datasheet, getters are written as `op = 0x01` with the
> parameter encoding the actual command (`0x30..0x35`). The framing is
> equivalent. The Rust enum `Op` uses the conventional naming.

## Worked examples

| Action                | Frame (hex)                       |
|-----------------------|-----------------------------------|
| Power on              | `B2 00 00 3A 01 40 01 7A`         |
| Power off             | `B2 00 00 3A 01 40 00 7B`         |
| Brightness 5 %        | `B2 00 00 3A 01 41 05 7F`         |
| Brightness 50 %       | `B2 00 00 3A 01 41 32 48`         |
| Brightness 100 %      | `B2 00 00 3A 01 41 64 1E`         |
| Volume 50 %           | `B2 00 00 3A 01 43 32 4A`         |
| Set input → HDMI      | `B2 00 00 3A 01 45 01 7F`         |

## Computing the checksum

```
checksum = 0x3A XOR 0x01 XOR <op> XOR <param>
```

Examples:

```
0x3A ^ 0x01 ^ 0x41 ^ 0x64  =  0x1E   (Brightness 100)
0x3A ^ 0x01 ^ 0x40 ^ 0x01  =  0x7A   (Power on)
0x3A ^ 0x01 ^ 0x43 ^ 0x32  =  0x4A   (Volume 50)
```

The checksum implementation lives in
`src-tauri/src/protocol.rs::checksum()` and is unit-tested against the
table above.
