# rusc-android fork policy

This fork tracks `Drewol/kson-rs` while developing a first-class Android version of the game (`game/`).

## Remotes and ownership

- `origin`: `Ganer228/rusc-android`
- `upstream`: `Drewol/kson-rs`
- upstream default branch: `master`
- fork default branch: `master`

The fork relationship is intentionally preserved. Do not detach the repository from upstream by recreating history.

## Update policy

Upstream changes are never merged blindly into the working Android branch.

1. Fetch upstream.
2. Create a temporary branch named `sync/upstream-YYYY-MM-DD` from the current fork `master`.
3. Merge or rebase the selected upstream revision there.
4. Build all existing targets, including the Android APK.
5. Run Android smoke tests on a real device.
6. Resolve Android-specific regressions on the sync branch.
7. Merge the sync branch only after it is known-good.

If an upstream update breaks Android, the current working Android version remains untouched until the sync branch is fixed.

## Patch isolation

Prefer Android-specific code under dedicated modules and keep edits to shared upstream files small. Avoid large Android-only rewrites inside `game_main.rs`, `songselect.rs`, or other frequently changed upstream files.

Commit prefixes:

- `upstream:` upstream synchronization only
- `android:` Android runtime, input, storage, lifecycle, UI and defaults
- `custom:` fork-specific presentation, skins and optional features

## First verified Android baseline

Baseline upstream commit: `b5f19ebf7a9f70bf0d90e493728a79be918a3200`.

Verified on a real Android device:

- APK installs and starts.
- Default Lua skin renders.
- Main menu works.
- Settings screen opens.
- Nautica song list and jacket loading work.
- Song transition can be entered.
- Touch events are received.

Known Android problems at this baseline:

- touch hitboxes are an invisible full-screen grid and do not match the controller guide;
- chart start/download/gameplay transition needs diagnostics;
- settings are desktop-oriented and expose unsuitable defaults such as window mode, high target FPS and desktop input options;
- Android currently forces the Nautica song provider instead of exposing a proper mobile local-library flow.

## First milestone

`Mobile Foundation`:

1. Replace the hidden touch grid with a usable, visible mobile control layout.
2. Add chart-loading diagnostics and make a Nautica chart reliably reach gameplay.
3. Add sane Android defaults and hide or replace desktop-only settings.
4. Preserve upstream compatibility and keep mobile patches isolated.

Do not begin cosmetic SDVX/character skin work until the basic Android gameplay path is reliable.
