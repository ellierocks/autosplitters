//! Joeyfordayz / Elle's retail-PC ASL logic, kept independent of JP memory reads.
//! Call `update` once for every successful sample before evaluating timer actions.

use asr::{settings::Gui, string::ArrayCString};

#[derive(Gui, Clone)]
pub(crate) struct Settings {
    /// Start on New Game
    #[default = true]
    pub(crate) newgame: bool,
    /// Start on Level Entry (Free Play)
    #[default = false]
    pub(crate) freeplay: bool,
    /// Start After Podrace Countdown (Minikit Rush/True Jedi Rush)
    #[default = false]
    pub(crate) rushstart: bool,
    /// End on Darth Maul Outro (Any%)
    #[default = true]
    pub(crate) maul: bool,
    /// End on Darth Vader Outro (All Minikits)
    #[default = false]
    pub(crate) vader: bool,
    /// End on A New Hope Outro (All Levels/100% NSR)
    #[default = false]
    pub(crate) anh: bool,
    /// End on Count Dooku Outro (NoCut Any%)
    #[default = false]
    pub(crate) dooku: bool,
    /// End on Gunship Outro (Minikit Rush/True Jedi Rush)
    #[default = false]
    pub(crate) rushend: bool,
    /// Split on every room in Any%
    #[default = false]
    pub(crate) room: bool,
    /// Automatically Reset Timer upon Exiting to Title Screen
    #[default = true]
    pub(crate) resetonmenu: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            newgame: true,
            freeplay: false,
            rushstart: false,
            maul: true,
            vader: false,
            anh: false,
            dooku: false,
            rushend: false,
            room: false,
            resetonmenu: true,
        }
    }
}

#[derive(Clone, Copy, Default)]
pub(crate) struct State {
    // Unknown in an uninitialized/synthetic state. Runtime samples read the JP
    // SFX snapshot byte; see addresses.md for the status-screen writer.
    pub(crate) status: Option<u8>,
    pub(crate) start: u8,
    pub(crate) title: u8,
    pub(crate) cutscene: u8,
    pub(crate) currscene: u8,
    pub(crate) door: ArrayCString<16>,
    pub(crate) levelpath: ArrayCString<64>,
}

#[derive(Default)]
pub(crate) struct Engine {
    diner_leeway: u32,
    ruin_inside_split: bool,
    maul_room3_split: bool,
    maul_ww_door_leeway: u32,
    maul_ww_levelpath_leeway: u32,
    podrace_lap_count: u32,
    rush_start_leeway: u32,
    gunship_outro_leeway: u32,
    chancellor_room2_split: bool,
    chancellor_room3_split: bool,
    chancellor_room4_split: bool,
    chancellor_room5_split: bool,
    chancellor_room6_split: bool,
    chancellor_room7_split: bool,
    negotiations_hangar_split: bool,
    theed_retake_bd_split: bool,
    theed_retake_de_split: bool,
    kamino_door10out_split: bool,
    factory_door8out_split: bool,
    kashyyyk_forest_split: bool,
}

