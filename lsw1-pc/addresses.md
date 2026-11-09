# Japanese PC 1.0 address research

Analysis date: 2026-11-09. All code/data addresses below are **absolute VAs**
at preferred image base `0x400000`; Rust constants subtract that base. All PC
scalars and pointers are little-endian. ASL byte watches deliberately read
only the low byte even where the JP engine uses a 32-bit integer.

Inputs:

- Stock JP exe SHA-256 `e224197c7ca58891d8bc0f3da6a3d29ed9ff213f9b8ef52e3708ec0db2f0921b`.
- Patch 2 exe: January 2006 SecuROM build, SHA-256 prefix `09cd92f1`, suffix
  `ba55`. Its game code is encrypted on disk; this investigation did **not**
  inspect decrypted patch 2 execution.
- `joey.asl`, exact supplied ASL snapshot; offsets below include its image base.
- The local `lsw1-decomp` repo: `docs/symbol_donors/pc_jp_globals.tsv`,
  `pc_jp_pairs.tsv`, `build/pc_analysis/gc_pc_anchors.tsv`, GC source and assembly.
- REA Ghidra 12.1.2 static decompilation of the stock JP exe. Evidence IDs below
  refer to that analysis, not gameplay observations. No decompiled game source,
  exe or game asset is committed here.

The launcher may rewrite the installed JP exe. The hash above identifies the
**analysis input**, not the running-image acceptance rule. Runtime identification
uses PE metadata and two code windows, described below.

## Mapping ledger

“Derived” means the JP code establishes the same role that the ASL needs, and
data/GC evidence corroborates it. It does not prove identical edge timing in
patch 2 and JP. The live capture below corroborates the Cruiser completion and
diner-to-title transitions; other routes and New Game remain untested.

| ASL value | Patch 2 VA | JP VA (RVA) | Confidence and JP evidence |
|---|---|---|---|
| `start` byte | `0x79B0D0` | `0x799390` (`0x399390`) | **Derived.** Menu-input handler `0x45F730` sets it to 1 at `0x45FA18` when a menu entry requests a pending scene. Main/scene loop `0x457F90` tests it to force a fresh level initialization, then clears it at `0x45841E`. Fits the ASL's rising edge while menu depth=1 and scene=124. This is a menu transition request, not the separate GC New Game flag. |
| `title` byte | `0x77B618` | `0x77B248` (`0x37B248`) | **Derived.** Menu stack depth. `0x4FB820` increments it, indexes records with stride `0xE0`, and logs `Checking GameMenuLevel %d`; pop paths decrement it (`0x4FBB68`, `0x4FBC38`). It gates `0x45F690`'s current-menu lookup. A 24-byte initialized data neighborhood at this JP location uniquely matches the source's patch 2 neighborhood. The ASL wants depth 1, not a Boolean “title visible” flag. |
| `currscene` byte | `0x771444` | `0x772434` (`0x372434`) | **Derived, strong.** `0x457F90` initializes -1 to `0x7C`, indexes the scene table `0x76D0D8` with stride `0xA8`, and later copies pending scene `0x772438` into this global. JP scene 124 has path/name `titles`, 123=`Map\\Map_E`, 125=`credits`, 126=`status`. A 24-byte initialized neighborhood uniquely matches patch 2. GC counterpart `lbl_804093AC`; pending scene `lbl_804093B0` maps separately to JP `0x772438`. |
| `door` string16 | `0x79B0E0` | `0x7993A0` (`0x3993A0`) | **Derived, strong.** `0x4510B0` copies the door name to this buffer using `0x4CDA80` (NUL-terminated copy). `0x450980` resolves it against the loaded world's door objects and clears its first byte if unmatched. `0x454A76` also clears it; `0x452FB0` writes hub spawn doors here. GC counterpart `lbl_801DC964`, copied by `fn_800CC3FC`. The backing buffer is larger than 16; the port preserves ASL string16. |
| `levelpath` string64, pointer+0 | `0x77220C → +0` | `0x7731FC → +0` (`0x3731FC`) | **Derived, strong.** Global pointer to current world, independently paired by `PadOutPause` JP `0x456850` ↔ GC `0x8009F0B8` / `lbl_80409410`. Constructor `0x453CF0` writes `levels\\` then the scene table's path to **offset 0** of the world, and builds the file path separately at +`0x80`. Main swaps this pointer between `0x3295D60` and `0x3297B78`. Read a 32-bit pointer once and then the string at +0; do not add another pointer dereference. |
| `cutscene` byte | `0x78D04C` | `0x78B30C` (`0x38B30C`) | **Derived candidate; timing unverified.** This JP global counts active cutscene instances. `0x41CDE0` zeroes it at `0x41CEC7`, walks eight slots, and increments it at `0x41D23A` for active instances. Start routine `0x41D8B0` writes 1 at `0x41DC5C`; render routine `0x41D780` checks >0. This supports the ASL's `==1` outro check. It is distinct from the engine playing flag `0x32A02E8`. No readable patch 2 writer was available to prove the cross-build correspondence; verify on each outro. |
| `status` byte | `0x79C58D` | `0x79A845` (`0x39A845`) | **Derived and observed on Cruiser.** SFX snapshot `0x79A7F8+0x4D`. The status-scene branch of `0x488170` adds physical sounds 617 and 620, producing `0x12` here; traced through `0x4BD700` and snapshot copy at `0x488BCC`. Both running-engine IDs and Cruiser `0 → 18 → 0` behavior were observed. Negotiations scene 6 and Cruiser scene 97 both select this branch. Generic completion reads this byte; Kamino `5 → 18` remains source parity pending live validation. |

