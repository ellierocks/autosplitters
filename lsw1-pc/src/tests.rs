use super::*;
use std::{string::String, vec::Vec};

fn text<const N: usize>(value: &str) -> ArrayCString<N> {
    assert!(value.len() <= N);
    let mut bytes = [0; N];
    bytes[..value.len()].copy_from_slice(value.as_bytes());
    bytemuck::cast(bytes)
}

fn at(door: &str, path: &str) -> State {
    State {
        door: text(door),
        levelpath: text(path),
        ..State::default()
    }
}

fn rooms() -> Settings {
    Settings {
        room: true,
        maul: false,
        ..Settings::default()
    }
}

fn split(engine: &mut Engine, settings: &Settings, old: &State, current: &State) -> bool {
    engine.update(settings, old, current);
    engine.should_split(settings, old, current)
}

#[test]
fn setting_ids_labels_and_defaults_match_joeys_asl() {
    let defaults = Settings::default();
    let actual = [
        ("newgame", defaults.newgame),
        ("freeplay", defaults.freeplay),
        ("rushstart", defaults.rushstart),
        ("maul", defaults.maul),
        ("vader", defaults.vader),
        ("anh", defaults.anh),
        ("dooku", defaults.dooku),
        ("rushend", defaults.rushend),
        ("room", defaults.room),
        ("resetonmenu", defaults.resetonmenu),
    ];
    let settings_source = include_str!("logic.rs")
        .split("impl Default for Settings")
        .next()
        .unwrap();
    let asl: Vec<_> = include_str!("../joey.asl")
        .lines()
        .filter(|line| line.contains("settings.Add("))
        .collect();
    assert_eq!(actual.len(), asl.len());
    for ((id, default), line) in actual.into_iter().zip(asl) {
        let strings: Vec<_> = line.split('"').collect();
        assert_eq!(id, strings[1]);
        assert_eq!(default, strings[2].contains("true"));
        assert!(settings_source.contains(&std::format!("/// {}\n", strings[3])));
        assert!(settings_source.contains(&std::format!(
            "#[default = {default}]\n    pub(crate) {id}: bool"
        )));
    }
}

#[test]
fn every_stateless_asl_door_entry_and_exit_is_ported_and_setting_gated() {
    let asl = include_str!("../joey.asl")
        .split("\nsplit\n")
        .nth(1)
        .unwrap()
        .split("\nstart\n")
        .next()
        .unwrap();
    let mut checked = 0;
    for line in asl.lines().filter(|line| {
        line.contains("settings[\"room\"]")
            && !line.contains("vars.")
            && !line.contains("levelpath")
            && line.contains("return true;")
    }) {
        let entering = line.contains("current.door == \"");
        let marker = if entering {
            "current.door == \""
        } else {
            "old.door == \""
        };
        let name = line
            .split(marker)
            .nth(1)
            .unwrap()
            .split('"')
            .next()
            .unwrap();
        let empty = at("", "");
        let named = at(name, "");
        let (old, current) = if entering {
            (&empty, &named)
        } else {
            (&named, &empty)
        };
        assert!(
            split(&mut Engine::default(), &rooms(), old, current),
            "{line}"
        );
        assert!(
            !split(
                &mut Engine::default(),
                &Settings {
                    room: false,
                    ..rooms()
                },
                old,
                current
            ),
            "disabled: {line}"
        );
        assert!(
            !split(&mut Engine::default(), &rooms(), current, current),
            "no edge: {line}"
        );
        checked += 1;
    }
    assert_eq!(
        checked, 28,
        "review parity expectations when Joey's rules change"
    );
}

#[test]
fn unknown_completion_status_cannot_create_a_split() {
    let settings = Settings::default();
    for (old, current) in [
        (None, None),
        (None, Some(18)),
        (Some(0), None),
        (Some(5), None),
    ] {
        assert!(!split(
            &mut Engine::default(),
            &settings,
            &State {
                status: old,
                ..State::default()
            },
            &State {
                status: current,
                ..State::default()
            },
        ));
    }
}

#[test]
fn generic_completion_is_unconditional_and_precedes_room_latches() {
    for previous in [0, 5] {
        let old = State {
            status: Some(previous),
            ..State::default()
        };
        let current = State {
            status: Some(18),
            ..at("door_to_hangar", "")
        };
        let mut engine = Engine::default();
        assert!(split(&mut engine, &rooms(), &old, &current));
        assert!(!engine.negotiations_hangar_split);
        assert!(split(
            &mut Engine::default(),
            &Settings::default(),
            &old,
            &current
        ));
    }
    for previous in [1, 17, 18, 19, 255] {
        assert!(!split(
            &mut Engine::default(),
            &Settings::default(),
            &State {
                status: Some(previous),
                ..State::default()
            },
            &State {
                status: Some(18),
                ..State::default()
            }
        ));
    }
}