impl Engine {
    pub(crate) fn update(&mut self, settings: &Settings, old: &State, current: &State) {
        if current.levelpath.as_bytes() == b"levels\\Map\\Map_E" {
            self.diner_leeway = 60; // ~1 second of leeway at refreshRate 60, counts down every tick you're not in the diner
        } else if self.diner_leeway > 0 {
            self.diner_leeway -= 1;
        }

        if !current
            .levelpath
            .as_bytes()
            .starts_with(b"levels\\episode_iii\\chapter_06\\")
        {
            self.ruin_inside_split = false; // only re-arms once you leave Ruin entirely, not just by backtracking between its rooms
        }

        // Maul room2->3 entry (door_3): latch it so backing out of a failed room3->4 WW and re-entering room 3
        // doesn't cause a second split. Only re-arms once you leave Maul's level entirely.
        if !current
            .levelpath
            .as_bytes()
            .starts_with(b"levels\\episode_i\\chapter_07\\")
        {
            self.maul_room3_split = false;
        }

        // Maul room3->4 WW: door clears away from "door_3" (which also happens if you just back out to room2
        // without WWing) and levelpath becomes Maul_E on different ticks, so track a short leeway window from
        // each edge and only trust the door-clear as a split when the Maul_E transition actually shows up too.
        if old.door.as_bytes() == b"door_3" && current.door != old.door {
            self.maul_ww_door_leeway = 30;
        } else if self.maul_ww_door_leeway > 0 {
            self.maul_ww_door_leeway -= 1;
        }

        // Podrace lap count: only laps 1->2 and 2->3 should split, not the lap3->finish crossing
        // (that one just rolls straight into the outro, which can ride along with the lap 3 split).
        // Resets once you leave Podrace entirely, so a retry/reset re-arms it.
        if !current
            .levelpath
            .as_bytes()
            .starts_with(b"levels\\episode_i\\chapter_04\\")
        {
            self.podrace_lap_count = 0;
        }

        // Rush categories (Minikit Rush/True Jedi Rush) start delay: these categories usually start on Podrace, and the
        // timer should start the moment you actually gain control (after the 3-second countdown), not on
        // level load. Arm a 4-second (120 tick @ 30/60fps) countdown the instant you land in PodRace_B (the race
        // track room itself), and the start block below fires once that countdown runs out.
        if settings.rushstart
            && current.levelpath.as_bytes() == b"levels\\episode_i\\chapter_04\\PodRace_B"
            && old.levelpath.as_bytes() != b"levels\\episode_i\\chapter_04\\PodRace_B"
        {
            self.rush_start_leeway = 120;
        } else if self.rush_start_leeway > 0 {
            self.rush_start_leeway -= 1;
        }

        // Gunship outro end delay: entering Gunship_B (door == "GunshipA_B") triggers a short midtro cutscene
        // of its own (cutscene == 1 briefly), which would otherwise false-split the rush end check below.
        // Arm a 10-second (600 tick @ 60fps) grace period the instant that door is touched, and only trust
        // the cutscene flag for the real outro once that period has fully run out.
        if settings.rushend && current.door.as_bytes() == b"GunshipA_B" && old.door != current.door
        {
            self.gunship_outro_leeway = 600;
        } else if self.gunship_outro_leeway > 0 {
            self.gunship_outro_leeway -= 1;
        }

        if current.levelpath.as_bytes() == b"levels\\episode_i\\chapter_07\\Maul_E"
            && old.levelpath.as_bytes() != b"levels\\episode_i\\chapter_07\\Maul_E"
        {
            self.maul_ww_levelpath_leeway = 30;
        } else if self.maul_ww_levelpath_leeway > 0 {
            self.maul_ww_levelpath_leeway -= 1;
        }

        // Chancellor latches: reset only once you leave the level entirely, so backtracking within
        // Chancellor (very easy to do given how fast/WW-heavy this level's route is) can't double-split.
        if !current
            .levelpath
            .as_bytes()
            .starts_with(b"levels\\episode_iii\\chapter_02\\")
        {
            self.chancellor_room2_split = false;
            self.chancellor_room3_split = false;
            self.chancellor_room4_split = false;
            self.chancellor_room5_split = false;
            self.chancellor_room6_split = false;
            self.chancellor_room7_split = false;
        }

        // Single-split latches for a few other doors: reset at the start of every fresh attempt
        // (same new-game edge the "start" block uses), rather than a level-exit check.
        if current.start > old.start && current.title == 1 && current.currscene == 124 {
            self.negotiations_hangar_split = false;
            self.theed_retake_bd_split = false;
            self.theed_retake_de_split = false;
            self.kamino_door10out_split = false;
            self.factory_door8out_split = false;
            self.kashyyyk_forest_split = false;
        }
    }

