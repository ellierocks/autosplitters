# LEGO Star Wars 1 (GameCube) autosplitter

Two supported autosplitters for LEGO Star Wars: The Video Game on GameCube under Dolphin, kept in sync:

- `dolphin.asl` — the original LiveSplit ASL script, for desktop LiveSplit.
- `src/lib.rs` — the LiveSplit One port (WebAssembly), for the OBS LiveSplit One plugin on Linux.

Both read the same memory addresses and make the same split decisions, so a run splits identically either way. Split
logic changes go into both.

## Install

Desktop LiveSplit: add [`dolphin.asl`](dolphin.asl) as an auto splitter. The script loads the emulator helper as
`Components/emu-help-v2.dll`, so that file has to be in LiveSplit's `Components` folder.

LiveSplit One (including the OBS plugin): build the `.wasm` below, or download it from the latest `lsw1-v*` release, and
load it in the auto splitter settings.

## Settings

| Setting | Default | Meaning |
| --- | --- | --- |
| Start on New Game | on | Start the timer when a new game begins |
| End on Darth Maul Outro (Any%) | on | Final split for Any% |
| Split on every room (Any%) | off | Split on the listed door transitions |
| End on Darth Vader Outro (All Minikits) | off | Final split for All Minikits |
| End on A New Hope Outro (100% NSR) | off | Final split for 100% NSR |

The timer also splits when `status` goes 0 to 1, and resets when `gamestate` is 124. One exception: podrace raises
`status` at each of the two crossings of the third lap door, so those rises are ignored while the door reads `door_A`
or `door_B`, and on the tick where it clears from one of those to nothing.

## Build

Requires Rust with the `wasm32-unknown-unknown` target:

```
rustup target add wasm32-unknown-unknown
cargo build --release -p lsw1_wasm
```

From the repo root, the output is `target/wasm32-unknown-unknown/release/lsw1_wasm.wasm`.

## Linux: allow OBS to read Dolphin's memory

The splitter reads Dolphin's emulated RAM from another process. With the default `kernel.yama.ptrace_scope=1`, only a process's ancestors can do that, so the splitter will sit waiting to attach, with no error, while the settings still show.

The plugin is a shared library loaded into OBS, so the permission has to go on the OBS binary. Capabilities on the plugin `.so` do nothing.

Option 1: give OBS the capability (narrowest):

```
sudo setcap cap_sys_ptrace=ep /usr/bin/obs
```

Restart OBS afterwards. A package update to `obs-studio` removes the capability, so run the command again after updating. To undo it:

```
sudo setcap -r /usr/bin/obs
```

Option 2: relax ptrace for all your processes. This is system-wide and weaker security-wise:

```
sudo sysctl kernel.yama.ptrace_scope=0
```

This lasts until reboot. To keep it, put `kernel.yama.ptrace_scope = 0` in a file under `/etc/sysctl.d/`.

These steps assume a native OBS install. A Flatpak sandbox needs a different approach.

## How it finds the game

It attaches to a process named `dolphin-emu` (or one of the Windows names), then looks for the 32 MiB read/write mapping whose bytes at offset 0x1C hold the GameCube magic `0xC2339F3D`. Game addresses are offsets into that mapping, minus `0x80000000`.

## Status

Attaching, auto start and the first split are confirmed working. Reset, room splits, the ending splits and the podrace
status-split suppression are untested.
