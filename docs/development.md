# Development reference

Monorepo for my autosplitters. Each splitter is a Cargo workspace member that compiles to WebAssembly for LiveSplit
One's auto-splitting runtime, so it runs in the OBS LiveSplit One plugin on Linux.

- `lsw1/` (crate `lsw1_wasm`): LEGO Star Wars 1 (GameCube, under Dolphin). Ships two supported autosplitters kept in
  sync: `lsw1/dolphin.asl` for desktop LiveSplit, and the wasm port in `lsw1/src/lib.rs` for LiveSplit One.
- `sadx/` (crate `sadx-livesplit-one`): Sonic Adventure DX (PC). A port of Sora-yx's desktop ASL; folded in from the
  old GitHub repo `ellierocks/sadx-livesplit-one` with its history on 2026-10-29.

## Layout

One `Cargo.lock`, `target/`, `rust-toolchain.toml`, release `[profile]` and `.cargo/config.toml` (defaults the build
target to `wasm32-unknown-unknown`) at the root. Each splitter has its own dir with `Cargo.toml`, `README.md`,
`CHANGELOG.md` and `src/`. To add one: a new member in the root `Cargo.toml`, plus a `[[package]]` entry in
`release-plz.toml` if its tags shouldn't be `<crate name>-v<version>`.

### lsw1 (all in `lsw1/src/lib.rs`)
- Memory addresses (`STATUS`, `GAMESTATE`, `CUTSCENE`, `DOOR`) in emulated GameCube RAM.
- `Settings` (shown in the LiveSplit One UI).
- The door lists for room splits.
- `find_ram`/`attach`, which locate Dolphin's MEM1 in the process.
- `should_split`, the split decision: old state compared with new state.

### sadx (all in `sadx/src/lib.rs`)
- Memory offsets from the game module (`GLOBAL_FRAME_COUNT`, `GAME_MODE`, ...), read through `read_state`.
- Frame-counted game time; pre-rendered movies are timed with a wall clock (`clock::Instant`).
- `should_start`, `should_reset`, `should_split`.

Both are built on the `asr` crate, pinned to the same git rev.

## Commands (what CI runs, from the repo root)

```sh
cargo build --release --locked --workspace     # .cargo/config.toml defaults the target to wasm32-unknown-unknown
cargo clippy --release --locked --workspace -- -D warnings
cargo fmt --all --check
cargo test --workspace --target x86_64-unknown-linux-gnu --locked                     # tests run on the host
cargo clippy --workspace --target x86_64-unknown-linux-gnu --locked --tests -- -D warnings
```

The Rust toolchain is pinned in `rust-toolchain.toml`; Renovate bumps it.

Output: `target/wasm32-unknown-unknown/release/<crate>.wasm` (`lsw1_wasm.wasm`, unchanged from before the monorepo,
and `sadx_livesplit_one.wasm`). A plain `cargo test` fails, because the default target is wasm. The no_std libs only
link for wasm, so host builds need `--tests`.

The lsw1 tests check that `ENTER_DOORS`/`LEAVE_DOORS`/`PODRACE_DOORS` still match `lsw1/dolphin.asl`, and check
`should_split`'s settings and edge cases. When split logic changes, change the ASL parity expectations deliberately.

## Release

release-plz cuts a release per splitter from Conventional Commits. lsw1's tags are `lsw1-vX.Y.Z` (older `vX.Y.Z` tags
from when this repo was `lsw1-wasm` are kept, with `lsw1-v` copies), and its changelog is `lsw1/CHANGELOG.md`.
sadx's tags are `sadx-vX.Y.Z`; 0.1.0 was released from the old repo and has no tag here (a tag on the imported
commits breaks release-plz, which needs the workspace root), so its first release here is 0.2.0.

The current [`release-plz.yml`](../.github/workflows/release-plz.yml) uses `release-plz/action` with
`RELEASE_PLZ_TOKEN`, falling back to `github.token`. Historical recipe: the CLI workflow passed the token to the
GitHub API via `--git-token`, but release-plz pushed its release branch and tags with plain `git push` from a temp
clone, so it also installed a `credential.helper` handing git the same token. Without that helper, the historical
workflow failed to authenticate pushes.

`release.yml` runs for those tags: it builds that splitter's `.wasm`, plus any `.asl` in its dir (lsw1's
`dolphin.asl`), and publishes a GitHub release with the changelog section as notes. GitHub is the home; Codeberg is a
read-only mirror.
