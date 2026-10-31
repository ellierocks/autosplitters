# autosplitters

Cargo workspace of WebAssembly autosplitters for LiveSplit One in OBS on Linux.

## Navigation

- [README](README.md): splitters, build outputs and downloads.
- [lsw1 setup](lsw1/README.md): LEGO Star Wars 1 on Dolphin; supported desktop
  `lsw1/dolphin.asl` and wasm `lsw1/src/lib.rs`.
- [sadx setup](sadx/README.md): Sonic Adventure DX PC; wasm `sadx/src/lib.rs`.
- [Development reference](docs/development.md): module maps, workspace layout,
  adding splitters, test coverage and release procedures/history. Read it for those tasks.

## Domain safeguards

- Keep lsw1's supported ASL and Rust split logic in sync; update parity expectations
  deliberately and document intentional differences.
- GameCube memory is big-endian: read through `FromEndian`.
- On Linux, attaching needs ptrace permission on OBS; see lsw1 setup for “Waiting to attach”.
- Live split validation needs Dolphin, the game and OBS; ask the user to test live.
- sadx imports WASI `clock_time_get` directly: asr's `Instant` exists only on WASI,
  while this workspace targets `wasm32-unknown-unknown`. LiveSplit One links WASI
  for modules importing it.

## Checks (repo root; exact CI commands)

```sh
cargo build --release --locked --workspace
cargo clippy --release --locked --workspace -- -D warnings
cargo fmt --all --check
cargo test --workspace --target x86_64-unknown-linux-gnu --locked
cargo clippy --workspace --target x86_64-unknown-linux-gnu --locked --tests -- -D warnings
```

`.cargo/config.toml` defaults to wasm; plain `cargo test` fails. Tests run on the
explicit host target; the no_std libraries only link for wasm, so host clippy needs
`--tests`. The toolchain is pinned in `rust-toolchain.toml` and bumped by Renovate.
