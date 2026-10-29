state("Dolphin") {}

startup {
    Assembly.Load(File.ReadAllBytes("Components/emu-help-v2")).CreateInstance("GCN");

    settings.Add("newgame", true, "Start on New Game");
    settings.Add("maul", true, "End on Darth Maul Outro (Any%)");
    settings.Add("room", false, "Split on every room (Any%)");
    settings.Add("vader", false, "End on Darth Vader Outro (All Minikits)");
    settings.Add("anh", false, "End on A New Hope Outro (100% NSR)");
}

init {
    vars.Helper.Load = (Func<dynamic, bool>)(emu => {
        emu.Make<byte>("status", 0x802E1CB8);
        emu.Make<int>("gamestate", 0x804093B0);
        emu.Make<byte>("cutscene", 0x801BB45D);
        emu.MakeString("door", 16, 0x801DC964);
        return true;
    });
}

start {
    if (settings["newgame"] && old.gamestate == -1 && current.gamestate == 123) return true;
}

split {
    if (old.status == 0 && current.status == 1 && current.door != "door_A" && current.door != "door_B" && !(current.door == "" && (old.door == "door_A" || old.door == "door_B"))) return true;
    if (settings["room"] && current.door == "door_to_hangar" && old.door != current.door) return true;
    if (settings["room"] && current.door == "door_into_ruins1" && old.door != current.door) return true;
    if (settings["room"] && current.door == "door_into_ruins2" && old.door != current.door) return true;
    if (settings["room"] && old.door == "door_into_ruins2" && current.door != old.door) return true;
    if (settings["room"] && current.door == "RescueA_RescueB" && old.door != current.door) return true;
    if (settings["room"] && current.door == "RescueB_RescueC" && old.door != current.door) return true;
    if (settings["room"] && current.door == "RescueC_RescueE" && old.door != current.door) return true;
    if (settings["room"] && current.door == "retakeA_retakeB" && old.door != current.door) return true;
    if (settings["room"] && current.door == "retakeB_retakeD" && old.door != current.door) return true;
    if (settings["room"] && current.door == "retakeD_retakeE" && old.door != current.door) return true;
    if (settings["room"] && current.door == "retakeE_retakeF" && old.door != current.door) return true;
    if (settings["room"] && current.door == "Palace_Hangar" && old.door != current.door) return true;
    if (settings["room"] && current.door == "door1in" && old.door != current.door) return true;
    if (settings["room"] && current.door == "door10out" && old.door != current.door) return true;
    if (settings["room"] && old.door == "door10out" && current.door != old.door) return true;
    if (settings["room"] && current.door == "door2corr" && old.door != current.door) return true;
    if (settings["room"] && current.door == "door_bossout" && old.door != current.door) return true;
    if (settings["room"] && current.door == "door_4" && old.door != current.door) return true;
    if (settings["room"] && current.door == "door_3in" && old.door != current.door) return true;
    if (settings["room"] && current.door == "door_5in" && old.door != current.door) return true;
    if (settings["room"] && current.door == "door_6in" && old.door != current.door) return true;
    if (settings["room"] && current.door == "door_8out" && old.door != current.door) return true;
    if (settings["room"] && current.door == "GunshipA_B" && old.door != current.door) return true;
    if (settings["room"] && current.door == "door_main_in" && old.door != current.door) return true;
    if (settings["room"] && current.door == "door_grev" && old.door != current.door) return true;
    if (settings["room"] && old.door == "door_vent2" && current.door != old.door) return true;
    if (settings["room"] && current.door == "door_lobby_out2" && old.door != current.door) return true;
    if (settings["room"] && old.door == "door_shaft_out3" && current.door != old.door) return true;
    if (settings["room"] && current.door == "door_to_beach" && old.door != current.door) return true;
    if (settings["room"] && current.door == "door_to_swamp" && old.door != current.door) return true;
    if (settings["room"] && current.door == "door_to_forest" && old.door != current.door) return true;
    if (settings["room"] && current.door == "door_to_inside" && old.door != current.door) return true;
    if (settings["room"] && current.door == "door_to_archive" && old.door != current.door) return true;
    if (settings["room"] && current.door == "door_to_control" && old.door != current.door) return true;
    if (settings["room"] && current.door == "door_fight" && old.door != current.door) return true;
    if (settings["room"] && current.door == "door_2" && old.door != current.door) return true;
    if (settings["room"] && current.door == "door_3" && old.door != current.door) return true;
    if (settings["room"] && old.door == "door_3" && current.door != old.door) return true;
    if (settings["room"] && current.door == "door_8" && old.door != current.door) return true;
    if (settings["maul"] && current.door == "door_8" && current.cutscene == 1) return true;
    if (settings["vader"] && current.door == "door_fight" && current.cutscene == 1) return true;
    if (settings["anh"] && current.door == "door_from_room2" && current.cutscene == 1) return true;
}

reset {
    return current.gamestate == 124;
}

exit {
    timer.IsGameTimePaused = true;
}