    pub(crate) fn should_start(&self, settings: &Settings, old: &State, current: &State) -> bool {
        if settings.newgame
            && current.start > old.start
            && current.title == 1
            && current.currscene == 124
        {
            return true;
        } // credit goes to coolestto
        if settings.freeplay && current.door.as_bytes() == b"" {
            return true;
        }
        if settings.rushstart && self.rush_start_leeway == 1 {
            return true;
        }
        false
    }

    pub(crate) fn should_split(
        &mut self,
        settings: &Settings,
        old: &State,
        current: &State,
    ) -> bool {
        // ----- Level Complete (generic, applies across levels) -----
        if current.status == Some(18) && old.status == Some(0) {
            return true;
        }
        if current.status == Some(18) && old.status == Some(5) {
            return true;
        } // kamino freeplay fix

        // ----- Negotiations -----
        if settings.room
            && current.door.as_bytes() == b"door_to_hangar"
            && old.door != current.door
            && !self.negotiations_hangar_split
        {
            self.negotiations_hangar_split = true;
            return true;
        }

        // ----- Invasion -----
        if settings.room
            && current.door.as_bytes() == b"door_into_ruins1"
            && old.door != current.door
        {
            return true;
        }
        if settings.room
            && current.door.as_bytes() == b"door_into_ruins2"
            && old.door != current.door
        {
            return true;
        }
        if settings.room && old.door.as_bytes() == b"door_into_ruins2" && current.door != old.door {
            return true;
        } // WW
        if settings.room && current.door.as_bytes() == b"door_swamp" && old.door != current.door {
            return true;
        } // Normal

        // ----- Escape -----
        if settings.room
            && current.door.as_bytes() == b"RescueA_RescueB"
            && old.door != current.door
        {
            return true;
        }
        if settings.room
            && current.door.as_bytes() == b"RescueB_RescueC"
            && old.door != current.door
        {
            return true;
        }
        if settings.room
            && current.door.as_bytes() == b"RescueC_RescueE"
            && old.door != current.door
        {
            return true;
        }

        // ----- Podrace -----
        if settings.room && current.door.as_bytes() == b"door_B" && old.door != current.door {
            self.podrace_lap_count += 1;
            if self.podrace_lap_count <= 2 {
                return true;
            } // only laps 1->2 and 2->3; lap3->finish rolls into the outro
        }

        // ----- Theed -----
        if settings.room
            && current.door.as_bytes() == b"retakeA_retakeB"
            && old.door != current.door
        {
            return true;
        }
        if settings.room
            && current.door.as_bytes() == b"retakeB_retakeD"
            && old.door != current.door
            && !self.theed_retake_bd_split
        {
            self.theed_retake_bd_split = true;
            return true;
        }
        if settings.room
            && current.door.as_bytes() == b"retakeD_retakeE"
            && old.door != current.door
            && !self.theed_retake_de_split
        {
            self.theed_retake_de_split = true;
            return true;
        }
        if settings.room
            && current.door.as_bytes() == b"retakeE_retakeF"
            && old.door != current.door
        {
            return true;
        }
        if settings.room && current.door.as_bytes() == b"Palace_Hangar" && old.door != current.door
        {
            return true;
        }

        // ----- Kamino -----
        if settings.room && current.door.as_bytes() == b"door1in" && old.door != current.door {
            return true;
        }
        if settings.room
            && current.door.as_bytes() == b"door10out"
            && old.door != current.door
            && !self.kamino_door10out_split
        {
            self.kamino_door10out_split = true;
            return true;
        }
        if settings.room && old.door.as_bytes() == b"door10out" && current.door != old.door {
            return true;
        }
        if settings.room && current.door.as_bytes() == b"door2corr" && old.door != current.door {
            return true;
        }
        if settings.room && current.door.as_bytes() == b"door_bossout" && old.door != current.door {
            return true;
        }

        // ----- Factory -----
        if settings.room && current.door.as_bytes() == b"door_4" && old.door != current.door {
            return true;
        }
        if settings.room && current.door.as_bytes() == b"door_3in" && old.door != current.door {
            return true;
        }
        if settings.room && current.door.as_bytes() == b"door_5in" && old.door != current.door {
            return true;
        }
        if settings.room && current.door.as_bytes() == b"door_6in" && old.door != current.door {
            return true;
        }
        if settings.room
            && current.door.as_bytes() == b"door_8out"
            && old.door != current.door
            && !self.factory_door8out_split
        {
            self.factory_door8out_split = true;
            return true;
        }

        // ----- Gunship -----
        if settings.room && current.door.as_bytes() == b"GunshipA_mid" && old.door != current.door {
            return true;
        } // room1 part2
        if settings.room && current.door.as_bytes() == b"GunshipA_B" && old.door != current.door {
            return true;
        }

        // ----- Dooku -----
        if settings.room && current.door.as_bytes() == b"door_main_in" && old.door != current.door {
            return true;
        }

        // ----- Chancellor -----
        if settings.room
            && current.door.as_bytes() == b"door_grev"
            && old.door != current.door
            && !self.chancellor_room2_split
        {
            self.chancellor_room2_split = true;
            return true;
        } // room1->2
        if settings.room
            && current.levelpath.as_bytes() == b"levels\\episode_iii\\chapter_02\\Cruiser_B"
            && old.levelpath.as_bytes() != b"levels\\episode_iii\\chapter_02\\Cruiser_B"
            && !self.chancellor_room3_split
        {
            self.chancellor_room3_split = true;
            return true;
        } // room2->3 (door_vent1 or door_vent2, Normal or WW - both land here)
        if settings.room
            && current.levelpath.as_bytes() == b"levels\\episode_iii\\chapter_02\\Cruiser_C"
            && old.levelpath.as_bytes() != b"levels\\episode_iii\\chapter_02\\Cruiser_C"
            && !self.chancellor_room4_split
        {
            self.chancellor_room4_split = true;
            return true;
        } // room3->4 (door_vent1/vent2 WW or door_shaft_out2 Normal - both land here)
        if settings.room
            && current.levelpath.as_bytes() == b"levels\\episode_iii\\chapter_02\\Cruiser_D"
            && old.levelpath.as_bytes() != b"levels\\episode_iii\\chapter_02\\Cruiser_D"
            && !self.chancellor_room5_split
        {
            self.chancellor_room5_split = true;
            return true;
        } // room4->5 (door_lobby_out2 Normal or door_shaft_out2 WW - both land here)
        if settings.room
            && current.levelpath.as_bytes() == b"levels\\episode_iii\\chapter_02\\Cruiser_E"
            && old.levelpath.as_bytes() != b"levels\\episode_iii\\chapter_02\\Cruiser_E"
            && !self.chancellor_room6_split
        {
            self.chancellor_room6_split = true;
            return true;
        } // room5->6 (door_shaft_out3 Normal or WW - both land here)
        if settings.room
            && current.levelpath.as_bytes() == b"levels\\episode_iii\\chapter_02\\Cruiser_F"
            && old.levelpath.as_bytes() != b"levels\\episode_iii\\chapter_02\\Cruiser_F"
            && !self.chancellor_room7_split
        {
            self.chancellor_room7_split = true;
            return true;
        } // room6->7 (door_main_hanger Normal or door_shaft_out3 WW - both land here)

        // ----- Kashyyyk -----
        if settings.room && current.door.as_bytes() == b"door_to_beach" && old.door != current.door
        {
            return true;
        }
        if settings.room && current.door.as_bytes() == b"door_to_swamp" && old.door != current.door
        {
            return true;
        }
        if settings.room
            && current.door.as_bytes() == b"door_to_forest"
            && old.door != current.door
            && !self.kashyyyk_forest_split
        {
            self.kashyyyk_forest_split = true;
            return true;
        }

        // ----- Ruin -----
        if settings.room
            && current.door.as_bytes() == b"door_to_inside"
            && old.door != current.door
            && !self.ruin_inside_split
        {
            self.ruin_inside_split = true;
            return true;
        } // room1->2
        if settings.room
            && current.levelpath.as_bytes() == b"levels\\episode_iii\\chapter_06\\Temple_C"
            && old.levelpath.as_bytes() != b"levels\\episode_iii\\chapter_06\\Temple_C"
        {
            return true;
        } // room2->3 (Normal or WW - both land here)
        if settings.room
            && current.door.as_bytes() == b"door_to_control"
            && old.door != current.door
        {
            return true;
        }

        // ----- Vader -----
        if settings.room
            && current.levelpath.as_bytes() == b"levels\\episode_iii\\chapter_07\\Vader_B"
            && old.levelpath.as_bytes() != b"levels\\episode_iii\\chapter_07\\Vader_B"
        {
            return true;
        } // room A->B (door is corrupted - starts with a literal null byte in-game, so it always resolves to "" and can never match either spelling)
        if settings.room
            && current.levelpath.as_bytes() == b"levels\\episode_iii\\chapter_07\\Vader_C"
            && old.levelpath.as_bytes() != b"levels\\episode_iii\\chapter_07\\Vader_C"
        {
            return true;
        } // room B->C (no door, levelpath-only transition)
        if settings.room && current.door.as_bytes() == b"door_fight" && old.door != current.door {
            return true;
        }

        // ----- Maul -----
        if settings.room && current.door.as_bytes() == b"door_2" && old.door != current.door {
            return true;
        }
        if settings.room
            && current.door.as_bytes() == b"door_3"
            && old.door != current.door
            && !self.maul_room3_split
        {
            self.maul_room3_split = true;
            return true;
        } // room2->3
        if settings.room
            && old.door.as_bytes() == b"door_3"
            && current.door != old.door
            && self.maul_ww_levelpath_leeway > 0
        {
            return true;
        } // room3->4 WW
        if settings.room
            && current.levelpath.as_bytes() == b"levels\\episode_i\\chapter_07\\Maul_E"
            && old.levelpath.as_bytes() != b"levels\\episode_i\\chapter_07\\Maul_E"
            && self.maul_ww_door_leeway > 0
        {
            return true;
        } // room3->4 WW (reverse order)
        if settings.room && current.door.as_bytes() == b"door_17" && old.door != current.door {
            return true;
        } // room3->4 Normal
        if settings.room && current.door.as_bytes() == b"door_8" && old.door != current.door {
            return true;
        }

        // ----- End-of-run outros (settings-gated, not tied to the "room" toggle) -----
        if settings.maul && current.door.as_bytes() == b"door_8" && current.cutscene == 1 {
            return true;
        }
        if settings.vader && current.door.as_bytes() == b"door_fight" && current.cutscene == 1 {
            return true;
        }
        if settings.anh && current.door.as_bytes() == b"door_from_room2" && current.cutscene == 1 {
            return true;
        }
        if settings.dooku
            && current.levelpath.as_bytes() == b"levels\\episode_ii\\chapter_07\\Dooku_Outro"
        {
            return true;
        }
        if settings.rushend
            && current.door.as_bytes() == b"GunshipA_B"
            && current.cutscene == 1
            && self.gunship_outro_leeway == 0
        {
            return true;
        }
        false
    }

    pub(crate) fn should_reset(
        &mut self,
        settings: &Settings,
        old: &State,
        current: &State,
    ) -> bool {
        if settings.resetonmenu
            && current.levelpath.as_bytes() == b"levels\\titles"
            && old.levelpath.as_bytes() != b"levels\\titles"
            && self.diner_leeway > 0
        {
            self.diner_leeway = 0;
            return true;
        }
        false
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
