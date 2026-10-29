# autosplitters

My autosplitters for LiveSplit One's auto-splitting runtime (WebAssembly), for use in the OBS LiveSplit One plugin on Linux.

| Splitter | Game | Desktop ASL | Build output |
|---|---|---|---|
| [`lsw1`](lsw1/README.md) | LEGO Star Wars: The Video Game (GameCube, Dolphin) | [`lsw1/dolphin.asl`](lsw1/dolphin.asl) | `target/wasm32-unknown-unknown/release/lsw1_wasm.wasm` |
| [`sadx`](sadx/README.md) | Sonic Adventure DX (PC, Steam) | [Sora-yx's ASL](https://github.com/Sora-yx/autosplitters) (external) | `target/wasm32-unknown-unknown/release/sadx_livesplit_one.wasm` |

lsw1's ASL and WASM are both supported and kept in sync; sadx's WASM is a port of the external ASL.

`cargo build --release` at the root builds every splitter for `wasm32-unknown-unknown`.

Releases (`lsw1-vX.Y.Z`, `sadx-vX.Y.Z`) carry the built `.wasm`, and lsw1's also carries `dolphin.asl`: [GitHub releases](https://github.com/ellierocks/autosplitters/releases).
