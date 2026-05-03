# sadx-livesplit-one

![sadx-livesplit-one example](assets/example.png)

Version: `0.1.0`

LiveSplit One compatible alternative to Sora-yx's desktop LiveSplit ASL autosplitter from [`Sora-yx/autosplitters`](https://github.com/Sora-yx/autosplitters).

Credit to Sora-yx and the earlier Sonic Adventure DX autosplitter contributors whose ASL logic this port is based on:

- [TurtleMan64](https://github.com/TurtleMan64/)
- [Ngolinvaux](https://github.com/Ngolinvaux)
- [IDGeek121](https://github.com/IDGeek121)
- [Sora](https://github.com/Sora-yx)
- [skoobmaster](https://github.com/skewbmaster/)

Credit also to [Refragg](https://github.com/Refragg), who previously created an independent LiveSplit One / WASM port at [`Refragg/sadx-wasm-auto-splitter`](https://github.com/Refragg/sadx-wasm-auto-splitter). This project was created separately before I discovered that earlier port.

This port targets LiveSplit One's Auto Splitting Runtime (`asr`) instead of ASL. It preserves the current desktop script's core behavior:

- Auto start from the story select fade-to-black
- Optional auto reset on main menu
- Frame-counted in-game time and load removal
- Emblem, boss, stage-entry, boss-entry, upgrade, character-entry, and miscellaneous splits
- `sonic.exe` and `Sonic Adventure DX.exe` process names

## Install

Download `sadx_livesplit_one.wasm` from the release assets and add it as the autosplitter in LiveSplit One.

## Build

Requires the Rust `wasm32-wasip1` target:

```sh
rustup target add wasm32-wasip1
```

Build the release WebAssembly module with:

```sh
cargo build --release
```

The repository's Cargo config targets `wasm32-wasip1` by default, so no extra target flag is needed. You can also use the shorter alias:

```sh
cargo b
```

The output is:

`target/wasm32-wasip1/release/sadx_livesplit_one.wasm`

## Releases

Every push to `main` builds `target/wasm32-wasip1/release/sadx_livesplit_one.wasm` and publishes it to the GitHub release whose tag matches the package version in `Cargo.toml`.

For example, version `0.1.0` publishes to tag/release `0.1.0`. If more commits are pushed without changing the version, the workflow moves that version tag to the latest `main` commit and replaces the release asset.

## Manual Release Checklist

- Run `cargo fmt --check`.
- Run `cargo check --release`.
- Run `cargo clippy --release -- -D warnings`.
- Run `cargo test --no-run`.
- Run `cargo build --release`.
- Attach `target/wasm32-wasip1/release/sadx_livesplit_one.wasm` to the GitHub release.
