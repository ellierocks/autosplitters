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
```

Output: `target/wasm32-unknown-unknown/release/lsw1_wasm.wasm`. `cargo test` doesn't work as-is, because the
default target is wasm. Host-side tests need `--target x86_64-unknown-linux-gnu`.

## Gotchas

- Behaviour must match `dolphin.asl`. When changing split logic, check the matching ASL lines and say where they
  differ on purpose.
- GameCube memory is big-endian, so reads go through `FromEndian`.
- On Linux the splitter needs ptrace permission on the OBS binary (see README). "Waiting to attach" with no
  error is usually that.
- A real run needs Dolphin, the game and OBS, so ask the user to test splits live.
