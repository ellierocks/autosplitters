# LEGO Star Wars: The Video Game — Japanese PC 1.0

Experimental LiveSplit One autosplitter using Rust `asr`, for the May 2005
Japanese PC release under Wine or Windows. Build from the workspace root:

```sh
cargo build --release --locked -p lsw1-pc
```

Load `target/wasm32-unknown-unknown/release/lsw1_pc.wasm` in LiveSplit One's
custom autosplitter setting. On Linux, OBS needs permission to read the Wine
game process (the same ptrace restriction described in the
[Dolphin setup](../lsw1/README.md#linux-allow-obs-to-read-dolphins-memory)).

Only the JP build is accepted. Identification uses PE metadata and two short
code signatures, preserving compatibility with `lsw1-jp-patch` debug labels,
cutscene pausing and resolution changes. It does not hash the installed exe.
Patch 2 and other builds are rejected before any JP gameplay globals are read.

Generic level-complete splitting reads JP `0x79A845` and follows Joey's
`0/5 → 18` predicates. The JP status-screen code loads the two sound bits that
produce 18; the running engine's sample table and a Cruiser completion capture
corroborate the mapping. Ellie also confirmed Negotiations (1-1) status-screen
splitting in LiveSplit One with the corrected build. The
earlier build left completion disabled; replace/reload its wasm to get this fix.
A diner-to-title capture also fits the reset window and is replayed in tests.
Other start/outro/category timing still needs live checks. See
[addresses.md](addresses.md) for evidence and the remaining checks. Treat this
as a starting point for live validation, not a race-ready splitter.

All ten setting IDs, labels and defaults follow Joey's ASL: `newgame`,
`freeplay`, `rushstart`, `maul`, `vader`, `anh`, `dooku`, `rushend`, `room`,
`resetonmenu`. The timer uses real time; the source has no load removal or game
time calculation. Sampling is 60 Hz, including the source's tick countdowns.
In particular, Rush starts 119 successful updates after track entry (about two
seconds at 60 Hz), despite the source comment calling it four seconds.

`src/logic.rs` is a separate PC port; it does not change the supported Dolphin
ASL/Rust pair. The latches, rule ordering and continuous predicates are kept
from the source, including empty-door Free Play starts and outro predicates
that may request a split on successive ticks. Manual timer resets do not clear
the ASL's per-process latches. A failed memory read discards the previous sample;
the next successful sample seeds a new baseline, avoiding stale edges across a
load. Only successful consecutive samples advance the ASL update state.

The source is [Elle and Joeyfordayz's retail PC ASL](https://github.com/Joeyfordayz/LSW1-PC-Assets/blob/main/LSW1-Autosplitter.asl),
as listed by LiveSplit's AutoSplitters.xml; the New Game trigger credits
coolestto. [joey.asl](joey.asl) is an unchanged research/parity snapshot
(`8507583524d92fc97d014642e716c61b443088294d55bf3326567427eea743b4`).
Its offsets target **patch 2**, so do not load it for JP. It is included with
releases as the reference, not as a supported JP desktop ASL.

Tests cover settings parity, room rules, latches, countdowns, start/reset edges,
and build rejection/launcher compatibility. Run the workspace's
[CI commands](../docs/development.md#commands-what-ci-runs-from-the-repo-root).
An additional ignored test checks the actual local binaries without storing
game data. In fish, substitute your local paths:

```fish
env LSW1_JP_EXE='/path/to/LegoStarwars.stock.exe' LSW1_PATCH2_EXE='/path/to/patch2/LegoStarwars.exe' cargo test -p lsw1-pc --target x86_64-unknown-linux-gnu --locked real_build_identity -- --ignored
```