These mappings are separate conclusions: initialized data moves by `+0xFF0`
for `currscene`, `-0x3D0` for `title`; other code-derived globals happen to have
a `-0x1D40` difference. The supplied LEGO Options flag moves by `-0x1D40`, its
label by `-0x5028`, and the cutscene-playing flag by `-0x3C780`. None supplies
a universal translation.

Status moves by `-0x1D48`, not the nearby globals' `-0x1D40`.

## Rejected status shortcut

Subtracting `0x1D40` from the ASL's status gives `0x79A84D`. In JP this falls
at `+0x55` of the 100-byte SFX bit snapshot at `0x79A7F8`:

- `SetLevelSfxBits` (`0x488170`, GC anchor `0x800E3DC4`) snapshots and masks
  that buffer at `0x488BCC` / `0x488BDD` using `0x4BD7C0` and `0x4BD800`.
  Restore helper `0x488D00` passes it to `0x4BD7E0`.
- `0x4BD7C0` copies 100 bytes from `0x3368980`. `0x4BD700` sets bits using a
  sample's physical sound ID, with 16-bit words and ordinary little-endian bit
  numbering. This is a resource bitset, not an enum with values 0, 5 and 18.
- GC's existing status watch is also inside an SFX snapshot (`0x802E1C74+0x44`),
  which explains why an audio bitset is worth investigating. Its byte position
  is **not** a JP address mapping; sample IDs and platform tables differ.
- JP snapshot byte `+0x55` represents physical sample IDs 680–687. A value of
  `0x12` selects `Ep3_1_ExplosionM` and `Dog_CloneARC170Gun`, referenced by
  `SetLevelSfxBits` at `0x488747` and `0x488774`. Status sounds instead occupy
  IDs 253 and 268–271 (`StatusAward` is referenced at `0x4881B5`).
- The readable built-in tables in patch 2 (`0x773488`) and JP (`0x774470`)
  each contain 246 entries. Combining these with each installation's
  `Audio/audio.cfg` gives the same ordered list of 783 physical sample paths.
  A PC sound-ID translation therefore does not explain this candidate. The
  containing global for patch 2's ASL byte still needs to be established.

Nothing in the JP readers/writers establishes that `0x79A84D` yields 18 at
completion. It also stayed zero throughout the Cruiser capture. The correct
JP byte is eight bytes earlier, as established independently below.

## Resolved status byte: `0x79A845`

Following the first 1-1 test, the status branch was traced in full rather than
assuming its sound names would all begin with `Status`:

- `SetLevelSfxBits` tests `world->scene_record->flags & 0x4000` at
  `0x4881A3–0x4881AF`. The scene record comes from world offset `0x1A8`;
  its flags are at record +4. Negotiations scene 6 (`Negotiations_Status`) has
  flags `0x4001`; Cruiser scene 97 has `0x4000`. All 18 chapter/bonus status
  entries have this bit. Generic scene 126 is not their completion scene.
- This branch looks up `Kam_TurretOn` at `0x4881D2`, then
  `Kam_DiscoFFieldOff` at `0x4881E1`, alongside `StatusAward` and other UI
  effects. Their names describe their original use; the status screen also
  loads them. REA evidence:
  `ev_2553d080c3a069443b2af57f837fc00923bfe521459a46c8675af55ab22145e7`.
- Read-only inspection of the running JP sample table `0x3375200` confirmed
  logical entries 381/384 resolve to physical IDs **617/620**. Lookup
  `0x4BD900` reads the name pointer at entry +0; `0x4BD700` extracts the signed
  15-bit physical ID at entry +4 (stride `0x40`). Both bits are in byte
  `617 / 8 = 620 / 8 = 0x4D`; bits 1 and 4 produce **`0x12`**.
