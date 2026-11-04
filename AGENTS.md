# autosplitters

Cargo workspace of WebAssembly autosplitters for LiveSplit One in OBS on Linux.

## Domain safeguards

- Keep lsw1's supported ASL and Rust split logic in sync; update parity expectations
  deliberately and document intentional differences.
- GameCube memory is big-endian: read through `FromEndian`.
- On Linux, attaching needs ptrace permission on OBS; see lsw1 setup for “Waiting to attach”.
- Live split validation needs Dolphin, the game and OBS; ask the user to test live.
- sadx imports WASI `clock_time_get` directly: asr's `Instant` exists only on WASI,
  while this workspace targets `wasm32-unknown-unknown`. LiveSplit One links WASI
  for modules importing it.

## Checks

Run the [CI commands](docs/development.md#commands-what-ci-runs-from-the-repo-root)
from the repo root.

`.cargo/config.toml` defaults to wasm; tests require the explicit
`--target x86_64-unknown-linux-gnu`. The no_std libraries only link for wasm, so
host clippy needs `--tests`.
