state("LegoStarwars")
{
    byte status : 0x39C58D;
    byte start : 0x39B0D0;
    byte title : 0x37B618;
    byte cutscene : 0x38D04C;
    string16 door : 0x39B0E0;
    string64 levelpath : 0x37220C, 0x0;
    byte currscene : 0x371444;
}

startup
{
    settings.Add("newgame", true, "Start on New Game");
    settings.Add("freeplay", false, "Start on Level Entry (Free Play)");
    settings.Add("rushstart", false, "Start After Podrace Countdown (Minikit Rush/True Jedi Rush)");
    settings.Add("maul", true, "End on Darth Maul Outro (Any%)");
    settings.Add("vader", false, "End on Darth Vader Outro (All Minikits)");
    settings.Add("anh", false, "End on A New Hope Outro (All Levels/100% NSR)");
    settings.Add("dooku", false, "End on Count Dooku Outro (NoCut Any%)");
    settings.Add("rushend", false, "End on Gunship Outro (Minikit Rush/True Jedi Rush)");
    settings.Add("room", false, "Split on every room in Any%");
    settings.Add("resetonmenu", true, "Automatically Reset Timer upon Exiting to Title Screen");
    refreshRate = 60;
}

init
{
    vars.dinerLeeway = 0;
    vars.ruinInsideSplit = false;
    vars.maulRoom3Split = false;
    vars.maulWWDoorLeeway = 0;
    vars.maulWWLevelpathLeeway = 0;
    vars.podraceLapCount = 0;
    vars.rushStartLeeway = 0;
    vars.gunshipOutroLeeway = 0;
    vars.chancellorRoom2Split = false;
    vars.chancellorRoom3Split = false;
    vars.chancellorRoom4Split = false;
    vars.chancellorRoom5Split = false;
    vars.chancellorRoom6Split = false;
    vars.chancellorRoom7Split = false;
    vars.negotiationsHangarSplit = false;
    vars.theedRetakeBDSplit = false;
    vars.theedRetakeDESplit = false;
    vars.kaminoDoor10outSplit = false;
    vars.factoryDoor8outSplit = false;
    vars.kashyyykForestSplit = false;
}

