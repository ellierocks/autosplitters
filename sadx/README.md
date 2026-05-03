# Sonic Adventure DX LiveSplit One Autosplitter

Version: `0.1.0`

LiveSplit One compatible alternative to the desktop LiveSplit ASL currently listed in `LiveSplit.AutoSplitters.xml`:

`https://raw.githubusercontent.com/Sora-yx/autosplitters/master/Sonic%20Adventure%20DX-%20Director-s%20Cut/LiveSplit.SADX.asl`

This port targets LiveSplit One's Auto Splitting Runtime (`asr`) instead of ASL. It preserves the current desktop script's core behavior:

- Auto start from the story select fade-to-black
- Optional auto reset on main menu
- Frame-counted in-game time and load removal
- Emblem, boss, stage-entry, boss-entry, upgrade, character-entry, and miscellaneous splits
- Steam `sonic.exe` and `Sonic Adventure DX.exe` process names

## Install

Download `sadx_livesplit_one.wasm` from the release assets and add it as the autosplitter in LiveSplit One.

## Build

Requires the Rust `wasm32-wasip1` target:

```sh
rustup target add wasm32-wasip1
```

Build the release WebAssembly module with:

```sh
cargo build --release --target wasm32-wasip1
```

The output is:

`target/wasm32-wasip1/release/sadx_livesplit_one.wasm`

## Release Checklist

- Run `cargo fmt --check`.
- Run `cargo check --release --target wasm32-wasip1`.
- Run `cargo clippy --release --target wasm32-wasip1 -- -D warnings`.
- Run `cargo test --target wasm32-wasip1 --no-run`.
- Run `cargo build --release --target wasm32-wasip1`.
- Attach `target/wasm32-wasip1/release/sadx_livesplit_one.wasm` to the GitHub release.
