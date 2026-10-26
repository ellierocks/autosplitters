# LEGO Star Wars 1 (GameCube) autosplitter, WASM

A port of the Dolphin `dolphin.asl` autosplitter to a LiveSplit One auto splitting runtime (WebAssembly), so it works on Linux with the OBS LiveSplit One plugin.

## Settings

| Setting | Default | Meaning |
| --- | --- | --- |
| Start on New Game | on | Start the timer when a new game begins |
| End on Darth Maul Outro (Any%) | on | Final split for Any% |
| Split on every room (Any%) | off | Split on the listed door transitions |
| End on Darth Vader Outro (All Minikits) | off | Final split for All Minikits |
| End on A New Hope Outro (100% NSR) | off | Final split for 100% NSR |

The timer also splits when `status` goes 0 to 1, and resets when `gamestate` is 124.

## Build

Requires Rust with the `wasm32-unknown-unknown` target:

```
rustup target add wasm32-unknown-unknown
cargo build --release
```

The output is `target/wasm32-unknown-unknown/release/lsw1_autosplitter.wasm`. Load it in the LiveSplit One OBS plugin's auto splitter settings.

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

Attaching, auto start and the first split are confirmed working. Reset, room splits and the ending splits are untested.