- `0x488BCC` copies the active sound bits into snapshot `0x79A7F8` through
  `0x4BD7C0`, then masks the snapshot through `0x4BD800`. Thus the watched byte
  is `0x79A7F8 + 0x4D = 0x79A845`, not snapshot +`0x55`.
- The Cruiser capture below observed exactly `0 → 18` on results entry and
  `18 → 0` on diner return. The same status branch covers Negotiations and
  the other status entries. This establishes the JP role needed by the ASL
  without claiming to have read encrypted patch 2 code. The runtime now reads
  the raw byte and uses the source's unchanged `0/5 → 18` predicates.

The regression test exercises the actual memory-reading function through the
split engine for both Negotiations and Cruiser, checking the status address,
both preceding values and no repeated split while 18 stays set. The first
experimental build supplied `None` unconditionally, so its logic-only tests
could pass while no live status screen could split.

## Cutscene and New Game distinctions

The supplied engine-playing pair `0x32DCA68 ↔ 0x32A02E8` is corroborated by
`PadOutPause` and JP `0x41CD40` / `0x41D670`; it does not identify Joey's
`0x78D04C`. Likewise, the GC splitter's `0x801BB45D` is the low byte of a
music-stream field, not proof of either PC watch. The JP **count** candidate
was selected from its own writers/readers. Check that one active instance is
the desired outro edge, including Gunship's midtro and debug pausing.

GC's New Game flag `lbl_80409118` is already paired to JP `0x79A050`.
The GC title selection handler sets that flag while requesting scene 123.
Joey's PC start formula instead wants the earlier menu scene-request byte
(`0x799390`) with menu depth=1 and current scene=124. Replacing it with the
GC flag would change the source trigger.

## Runtime build guard

Before polling any globals, require DOS/PE magic, x86, PE32, four sections,
timestamp `0x429C4F34`, SizeOfImage `0x02FCC000`, and these exact windows:

| JP VA | Bytes | Role |
|---|---|---|
| `0x453CF0` | `53 8B 5C 24 0C 56 8B 74 24 0C 8B 96 90 01 00 00` | world constructor prologue |
| `0x4BD800` | `8B 54 24 08 56 8B 74 24 08 8D 4A 06 8D 46 02 2B` | SFX-bit mask function prologue |

These windows contain no absolute relocation operands. They avoid launcher
patches at `0x41CD00`, `0x45ECC4`, `0x45EDA9`, `0x45EDBE`, `0x74E670`,
the `.rdata` VirtualSize field, and size immediates `0x51EB0B` / `0x51EB16`.
Patch 2 has a different timestamp (`0x43C5009A`), image size and section count.
The ignored real-file test accepted stock JP, accepted JP with all listed
patch locations changed, and rejected the supplied patch 2 file. This is a
build discriminator, not a claim that every byte of the process is unmodified.

## Remaining live checks

No game was launched, stopped, restarted or controlled by the agent. Ellie
performed the gameplay transitions in the capture below. These remaining checks
need OBS/LiveSplit One as well as the game; reading RAM does not verify the
runtime's process attachment or actual timer actions.

1. **New Game:** watch JP `0x799390`, `0x77B248`, `0x772434` together at 60 Hz.
   Confirm the start byte rises while depth=1 and current scene=124, before
   scene becomes 123. Check load-game selection does not accidentally start.
2. **Generic completion:** confirm the installed wasm splits once on
   Negotiations results and on other chapter results. Watch `0x79A845` for
   `0→18`; test Kamino Free Play's source `5→18` case and no lap/room false
   positives. A patch 2 writer capture would further corroborate the source
   correspondence; no source code has been read from its encrypted disk image.
3. **Paths/doors:** watch `*(u32*)0x7731FC` at offset 0 and `0x7993A0` through
   loads, Podrace laps and Maul's wall warp; confirm expected ASCII paths,
   capitalization, 16-byte door truncation and edge order. JP scene-table paths
   for titles, hub and Podrace match the ASL; all routes still need testing.
4. **Outros:** record `0x78B30C` alongside the engine flag `0x32A02E8` during
   Maul, Vader, A New Hope and Gunship; confirm count=1 occurs at the intended
   final cue. Test Gunship after the 600-update grace period and debug pause.
5. **Reset/Rush/latches:** diner→title must reach `levels\\titles` within the
   60-update leeway (observed once below); loading a save must not reset. Check Rush countdown at
   60 Hz and room backtracking does not retrigger one-shot latches.

## Live capture: Cruiser completion and diner-to-title

