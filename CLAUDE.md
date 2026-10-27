# lsw1-wasm

LEGO Star Wars 1 (GameCube, under Dolphin) autosplitter for LiveSplit One's auto-splitting runtime, compiled to
WebAssembly so it runs in the OBS LiveSplit One plugin on Linux. It is a port of `dolphin.asl`, the original
LiveSplit script kept in the repo as the reference for addresses and split logic.

## Layout

All the code is in `src/lib.rs`:
- Memory addresses (`STATUS`, `GAMESTATE`, `CUTSCENE`, `DOOR`) in emulated GameCube RAM.
- `Settings` (shown in the LiveSplit One UI).
- The door lists for room splits.
- `find_ram`/`attach`, which locate Dolphin's MEM1 in the process.
- `should_split`, the split decision: old state compared with new state.

Built on the `asr` crate, pinned to a git rev.

## Commands (what CI runs)

```sh
cargo build --release --locked      # .cargo/config.toml defaults the target to wasm32-unknown-unknown
cargo clippy --release --locked -- -D warnings
cargo fmt --check
cargo test --target x86_64-unknown-linux-gnu --locked                     # tests run on the host
cargo clippy --target x86_64-unknown-linux-gnu --locked --tests -- -D warnings
```

The Rust toolchain is pinned in `rust-toolchain.toml`; Renovate bumps it.

Output: `target/wasm32-unknown-unknown/release/lsw1_wasm.wasm`. A plain `cargo test` fails, because the default
target is wasm. The no_std lib only links for wasm, so host builds need `--tests`.

The tests check that `ENTER_DOORS`/`LEAVE_DOORS` still match `dolphin.asl`, and check `should_split`'s settings
and edge cases. When split logic changes, change the ASL parity expectations deliberately.

## Gotchas

- Behaviour must match `dolphin.asl`. When changing split logic, check the matching ASL lines and say where they
  differ on purpose.
- GameCube memory is big-endian, so reads go through `FromEndian`.
- On Linux the splitter needs ptrace permission on the OBS binary (see README). "Waiting to attach" with no
  error is usually that.
- A real run needs Dolphin, the game and OBS, so ask the user to test splits live.
