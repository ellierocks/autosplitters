# sadx-livesplit-one

![sadx-livesplit-one example](assets/example.png)

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

Download `sadx_livesplit_one.wasm` from the latest `sadx-v*` release of
[ellierocks/autosplitters](https://github.com/ellierocks/autosplitters/releases) and add it as the autosplitter in
LiveSplit One. Versions up to 0.1.0 were released from the old `sadx-livesplit-one` repository.

## Build

From the repository root (the workspace defaults the target to `wasm32-unknown-unknown`):

```sh
cargo build --release -p sadx-livesplit-one
```

The output is `target/wasm32-unknown-unknown/release/sadx_livesplit_one.wasm`.

The splitter times pre-rendered movies with a wall clock. asr only provides one on the WASI target, so `src/lib.rs`
imports WASI's `clock_time_get` directly; LiveSplit One's runtime provides it to any module that imports it (and logs
that the splitter uses WASI).

## Releases

release-plz cuts `sadx-vX.Y.Z` releases from Conventional Commits, and the release workflow attaches the `.wasm`.