update
{
    if (current.levelpath == "levels\\Map\\Map_E")
    {
        vars.dinerLeeway = 60; // ~1 second of leeway at refreshRate 60, counts down every tick you're not in the diner
    }
    else if (vars.dinerLeeway > 0)
    {
        vars.dinerLeeway--;
    }

    if (!current.levelpath.StartsWith("levels\\episode_iii\\chapter_06\\"))
    {
        vars.ruinInsideSplit = false; // only re-arms once you leave Ruin entirely, not just by backtracking between its rooms
    }

    // Maul room2->3 entry (door_3): latch it so backing out of a failed room3->4 WW and re-entering room 3
    // doesn't cause a second split. Only re-arms once you leave Maul's level entirely.
    if (!current.levelpath.StartsWith("levels\\episode_i\\chapter_07\\"))
    {
        vars.maulRoom3Split = false;
    }

    // Maul room3->4 WW: door clears away from "door_3" (which also happens if you just back out to room2
    // without WWing) and levelpath becomes Maul_E on different ticks, so track a short leeway window from
    // each edge and only trust the door-clear as a split when the Maul_E transition actually shows up too.
    if (old.door == "door_3" && current.door != old.door)
    {
        vars.maulWWDoorLeeway = 30;
    }
    else if (vars.maulWWDoorLeeway > 0)
    {
        vars.maulWWDoorLeeway--;
    }

    // Podrace lap count: only laps 1->2 and 2->3 should split, not the lap3->finish crossing
    // (that one just rolls straight into the outro, which can ride along with the lap 3 split).
    // Resets once you leave Podrace entirely, so a retry/reset re-arms it.
    if (!current.levelpath.StartsWith("levels\\episode_i\\chapter_04\\"))
    {
        vars.podraceLapCount = 0;
    }

    // Rush categories (Minikit Rush/True Jedi Rush) start delay: these categories usually start on Podrace, and the
    // timer should start the moment you actually gain control (after the 3-second countdown), not on
    // level load. Arm a 4-second (120 tick @ 30/60fps) countdown the instant you land in PodRace_B (the race
    // track room itself), and the start block below fires once that countdown runs out.
    if (settings["rushstart"] && current.levelpath == "levels\\episode_i\\chapter_04\\PodRace_B" && old.levelpath != "levels\\episode_i\\chapter_04\\PodRace_B")
    {
        vars.rushStartLeeway = 120;
    }
    else if (vars.rushStartLeeway > 0)
    {
        vars.rushStartLeeway--;
    }

    // Gunship outro end delay: entering Gunship_B (door == "GunshipA_B") triggers a short midtro cutscene
    // of its own (cutscene == 1 briefly), which would otherwise false-split the rush end check below.
    // Arm a 10-second (600 tick @ 60fps) grace period the instant that door is touched, and only trust
    // the cutscene flag for the real outro once that period has fully run out.
    if (settings["rushend"] && current.door == "GunshipA_B" && old.door != current.door)
    {
        vars.gunshipOutroLeeway = 600;
    }
    else if (vars.gunshipOutroLeeway > 0)
    {
        vars.gunshipOutroLeeway--;
    }

    if (current.levelpath == "levels\\episode_i\\chapter_07\\Maul_E" && old.levelpath != "levels\\episode_i\\chapter_07\\Maul_E")
    {
        vars.maulWWLevelpathLeeway = 30;
    }
    else if (vars.maulWWLevelpathLeeway > 0)
    {
        vars.maulWWLevelpathLeeway--;
    }

    // Chancellor latches: reset only once you leave the level entirely, so backtracking within
    // Chancellor (very easy to do given how fast/WW-heavy this level's route is) can't double-split.
    if (!current.levelpath.StartsWith("levels\\episode_iii\\chapter_02\\"))
    {
        vars.chancellorRoom2Split = false;
        vars.chancellorRoom3Split = false;
        vars.chancellorRoom4Split = false;
        vars.chancellorRoom5Split = false;
        vars.chancellorRoom6Split = false;
        vars.chancellorRoom7Split = false;
    }

    // Single-split latches for a few other doors: reset at the start of every fresh attempt
    // (same new-game edge the "start" block uses), rather than a level-exit check.
    if (current.start > old.start && current.title == 1 && current.currscene == 124)
    {
        vars.negotiationsHangarSplit = false;
        vars.theedRetakeBDSplit = false;
        vars.theedRetakeDESplit = false;
        vars.kaminoDoor10outSplit = false;
        vars.factoryDoor8outSplit = false;
        vars.kashyyykForestSplit = false;
    }
}