#[test]
fn fresh_new_game_rearms_attempt_latches_even_with_newgame_disabled() {
    let settings = Settings {
        newgame: false,
        ..rooms()
    };
    for door in [
        "door_to_hangar",
        "retakeB_retakeD",
        "retakeD_retakeE",
        "door10out",
        "door_8out",
        "door_to_forest",
    ] {
        let mut engine = Engine::default();
        let empty = State::default();
        let current = at(door, "");
        assert!(split(&mut engine, &settings, &empty, &current), "{door}");
        assert!(
            !split(&mut engine, &settings, &empty, &current),
            "latched: {door}"
        );
        engine.update(&settings, &current, &at("", "levels\\titles"));
        assert!(
            !split(&mut engine, &settings, &empty, &current),
            "level exit: {door}"
        );
        let new_game = State {
            start: 1,
            title: 1,
            currscene: 124,
            ..empty
        };
        engine.update(&settings, &empty, &new_game);
        assert!(
            split(&mut engine, &settings, &empty, &current),
            "new game: {door}"
        );
    }
}

#[test]
fn level_latches_survive_backtracking_and_rearm_only_after_leaving_level() {
    for (door, path) in [
        (
            "door_to_inside",
            "levels\\episode_iii\\chapter_06\\Temple_B",
        ),
        ("door_3", "levels\\episode_i\\chapter_07\\Maul_C"),
        ("door_grev", "levels\\episode_iii\\chapter_02\\Cruiser_A"),
    ] {
        let mut engine = Engine::default();
        let old = at("", path);
        let current = at(door, path);
        assert!(split(&mut engine, &rooms(), &old, &current));
        assert!(!split(&mut engine, &rooms(), &old, &current));
        engine.update(&rooms(), &current, &at("", "levels\\Map\\Map_E"));
        assert!(split(&mut engine, &rooms(), &old, &current));
    }
    for room in ['B', 'C', 'D', 'E', 'F'] {
        let mut engine = Engine::default();
        let old = at("", "levels\\episode_iii\\chapter_02\\Cruiser_A");
        let path = std::format!("levels\\episode_iii\\chapter_02\\Cruiser_{room}");
        let current = at("", &path);
        assert!(split(&mut engine, &rooms(), &old, &current), "{path}");
        assert!(!split(&mut engine, &rooms(), &old, &current), "{path}");
        engine.update(&rooms(), &current, &State::default());
        assert!(split(&mut engine, &rooms(), &old, &current), "{path}");
    }
}

#[test]
fn path_only_rooms_work_with_empty_corrupted_door_strings() {
    for path in [
        "levels\\episode_iii\\chapter_06\\Temple_C",
        "levels\\episode_iii\\chapter_07\\Vader_B",
        "levels\\episode_iii\\chapter_07\\Vader_C",
    ] {
        let current = at("", path);
        assert!(split(
            &mut Engine::default(),
            &rooms(),
            &State::default(),
            &current
        ));
        assert!(!split(&mut Engine::default(), &rooms(), &current, &current));
    }
}

#[test]
fn podrace_only_splits_first_two_lap_crossings_and_rearms_after_level_exit() {
    let mut engine = Engine::default();
    let empty = at("", "levels\\episode_i\\chapter_04\\PodRace_B");
    let lap = at("door_B", "levels\\episode_i\\chapter_04\\PodRace_B");
    for expected in [true, true, false, false] {
        assert_eq!(split(&mut engine, &rooms(), &empty, &lap), expected);
    }
    engine.update(&rooms(), &lap, &State::default());
    assert!(split(&mut engine, &rooms(), &empty, &lap));
}

#[test]
fn maul_wall_warp_needs_both_edges_and_accepts_either_order() {
    let before = at("door_3", "levels\\episode_i\\chapter_07\\Maul_D");
    let door_cleared = at("", "levels\\episode_i\\chapter_07\\Maul_D");
    let path_changed = at("door_3", "levels\\episode_i\\chapter_07\\Maul_E");
    let after = at("", "levels\\episode_i\\chapter_07\\Maul_E");
    for middle in [door_cleared, path_changed] {
        let mut engine = Engine::default();
        assert!(!split(&mut engine, &rooms(), &before, &middle));
        for _ in 0..28 {
            assert!(!split(&mut engine, &rooms(), &middle, &middle));
        }
        assert!(split(&mut engine, &rooms(), &middle, &after));
    }
    for middle in [door_cleared, path_changed] {
        let mut engine = Engine::default();
        assert!(!split(&mut engine, &rooms(), &before, &middle));
        for _ in 0..29 {
            engine.update(&rooms(), &middle, &middle);
        }
        assert!(
            !split(&mut engine, &rooms(), &middle, &after),
            "30-tick window expired"
        );
    }
}

