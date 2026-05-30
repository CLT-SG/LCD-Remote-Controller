# Status Refresh Behavior

This document explains how device status is refreshed in the CLT LCD Remote
Controller application.

## Overview

The application uses **manual refresh** rather than automatic polling to fetch
the current device status. This design decision prevents conflicts between
status reads and user commands.

## Why Manual Refresh?

### The Problem with Auto-Polling

When auto-polling was enabled, the application would periodically query the
LCD display for its current status (power, brightness, volume, etc.). This
caused issues when a user attempted to change a setting:

1. User drags brightness slider → application sends brightness command
2. Auto-poll triggers simultaneously → application sends status query
3. Commands collide on the serial port → unpredictable behavior
4. Display may ignore one command or return incorrect response

### The Solution

By removing auto-polling, the application ensures:

- **No command conflicts** — user actions are never interrupted by background
  status reads
- **Predictable behavior** — each command completes fully before the next
- **Reliable serial communication** — the RS-232 link handles one
  request/response cycle at a time

## How to Refresh Status

### Manual Refresh Button

Click the **Refresh** button (↻) in the header to fetch the latest status from
the display. This updates:

- Power state
- Brightness level
- Contrast level
- Volume level
- Mute state
- Input source
- Communication logs

### Automatic Refresh After Commands

When you change a setting (e.g., brightness, power), the application
automatically receives the updated status in the command response. This means
you typically don't need to manually refresh after making changes.

## Implementation Details

The refresh logic is implemented in `src/App.vue`:

```typescript
async function refreshAll() {
  await Promise.all([loadStatus(), loadLogs()]);
}
```

This function is called:

1. **On mount** — when the application first loads
2. **On button click** — when the user clicks the Refresh button
3. **After each command** — via the `withCall()` wrapper which updates status
   from the command response

## Best Practices

- **Refresh after physical changes** — if someone adjusts the display using
  physical buttons or another controller, click Refresh to sync the UI
- **Refresh if unsure** — when in doubt about the current state, click Refresh
- **No need to spam** — one refresh is enough; rapid clicking adds no benefit

## Related Documentation

- [UI Guide](UI.md) — describes the Refresh button location and header controls
- [User Guide](USAGE.md) — covers general usage and troubleshooting
- [Architecture](ARCHITECTURE.md) — explains the overall application structure
