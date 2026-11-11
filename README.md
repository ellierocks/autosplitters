# autosplitters

My autosplitters for LiveSplit One's auto-splitting runtime (WebAssembly), for use in the OBS LiveSplit One plugin on Linux.

| Splitter | Game | Desktop ASL | Build output |
|---|---|---|---|
| [`lsw1`](lsw1/README.md) | LEGO Star Wars: The Video Game (GameCube, Dolphin) | [`lsw1/dolphin.asl`](lsw1/dolphin.asl) | `target/wasm32-unknown-unknown/release/lsw1_wasm.wasm` |
| [`lsw1-pc`](lsw1-pc/README.md) | LEGO Star Wars: The Video Game (Japanese PC 1.0, experimental) | [Joeyfordayz / Elle](lsw1-pc/joey.asl) (patch 2 reference) | `target/wasm32-unknown-unknown/release/lsw1_pc.wasm` |
| [`sadx`](sadx/README.md) | Sonic Adventure DX (PC, Steam) | [Sora-yx's ASL](https://github.com/Sora-yx/autosplitters) (external) | `target/wasm32-unknown-unknown/release/sadx_livesplit_one.wasm` |

lsw1's ASL and WASM are both supported and kept in sync; sadx's WASM is a port of the external ASL.

`cargo build --release` at the root builds every splitter for `wasm32-unknown-unknown`.

Releases (`lsw1-vX.Y.Z`, `lsw1-pc-vX.Y.Z`, `sadx-vX.Y.Z`) carry the built `.wasm`. lsw1 also carries `dolphin.asl`; lsw1-pc's `joey.asl` is its upstream reference for patch 2, not a JP desktop splitter: [GitHub releases](https://github.com/ellierocks/autosplitters/releases).