#[test]
fn new_game_requires_rising_start_byte_title_and_scene_124() {
    let settings = Settings::default();
    let engine = Engine::default();
    let old = State::default();
    let current = State {
        start: 1,
        title: 1,
        currscene: 124,
        ..old
    };
    assert!(engine.should_start(&settings, &old, &current));
    assert!(!engine.should_start(&settings, &current, &current));
    assert!(!engine.should_start(&settings, &current, &old));
    assert!(!engine.should_start(
        &settings,
        &old,
        &State {
            title: 0,
            ..current
        }
    ));
    assert!(!engine.should_start(
        &settings,
        &old,
        &State {
            currscene: 123,
            ..current
        }
    ));
    assert!(!engine.should_start(
        &Settings {
            newgame: false,
            ..settings
        },
        &old,
        &current
    ));
}

#[test]
fn freeplay_preserves_asl_empty_door_predicate_without_an_edge() {
    let settings = Settings {
        freeplay: true,
        ..Settings::default()
    };
    let current = State::default();
    assert!(Engine::default().should_start(&settings, &current, &current));
    assert!(!Engine::default().should_start(&settings, &current, &at("door_x", "")));
}

#[test]
fn rush_start_uses_119_successful_updates_after_entry() {
    let settings = Settings {
        rushstart: true,
        ..Settings::default()
    };
    let old = State::default();
    let current = at("", "levels\\episode_i\\chapter_04\\PodRace_B");
    let mut engine = Engine::default();
    engine.update(&settings, &old, &current);
    assert!(!engine.should_start(&settings, &old, &current));
    for _ in 0..118 {
        engine.update(&settings, &current, &current);
        assert!(!engine.should_start(&settings, &current, &current));
    }
    engine.update(&settings, &current, &current);
    assert!(engine.should_start(&settings, &current, &current));
    engine.update(&settings, &current, &current);
    assert!(!engine.should_start(&settings, &current, &current));
}

#[test]
fn gunship_grace_period_filters_midtro_for_600_ticks() {
    let settings = Settings {
        rushend: true,
        ..Settings::default()
    };
    let old = State::default();
    let current = State {
        cutscene: 1,
        ..at("GunshipA_B", "")
    };
    let mut engine = Engine::default();
    assert!(!split(&mut engine, &settings, &old, &current));
    for _ in 0..599 {
        assert!(!split(&mut engine, &settings, &current, &current));
    }
    assert!(split(&mut engine, &settings, &current, &current));
    // ASL leaves outro predicates true on subsequent samples.
    assert!(split(&mut engine, &settings, &current, &current));
}

#[test]
fn outros_preserve_settings_and_continuous_predicates() {
    for (id, door, path) in [
        ("maul", "door_8", ""),
        ("vader", "door_fight", ""),
        ("anh", "door_from_room2", ""),
        ("dooku", "", "levels\\episode_ii\\chapter_07\\Dooku_Outro"),
    ] {
        let mut settings = Settings {
            maul: false,
            ..Settings::default()
        };
        let current = State {
            cutscene: 1,
            ..at(door, path)
        };
        assert!(!split(
            &mut Engine::default(),
            &settings,
            &current,
            &current
        ));
        match id {
            "maul" => settings.maul = true,
            "vader" => settings.vader = true,
            "anh" => settings.anh = true,
            "dooku" => settings.dooku = true,
            _ => unreachable!(),
        }
        assert!(
            split(&mut Engine::default(), &settings, &current, &current),
            "{id}"
        );
        if id != "dooku" {
            let no_cutscene = State {
                cutscene: 0,
                ..current
            };
            assert!(!split(
                &mut Engine::default(),
                &settings,
                &current,
                &no_cutscene
            ));
        }
    }
}

#[test]
fn reset_only_fires_on_title_entry_within_diner_leeway_and_consumes_it() {
    let diner = at("", "levels\\Map\\Map_E");
    let elsewhere = at("", "levels\\other");
    let titles = at("", "levels\\titles");
    let settings = Settings::default();
    let mut engine = Engine::default();
    engine.update(&settings, &diner, &diner);
    assert!(!engine.should_reset(&settings, &titles, &titles));
    for _ in 0..58 {
        engine.update(&settings, &elsewhere, &elsewhere);
    }
    engine.update(&settings, &elsewhere, &titles);
    assert!(engine.should_reset(&settings, &elsewhere, &titles));
    assert!(!engine.should_reset(&settings, &elsewhere, &titles));
    engine.update(&settings, &titles, &diner);
    for _ in 0..59 {
        engine.update(&settings, &elsewhere, &elsewhere);
    }
    engine.update(&settings, &elsewhere, &titles);
    assert!(!engine.should_reset(&settings, &elsewhere, &titles));
    engine.update(&settings, &titles, &diner);
    engine.update(&settings, &diner, &titles);
    assert!(!engine.should_reset(
        &Settings {
            resetonmenu: false,
            ..settings
        },
        &diner,
        &titles
    ));
}

#[test]
fn all_asl_path_literals_fit_the_64_byte_watch_buffer() {
    for path in include_str!("../joey.asl")
        .split('"')
        .skip(1)
        .step_by(2)
        .filter(|s| s.starts_with("levels"))
    {
        let decoded: String = path.replace("\\\\", "\\");
        assert!(decoded.len() < 64, "{path}");
    }
}
