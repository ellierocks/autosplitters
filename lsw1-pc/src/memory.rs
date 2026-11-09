//! Module-relative JP addresses. See ../addresses.md for evidence and confidence.
use asr::{Address, Address32, Process};
use core::mem::MaybeUninit;

use crate::logic::State;

const PROCESS_NAMES: &[&str] = &["LegoStarwars.exe", "LegoStarwars"];
const START: u64 = 0x0039_9390;
const TITLE: u64 = 0x0037_b248;
const CUTSCENE: u64 = 0x0038_b30c;
const DOOR: u64 = 0x0039_93a0;
const LEVELPATH: u64 = 0x0037_31fc;
const CURRSCENE: u64 = 0x0037_2434;

// These code windows avoid all lsw1-jp-patch debug/resolution edits and have no
// relocation-sensitive absolute addresses, so a relocated JP image also works.
const SIGNATURES: &[(u32, &[u8])] = &[
    (
        0x0005_3cf0,
        &[
            0x53, 0x8b, 0x5c, 0x24, 0x0c, 0x56, 0x8b, 0x74, 0x24, 0x0c, 0x8b, 0x96, 0x90, 0x01,
            0x00, 0x00,
        ],
    ),
    (
        0x000b_d800,
        &[
            0x8b, 0x54, 0x24, 0x08, 0x56, 0x8b, 0x74, 0x24, 0x08, 0x8d, 0x4a, 0x06, 0x8d, 0x46,
            0x02, 0x2b,
        ],
    ),
];

/// Reads only fixed, bounded ranges before any JP globals are accessed.
fn is_jp(mut read: impl FnMut(u32, &mut [u8]) -> Option<()>) -> bool {
    let mut dos = [0; 64];
    if read(0, &mut dos).is_none() || &dos[..2] != b"MZ" {
        return false;
    }
    let pe = u32::from_le_bytes(dos[60..64].try_into().unwrap());
    if !(64..=0x1000).contains(&pe) {
        return false;
    }
    let mut header = [0; 84];
    if read(pe, &mut header).is_none()
        || &header[..4] != b"PE\0\0"
        || header[4..6] != 0x014cu16.to_le_bytes() // x86
        || header[6..8] != 4u16.to_le_bytes() // stock JP sections
        || header[8..12] != 0x429c_4f34u32.to_le_bytes()
        || header[24..26] != 0x010bu16.to_le_bytes() // PE32
        || header[80..84] != 0x02fc_c000u32.to_le_bytes()
    // SizeOfImage
    {
        return false;
    }
    SIGNATURES.iter().all(|&(rva, expected)| {
        let mut bytes = [0; 16];
        read(rva, &mut bytes[..expected.len()]).is_some() && &bytes[..expected.len()] == expected
    })
}

pub(crate) struct Game {
    pub(crate) process: Process,
    module: Address,
}

pub(crate) fn attach() -> Option<Game> {
    // Check each matching PID: an unsupported build must not hide a JP process
    // with the same executable name. Failed identification drops the handle.
    for &name in PROCESS_NAMES {
        let mut ids = [MaybeUninit::uninit(); 32];
        let Some((ids, _)) = Process::list_by_name_into(name, &mut ids) else {
            continue;
        };
        for &id in ids.iter() {
            let Some(process) = Process::attach_by_pid(id) else {
                continue;
            };
            let module = process
                .get_module_address("LegoStarwars.exe")
                .or_else(|_| process.get_module_address(name))
                .ok()
                .filter(|&base| is_jp(|rva, bytes| process.read_into_buf(base + rva, bytes).ok()))
                .or_else(|| {
                    // Wine can report the Unix loader as the main module. The
                    // PE image has its own memory mapping; verify every candidate.
                    process
                        .memory_ranges()
                        .filter_map(|range| range.address().ok())
                        .find(|&base| {
                            is_jp(|rva, bytes| process.read_into_buf(base + rva, bytes).ok())
                        })
                });
            let Some(module) = module else { continue };
            return Some(Game { process, module });
        }
    }
    None
}

impl Game {
    pub(crate) fn poll(&self) -> Option<State> {
        let p = &self.process;
        let base = self.module;
        let scene = p.read::<u8>(base + CURRSCENE).ok()?;
        let path = p.read::<Address32>(base + LEVELPATH).ok()?;
        if path == Address32::NULL {
            return None;
        }
        let state = State {
            // JP status is unresolved; see addresses.md. Keep completion
            // predicates in the logic, but do not read a guessed offset.
            status: None,
            start: p.read(base + START).ok()?,
            title: p.read(base + TITLE).ok()?,
            cutscene: p.read(base + CUTSCENE).ok()?,
            currscene: scene,
            door: p.read(base + DOOR).ok()?,
            levelpath: p.read(path).ok()?,
        };
        // The path is the first field of a double-buffered world object. Do not
        // mix a room from one buffer with the globals from its successor.
        if p.read::<Address32>(base + LEVELPATH).ok()? != path
            || p.read::<u8>(base + CURRSCENE).ok()? != scene
        {
            return None;
        }
        Some(state)
    }
}

#[cfg(test)]
mod tests;