split
{
    // ----- Level Complete (generic, applies across levels) -----
    if (current.status == 18 && old.status == 0) return true;
    if (current.status == 18 && old.status == 5) return true; // kamino freeplay fix

    // ----- Negotiations -----
    if (settings["room"] && current.door == "door_to_hangar" && old.door != current.door && !vars.negotiationsHangarSplit) { vars.negotiationsHangarSplit = true; return true; }

    // ----- Invasion -----
    if (settings["room"] && current.door == "door_into_ruins1" && old.door != current.door) return true;
    if (settings["room"] && current.door == "door_into_ruins2" && old.door != current.door) return true;
    if (settings["room"] && old.door == "door_into_ruins2" && current.door != old.door) return true; // WW
    if (settings["room"] && current.door == "door_swamp" && old.door != current.door) return true; // Normal

    // ----- Escape -----
    if (settings["room"] && current.door == "RescueA_RescueB" && old.door != current.door) return true;
    if (settings["room"] && current.door == "RescueB_RescueC" && old.door != current.door) return true;
    if (settings["room"] && current.door == "RescueC_RescueE" && old.door != current.door) return true;

    // ----- Podrace -----
    if (settings["room"] && current.door == "door_B" && old.door != current.door)
    {
        vars.podraceLapCount++;
        if (vars.podraceLapCount <= 2) return true; // only laps 1->2 and 2->3; lap3->finish rolls into the outro
    }

    // ----- Theed -----
    if (settings["room"] && current.door == "retakeA_retakeB" && old.door != current.door) return true;
    if (settings["room"] && current.door == "retakeB_retakeD" && old.door != current.door && !vars.theedRetakeBDSplit) { vars.theedRetakeBDSplit = true; return true; }
    if (settings["room"] && current.door == "retakeD_retakeE" && old.door != current.door && !vars.theedRetakeDESplit) { vars.theedRetakeDESplit = true; return true; }
    if (settings["room"] && current.door == "retakeE_retakeF" && old.door != current.door) return true;
    if (settings["room"] && current.door == "Palace_Hangar" && old.door != current.door) return true;

    // ----- Kamino -----
    if (settings["room"] && current.door == "door1in" && old.door != current.door) return true;
    if (settings["room"] && current.door == "door10out" && old.door != current.door && !vars.kaminoDoor10outSplit) { vars.kaminoDoor10outSplit = true; return true; }
    if (settings["room"] && old.door == "door10out" && current.door != old.door) return true;
    if (settings["room"] && current.door == "door2corr" && old.door != current.door) return true;
    if (settings["room"] && current.door == "door_bossout" && old.door != current.door) return true;

    // ----- Factory -----
    if (settings["room"] && current.door == "door_4" && old.door != current.door) return true;
    if (settings["room"] && current.door == "door_3in" && old.door != current.door) return true;
    if (settings["room"] && current.door == "door_5in" && old.door != current.door) return true;
    if (settings["room"] && current.door == "door_6in" && old.door != current.door) return true;
    if (settings["room"] && current.door == "door_8out" && old.door != current.door && !vars.factoryDoor8outSplit) { vars.factoryDoor8outSplit = true; return true; }

    // ----- Gunship -----
    if (settings["room"] && current.door == "GunshipA_mid" && old.door != current.door) return true; // room1 part2
    if (settings["room"] && current.door == "GunshipA_B" && old.door != current.door) return true;

    // ----- Dooku -----
    if (settings["room"] && current.door == "door_main_in" && old.door != current.door) return true;

    // ----- Chancellor -----
    if (settings["room"] && current.door == "door_grev" && old.door != current.door && !vars.chancellorRoom2Split) { vars.chancellorRoom2Split = true; return true; } // room1->2
    if (settings["room"] && current.levelpath == "levels\\episode_iii\\chapter_02\\Cruiser_B" && old.levelpath != "levels\\episode_iii\\chapter_02\\Cruiser_B" && !vars.chancellorRoom3Split) { vars.chancellorRoom3Split = true; return true; } // room2->3 (door_vent1 or door_vent2, Normal or WW - both land here)
    if (settings["room"] && current.levelpath == "levels\\episode_iii\\chapter_02\\Cruiser_C" && old.levelpath != "levels\\episode_iii\\chapter_02\\Cruiser_C" && !vars.chancellorRoom4Split) { vars.chancellorRoom4Split = true; return true; } // room3->4 (door_vent1/vent2 WW or door_shaft_out2 Normal - both land here)
    if (settings["room"] && current.levelpath == "levels\\episode_iii\\chapter_02\\Cruiser_D" && old.levelpath != "levels\\episode_iii\\chapter_02\\Cruiser_D" && !vars.chancellorRoom5Split) { vars.chancellorRoom5Split = true; return true; } // room4->5 (door_lobby_out2 Normal or door_shaft_out2 WW - both land here)
    if (settings["room"] && current.levelpath == "levels\\episode_iii\\chapter_02\\Cruiser_E" && old.levelpath != "levels\\episode_iii\\chapter_02\\Cruiser_E" && !vars.chancellorRoom6Split) { vars.chancellorRoom6Split = true; return true; } // room5->6 (door_shaft_out3 Normal or WW - both land here)
    if (settings["room"] && current.levelpath == "levels\\episode_iii\\chapter_02\\Cruiser_F" && old.levelpath != "levels\\episode_iii\\chapter_02\\Cruiser_F" && !vars.chancellorRoom7Split) { vars.chancellorRoom7Split = true; return true; } // room6->7 (door_main_hanger Normal or door_shaft_out3 WW - both land here)

    // ----- Kashyyyk -----
    if (settings["room"] && current.door == "door_to_beach" && old.door != current.door) return true;
    if (settings["room"] && current.door == "door_to_swamp" && old.door != current.door) return true;
    if (settings["room"] && current.door == "door_to_forest" && old.door != current.door && !vars.kashyyykForestSplit) { vars.kashyyykForestSplit = true; return true; }

    // ----- Ruin -----
    if (settings["room"] && current.door == "door_to_inside" && old.door != current.door && !vars.ruinInsideSplit) { vars.ruinInsideSplit = true; return true; } // room1->2
    if (settings["room"] && current.levelpath == "levels\\episode_iii\\chapter_06\\Temple_C" && old.levelpath != "levels\\episode_iii\\chapter_06\\Temple_C") return true; // room2->3 (Normal or WW - both land here)
    if (settings["room"] && current.door == "door_to_control" && old.door != current.door) return true;

    // ----- Vader -----
    if (settings["room"] && current.levelpath == "levels\\episode_iii\\chapter_07\\Vader_B" && old.levelpath != "levels\\episode_iii\\chapter_07\\Vader_B") return true; // room A->B (door is corrupted - starts with a literal null byte in-game, so it always resolves to "" and can never match either spelling)
    if (settings["room"] && current.levelpath == "levels\\episode_iii\\chapter_07\\Vader_C" && old.levelpath != "levels\\episode_iii\\chapter_07\\Vader_C") return true; // room B->C (no door, levelpath-only transition)
    if (settings["room"] && current.door == "door_fight" && old.door != current.door) return true;

    // ----- Maul -----
    if (settings["room"] && current.door == "door_2" && old.door != current.door) return true;
    if (settings["room"] && current.door == "door_3" && old.door != current.door && !vars.maulRoom3Split) { vars.maulRoom3Split = true; return true; } // room2->3
    if (settings["room"] && old.door == "door_3" && current.door != old.door && vars.maulWWLevelpathLeeway > 0) return true; // room3->4 WW
    if (settings["room"] && current.levelpath == "levels\\episode_i\\chapter_07\\Maul_E" && old.levelpath != "levels\\episode_i\\chapter_07\\Maul_E" && vars.maulWWDoorLeeway > 0) return true; // room3->4 WW (reverse order)
    if (settings["room"] && current.door == "door_17" && old.door != current.door) return true; // room3->4 Normal
    if (settings["room"] && current.door == "door_8" && old.door != current.door) return true;

    // ----- End-of-run outros (settings-gated, not tied to the "room" toggle) -----
    if (settings["maul"] && current.door == "door_8" && current.cutscene == 1) return true;
    if (settings["vader"] && current.door == "door_fight" && current.cutscene == 1) return true;
    if (settings["anh"] && current.door == "door_from_room2" && current.cutscene == 1) return true;
    if (settings["dooku"] && current.levelpath == "levels\\episode_ii\\chapter_07\\Dooku_Outro") return true;
    if (settings["rushend"] && current.door == "GunshipA_B" && current.cutscene == 1 && vars.gunshipOutroLeeway == 0) return true;
}

start
{
    if (settings["newgame"] && current.start > old.start && current.title == 1 && current.currscene == 124) return true; // credit goes to coolestto
    if (settings["freeplay"] && current.door == "") return true;
    if (settings["rushstart"] && vars.rushStartLeeway == 1) return true;
}

reset
{
    if (settings["resetonmenu"] && current.levelpath == "levels\\titles" && old.levelpath != "levels\\titles" && vars.dinerLeeway > 0)
    {
        vars.dinerLeeway = 0;
        return true;
    }
}
