//! LEGO Star Wars: The Complete Saga... no, LEGO Star Wars 1 (GameCube) autosplitter
//! for Dolphin, ported from the original ASL. Works on Linux (`dolphin-emu`) and Windows.

#![no_std]

use asr::{
    future::{next_tick, retry},
    settings::Gui,
    string::ArrayCString,
    timer::{self, TimerState},
    Address, Endian, FromEndian, MemoryRangeFlags, Process,
};

asr::async_main!(stable);
asr::panic_handler!();

const STATUS: u32 = 0x802E1CB8;
const GAMESTATE: u32 = 0x804093B0;
const CUTSCENE: u32 = 0x801BB45D;
const DOOR: u32 = 0x801DC964;

const MEM1_SIZE: u64 = 0x2000000;

const PROCESS_NAMES: &[&str] = &[
    "dolphin-emu",
    "dolphin-emu-nogui",
    "Dolphin.exe",
    "DolphinQt2.exe",
    "DolphinWx.exe",
    "DolphinNoGUI.exe",
];

#[derive(Gui)]
struct Settings {
    /// Start on New Game
    #[default = true]
    newgame: bool,
    /// End on Darth Maul Outro (Any%)
    #[default = true]
    maul: bool,
    /// Split on every room (Any%)
    #[default = false]
    room: bool,
    /// End on Darth Vader Outro (All Minikits)
    #[default = false]
    vader: bool,
    /// End on A New Hope Outro (100% NSR)
    #[default = false]
    anh: bool,
}

/// Doors where entering them (the door string changing to them) splits.
const ENTER_DOORS: &[&[u8]] = &[
    b"door_to_hangar",
    b"door_into_ruins1",
    b"door_into_ruins2",
    b"RescueA_RescueB",
    b"RescueB_RescueC",
    b"RescueC_RescueE",
    b"retakeA_retakeB",
    b"retakeB_retakeD",
    b"retakeD_retakeE",
    b"retakeE_retakeF",
    b"Palace_Hangar",
    b"door1in",
    b"door10out",
    b"door2corr",
    b"door_bossout",
    b"door_4",
    b"door_3in",
    b"door_5in",
    b"door_6in",
    b"door_8out",
    b"GunshipA_B",
    b"door_main_in",
    b"door_grev",
    b"door_lobby_out2",
    b"door_to_beach",
    b"door_to_swamp",
    b"door_to_forest",
    b"door_to_inside",
    b"door_to_archive",
    b"door_to_control",
    b"door_fight",
    b"door_2",
    b"door_3",
    b"door_8",
];

/// Doors where leaving them (the door string changing away from them) splits.
const LEAVE_DOORS: &[&[u8]] = &[
    b"door_into_ruins2",
    b"door10out",
    b"door_vent2",
    b"door_shaft_out3",
    b"door_3",
];

#[derive(Clone, Copy, Default)]
struct Watched {
    status: u8,
    gamestate: i32,
    cutscene: u8,
    door: ArrayCString<16>,
}

struct Emu {
    process: Process,
    ram: Address,
}

impl Emu {
    fn read<T: bytemuck::CheckedBitPattern + FromEndian>(&self, addr: u32) -> Option<T> {
        let off = addr.checked_sub(0x8000_0000)? as u64;
        self.process
            .read::<T>(self.ram + off)
            .ok()
            .map(|v| v.from_endian(Endian::Big))
    }

    fn read_door(&self) -> Option<ArrayCString<16>> {
        let off = (DOOR - 0x8000_0000) as u64;
        self.process.read(self.ram + off).ok()
    }

    fn poll(&self) -> Option<Watched> {
        Some(Watched {
            status: self.read(STATUS)?,
            gamestate: self.read(GAMESTATE)?,
            cutscene: self.read(CUTSCENE)?,
            door: self.read_door()?,
        })
    }
}

/// Finds Dolphin's emulated MEM1. Dolphin exposes it as a 32 MiB rw mapping;
/// bytes 0x1C..0x20 of GameCube memory hold the DVD magic 0xC2339F3D.
fn find_ram(process: &Process) -> Option<Address> {
    process
        .memory_ranges()
        .find(|range| {
            range
                .flags()
                .is_ok_and(|f| f.contains(MemoryRangeFlags::READ | MemoryRangeFlags::WRITE))
                && range.size().is_ok_and(|s| s == MEM1_SIZE)
                && range.address().is_ok_and(|a| {
                    process
                        .read::<u32>(a + 0x1C)
                        .is_ok_and(|m| u32::from_be(m) == 0xC233_9F3D)
                })
        })?
        .address()
        .ok()
}

fn attach() -> Option<Emu> {
    let process = PROCESS_NAMES.iter().find_map(|n| Process::attach(n))?;
    let ram = find_ram(&process)?;
    Some(Emu { process, ram })
}

fn door_is(door: &ArrayCString<16>, name: &[u8]) -> bool {
    door.as_bytes() == name
}

fn door_in(door: &ArrayCString<16>, set: &[&[u8]]) -> bool {
    set.iter().any(|n| door_is(door, n))
}

fn should_split(s: &Settings, old: &Watched, cur: &Watched) -> bool {
    if old.status == 0 && cur.status == 1 {
        return true;
    }
    let changed = old.door != cur.door;
    if s.room && changed && (door_in(&cur.door, ENTER_DOORS) || door_in(&old.door, LEAVE_DOORS)) {
        return true;
    }
    if s.maul && door_is(&cur.door, b"door_8") && cur.cutscene == 1 {
        return true;
    }
    if s.vader && door_is(&cur.door, b"door_fight") && cur.cutscene == 1 {
        return true;
    }
    if s.anh && door_is(&cur.door, b"door_from_room2") && cur.cutscene == 1 {
        return true;
    }
    false
}

async fn main() {
    let mut settings = Settings::register();

    loop {
        let emu = retry(attach).await;
        let mut old: Option<Watched> = None;

        while emu.process.is_open() {
            settings.update();

            let Some(cur) = emu.poll() else {
                // RAM went away (emulation stopped); re-scan.
                if find_ram(&emu.process) != Some(emu.ram) {
                    break;
                }
                next_tick().await;
                continue;
            };

            if let Some(old) = old {
                match timer::state() {
                    TimerState::NotRunning => {
                        if settings.newgame && old.gamestate == -1 && cur.gamestate == 123 {
                            timer::start();
                        }
                    }
                    TimerState::Running => {
                        if cur.gamestate == 124 {
                            timer::reset();
                        } else if should_split(&settings, &old, &cur) {
                            timer::split();
                        }
                    }
                    _ => {}
                }
            }
            old = Some(cur);

            next_tick().await;
        }

        timer::pause_game_time();
    }
}
