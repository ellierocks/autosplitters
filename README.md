# autosplitters

My autosplitters for LiveSplit One's auto-splitting runtime (WebAssembly), for use in the OBS LiveSplit One plugin on Linux.

| Splitter | Game | Build output |
|---|---|---|
| [`lsw1`](lsw1/README.md) | LEGO Star Wars: The Video Game (GameCube, Dolphin) | `target/wasm32-unknown-unknown/release/lsw1_wasm.wasm` |
| [`sadx`](sadx/README.md) | Sonic Adventure DX (PC, Steam) | `target/wasm32-unknown-unknown/release/sadx_livesplit_one.wasm` |

`cargo build --release` at the root builds every splitter for `wasm32-unknown-unknown`.

Releases (`lsw1-vX.Y.Z`, `sadx-vX.Y.Z`) carry the built `.wasm`: [GitHub releases](https://github.com/ellierocks/autosplitters/releases).