On 2026-11-09 at 14:57:21 UTC, a read-only `/proc/689554/mem` observer sampled
the already-running launcher-patched JP game at a requested 120 Hz for 91.633 s.
Both build-guard code signatures were checked first. The capture contains
10,996 consistent samples and no read errors; unchanged samples were compressed.
Ellie performed the completion and menu actions. This is RAM observation, not
an OBS/wasm timer test, and the fields are read sequentially rather than atomically.

| Elapsed seconds | Observation |
|---|---|
| 0.000042 | Scene 94, `levels\\episode_iii\\chapter_02\\Cruiser_G`, door `door_grev`, count/playing=0. |
| 50.783401 | Menu request byte rises to 1, pending scene=96, depth=3; the door clears. |
| 50.833421 | Scene 96, `Cruiser_Outro`; world switches to `0x3297B78`; count=1 and playing=1. |
| 65.266735 | Playing clears while count remains 1. |
| 65.300072 | Scene 97, `Cruiser_Status`; world switches to `0x3295D60`. Snapshot byte `0x79A845` (`+0x4D`) changes `0 → 18`. |
| 65.325070 | Active count clears, about 25 ms after results-path entry. |
| 74.216787 | Scene 123, `levels\\Map\\Map_E`; scene request is still pending. |
| 75.225073 | Request clears, door becomes `door_to_E3_02`; new status candidate returns to 0. |
| 78.458401 | Request rises with pending scene=124, depth=2; diner path remains readable. |
| 79.058414 | Current scene becomes 124 while the old diner path remains readable. |
| 79.066783 | World path is briefly empty. |
| 79.075077 | `levels\\titles`, request=0, depth=1; world switches to `0x3297B78`. |

The reset leeway continues to refresh while the diner path remains visible,
including most of the 0.617 s return transition. Only about 17 ms separates the
last observed diner sample and the title-path sample. Replaying these memory
states at 60 Hz with several sampling phases produces exactly one reset and
no New Game start. This confirms the predicate against this trace, not an
actual LiveSplit timer reset. New Game's rising edge was not exercised.

The rejected `0x79A84D` stayed zero throughout completion. The observed byte
`0x79A845` was subsequently mapped through its writer and the running-engine
sound IDs, as described above. This route uses scene **97**, not generic scene
126: a scene-126-only completion fallback would miss it.

The local raw JSONL capture was kept outside Git at
`/tmp/lsw1-jp-ram-2026-11-09.jsonl`, SHA-256
`c1f0e892ca393d41bd912600d27e868fd397ae4dae44a88510a506df73ca5fec`.

## Reusable static evidence

Selected REA Evidence IDs (full local bundle exported separately):

| Finding | Evidence ID |
|---|---|
| PadOutPause world pointer / playing flag | `ev_d8be8a77568528af94dc2ca6788f85738b6c8fe6d82ceffab0ecae450b397386` |
| Menu transition request writer | `ev_482e601e342afab8e14f46e1c436a3f2c2032d9ec406a86bd6ee48b37e469a90` |
| Menu stack push | `ev_248d3725f97ae01574e44ccd95a4501abed2144b6f74636a4f8a6fe389e5d919` |
| Main scene loop / current scene | `ev_8ec1b57c520c3766894b95e33dd9acfe0b27d930e219c1e4fd5879a1fc8f6d0b` |
| Door writer | `ev_a890467f826c5fa9d1f38e13f8f5b479995ae30943e3b27357f5cbf887b004f4` |
| Door resolver/clear | `ev_cde65378f3ac01d2293786da93598592b252f544d2978614f58a9eecb2912684` |
| World constructor / levelpath at offset 0 | `ev_984b3f836358a04dba13c0a213f81980ea83d7be0217bd870440c7e18d9b14b8` |
| Active cutscene count writers | `ev_e9f87420f3288fd70ae4040921dfbfd3c837ea55982fc4b1134203812a98a5d8` |
| Start-cutscene count=1 | `ev_7110da9aff3e18686b4eddd73fa6994003b960a8b98875311b9e57b3f0852f42` |
| SFX snapshot copy | `ev_25b12f93c620eadbba2910f2cc17642c6d2dd78fed73bd08fff254c2baab71ed` |
| SFX physical ID→bit | `ev_54804fc358e169e99aa2c73109f3c972067e30e16fb624d8670b60d826b758e9` |

Reproduce by opening the stock JP exe in REA/Ghidra and requesting
`procedure_pseudo_code` at the listed code addresses, or disassemble with
`objdump -d -Mintel`. No original PDB was loaded. GC anchors are donor evidence;
they do not guarantee every generated pairing, function name or decompiler
type is correct. Indirect references and sample/read races remain limitations.
