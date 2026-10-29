# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-10-29

### Changed

- Moved into the `autosplitters` monorepo (`sadx/`); releases are now `sadx-vX.Y.Z` on
  [ellierocks/autosplitters](https://github.com/ellierocks/autosplitters/releases).
- Built for `wasm32-unknown-unknown` with the workspace's pinned toolchain and `asr` revision. Pre-rendered movie
  timing calls WASI's clock directly instead of asr's WASI-only `Instant`; the split logic is unchanged.

## [0.1.0] - 2026-05-03

- Initial LiveSplit One WebAssembly release for Sonic Adventure DX (from the old `sadx-livesplit-one` repo).
- Ports the core start, reset, split, and load-removal flow from the desktop ASL autosplitter.
- Supports Steam `sonic.exe` and `Sonic Adventure DX.exe` process names.
- Verified in game across two tested memory address layouts.
