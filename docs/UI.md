# UI Guide

This document describes the visual structure of the **CLT LCD Remote
Controller** dashboard and the conventions used by the Vue components.
It is intended for future contributors who need to extend or restyle
the interface.

## Layout overview

The dashboard is a single-page Vue 3 application rendered by
`src/App.vue`. From top to bottom it consists of:

```
┌──────────────────────────────────────────────────────────────┐
│  Header                                                       │
│  ┌─────┐  CLT LCD Remote Controller                          │
│  │LOGO │  BPLRT-BSD-E6 series · RS-232 · Connected      [COM][↻][☀]│
│  └─────┘                                                      │
├──────────────────────────────────────────────────────────────┤
│  (optional)  Communication Configuration card                 │
├──────────────────────────────────────────────────────────────┤
│  Row 1   ┌─────────┐ ┌──────────────┐ ┌──────────────────┐   │
│          │  Power  │ │ Input Source │ │     Volume       │   │
│          └─────────┘ └──────────────┘ └──────────────────┘   │
│                                                               │
│  Row 2   ┌────────────────────┐ ┌────────────────────┐       │
│          │     Brightness     │ │      Contrast      │       │
│          └────────────────────┘ └────────────────────┘       │
│                                                               │
│  Row 3   ┌────────────────────────────────────────────┐      │
│          │  Mobile Access (URLs for phones on LAN)    │      │
│          └────────────────────────────────────────────┘      │
├──────────────────────────────────────────────────────────────┤
│  Footer:  v0.1.0 · CLT LCD Remote Controller                  │
└──────────────────────────────────────────────────────────────┘
```

The grid breakpoints are:

* `grid-cols-1` on small screens (every card stacks vertically).
* `md:grid-cols-3` for **Row 1** so Power, Input Source and Volume
  share a single row from the `md` breakpoint upwards.
* `md:grid-cols-2` for **Row 2** placing Brightness and Contrast
  side-by-side from the `md` breakpoint upwards.

## Header controls

| Control       | Component / file                  | Purpose                                       |
|---------------|-----------------------------------|-----------------------------------------------|
| Logo          | `src/components/CltLogo.vue`      | Inline SVG brand mark.                        |
| `COM` toggle  | `src/App.vue`                     | Show / hide the COM configuration card.       |
| Refresh `↻`   | `src/App.vue`                     | Manually re-fetch device status.              |
| Theme toggle  | `src/App.vue` + `useTheme.ts`     | Switch between dark and light themes.         |

## Theming

Theme management is centralised in `src/composables/useTheme.ts`.

* The chosen theme is applied by adding either `light` or `dark` as a
  class on the root `<html>` element.
* The user's choice is persisted in `localStorage` under the key
  `clt.theme`. On first run we fall back to the OS-level
  `prefers-color-scheme` preference.
* The dark theme is the historical default and is encoded directly via
  Tailwind utility classes (`text-white`, `bg-white/5`, …).
* For the light theme we override those utilities under the
  `html.light` selector inside `src/style.css`. This avoids editing
  every component while still producing a faithful light rendering.

To add a new colour utility that must respond to the theme:

1. Use the existing dark-friendly Tailwind class in the component.
2. Add a matching `html.light .your-class { … }` override in
   `src/style.css`.

## Components

| File                                       | Responsibility                                |
|--------------------------------------------|-----------------------------------------------|
| `src/components/CltLogo.vue`               | Brand mark (inline SVG).                      |
| `src/components/DashboardCard.vue`         | Card shell with title / subtitle.             |
| `src/components/PowerToggle.vue`           | Fluent toggle switch (power / mute).          |
| `src/components/InputSourceSelect.vue`     | Source picker (VGA / HDMI / DP / DVI / AV).   |
| `src/components/FluentSlider.vue`          | Slider with drag-tooltip (volume / bright …). |
| `src/components/ComConfigPanel.vue`        | Serial-link configuration form.               |
| `src/composables/useTheme.ts`              | Theme state, persistence and DOM application. |

## Accessibility

* All icon-only buttons declare `aria-label` and `title`.
* The theme toggle, refresh and COM toggle are reachable by keyboard.
* `PowerToggle` uses `role="switch"` and `aria-checked`.
* The slider keeps a native `<input type="range">` underneath the
  custom visuals to retain keyboard, screen-reader and pointer
  support.
