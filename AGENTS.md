# autosplitters

Monorepo for my autosplitters. Each splitter is a Cargo workspace member that compiles to WebAssembly for LiveSplit
One's auto-splitting runtime, so it runs in the OBS LiveSplit One plugin on Linux.

- `lsw1/` (crate `lsw1_wasm`): LEGO Star Wars 1 (GameCube, under Dolphin). A port of `dolphin.asl`, the original
  LiveSplit script kept in `lsw1/` as the reference for addresses and split logic.

## Layout

One `Cargo.lock`, `target/`, `rust-toolchain.toml`, release `[profile]` and `.cargo/config.toml` (defaults the build
target to `wasm32-unknown-unknown`) at the root. Each splitter has its own dir with `Cargo.toml`, `README.md`,
`CHANGELOG.md` and `src/`. To add one: a new member in the root `Cargo.toml`, plus a `[[package]]` entry in
`release-plz.toml` if its tags shouldn't be `<crate name>-v<version>`.

### lsw1 (all in `lsw1/src/lib.rs`)
- Memory addresses (`STATUS`, `GAMESTATE`, `CUTSCENE`, `DOOR`) in emulated GameCube RAM.
- `Settings` (shown in the LiveSplit One UI).
- The door lists for room splits.
- `find_ram`/`attach`, which locate Dolphin's MEM1 in the process.
- `should_split`, the split decision: old state compared with new state.

Built on the `asr` crate, pinned to a git rev.

## Commands (what CI runs, from the repo root)

```sh
cargo build --release --locked --workspace     # .cargo/config.toml defaults the target to wasm32-unknown-unknown
cargo clippy --release --locked --workspace -- -D warnings
cargo fmt --all --check
cargo test --workspace --target x86_64-unknown-linux-gnu --locked                     # tests run on the host
cargo clippy --workspace --target x86_64-unknown-linux-gnu --locked --tests -- -D warnings
```

The Rust toolchain is pinned in `rust-toolchain.toml`; Renovate bumps it.

Output: `target/wasm32-unknown-unknown/release/lsw1_wasm.wasm` (the path is unchanged from before the monorepo). A
plain `cargo test` fails, because the default target is wasm. The no_std lib only links for wasm, so host builds need
`--tests`.

The lsw1 tests check that `ENTER_DOORS`/`LEAVE_DOORS` still match `lsw1/dolphin.asl`, and check `should_split`'s
settings and edge cases. When split logic changes, change the ASL parity expectations deliberately.

## Release

release-plz cuts a release per splitter from Conventional Commits. lsw1's tags are `lsw1-vX.Y.Z` (older `vX.Y.Z` tags
from when this repo was `lsw1-wasm` are kept, with `lsw1-v` copies), and its changelog is `lsw1/CHANGELOG.md`.

## Gotchas

- lsw1 behaviour must match `lsw1/dolphin.asl`. When changing split logic, check the matching ASL lines and say where
  they differ on purpose.
- GameCube memory is big-endian, so reads go through `FromEndian`.
- On Linux the splitter needs ptrace permission on the OBS binary (see `lsw1/README.md`). "Waiting to attach" with no
  error is usually that.
- A real run needs Dolphin, the game and OBS, so ask the user to test splits live.
