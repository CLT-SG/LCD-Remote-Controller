# Remove Auto-Polling to Prevent Command Conflicts

## Overview

This PR removes the automatic status polling mechanism that was causing race conditions when users attempted to control the LCD display. Status refresh is now manual via the Refresh button.

## Problem

Auto-polling status every 5 seconds interfered with user commands. When a user changed brightness, power, volume, or other settings, the background poll could trigger simultaneously, causing:

- Commands to be skipped or ignored
- Race conditions on the serial port
- Unpredictable display behavior
- User frustration when controls did not respond as expected

The `setInterval(refreshAll, 5000)` in `App.vue` was the root cause. The poll and user action would collide on the RS-232 link, and the display could only process one command at a time.

## Solution

### Remove auto-polling from App.vue

- Removed `pollHandle` variable that stored the interval ID
- Removed `setInterval(refreshAll, 5000)` call from `onMounted()`
- Removed `onUnmounted()` cleanup hook (no longer needed)
- Removed unused `onUnmounted` import from Vue
- Updated component comment to reflect new manual refresh behavior

### Improve Refresh button behavior

- Changed Refresh button from calling `loadStatus()` to `refreshAll()`
- This ensures both status and communication logs are updated on manual refresh

### Add documentation

- Created `docs/STATUS_REFRESH.md` explaining the manual refresh behavior, rationale, and best practices
- Updated `docs/USAGE.md` with a note about manual refresh and link to new documentation
- Updated `docs/README.md` to include the new documentation file in the index
- Added v1.0.8 entry to `CHANGELOG.md`

## Files changed

| File | Change |
|------|--------|
| `src/App.vue` | Removed auto-polling logic (pollHandle, setInterval, onUnmounted); updated Refresh button to call refreshAll(); updated comment |
| `docs/STATUS_REFRESH.md` | New documentation explaining manual refresh behavior and rationale |
| `docs/USAGE.md` | Added manual refresh note with link to new documentation |
| `docs/README.md` | Added STATUS_REFRESH.md to documentation index |
| `CHANGELOG.md` | Added v1.0.8 entry documenting the change |

## Verification

- `tsc --noEmit` passes with zero type errors
- Application loads and displays status on mount
- Refresh button updates both status and logs
- User commands (brightness, power, volume, etc.) execute without interference

## Deployment notes

No configuration changes required. The change is purely frontend behavior. Users should click the Refresh button to sync the UI with the display after physical adjustments or when using multiple controllers.
