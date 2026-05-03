#![no_std]

use asr::{
    future::{next_tick, retry},
    settings::{gui::Title, Gui},
    time::Duration,
    time_util::Instant,
    timer::{self, TimerState},
    Address, Process,
};
use bytemuck::Pod;

asr::async_main!(stable);
asr::panic_handler!();

const GLOBAL_FRAME_COUNT: u64 = 0x0372_C6C8;
const OUTPUT_FRAME_COUNT: u64 = 0x036B_DF58;
const IN_CUTSCENE: u64 = 0x0372_C55C;
const GAME_STATUS: u64 = 0x0372_2DE4;
const GAME_MODE: u64 = 0x036B_DC7C;
const IN_PRERENDERED_MOVIE: u64 = 0x0368_B0D0;
const CURR_CHARACTER: u64 = 0x0372_2DC0;
const LAST_STORY: u64 = 0x0371_8DB4;
const TIMER_START: u64 = 0x0051_2DF0;
const DEMO_PLAYING: u64 = 0x0372_A2E4;
const BOSS_HEALTH: u64 = 0x0385_8150;
const LEVEL: u64 = 0x0372_2DCC;
const LEVEL_COPY: u64 = 0x0372_2DC4;
const ACT: u64 = 0x0372_2DEC;
const LIVES: u64 = 0x0370_EF34;
const E101_FLAG: u64 = 0x0371_89A8;
const ADVENTURE_DATA: u64 = 0x0371_83F0;
const RM_FLAG: u64 = 0x0371_89A5;
const MK2_VALUE: u64 = 0x0386_C760;
const WALKER_VALUE: u64 = 0x0385_A7ED;
const SELECTED_CHARACTER: u64 = 0x0372_A2FD;
const MUSIC: u64 = 0x0051_2698;
const LW_FLAG: u64 = 0x0371_892A;
const EMBLEM_BYTES: u64 = 0x0372_B5E8;
const EVENT_BYTES: u64 = 0x0371_8888;

const LIGHT_SHOES: u64 = 0x0371_8895;
const CRYSTAL_RING: u64 = 0x0371_8896;
const ANCIENT_LIGHT: u64 = 0x0371_88A7;
const JET_ANKLET: u64 = 0x0371_88D5;
const RHYTHM_BADGE: u64 = 0x0371_88E3;
const SHOVEL_CLAW: u64 = 0x0371_8921;
const FIGHTING_GLOVES: u64 = 0x0371_8922;
const WARRIOR_FEATHER: u64 = 0x0371_895A;
const LONG_HAMMER: u64 = 0x0371_8966;
const LIFE_BELT: u64 = 0x0371_89D8;
const POWER_ROD: u64 = 0x0371_89D9;
const LURE_EC: u64 = 0x0371_89E6;
const LURE_IC: u64 = 0x0371_89E7;
const LURE_SS: u64 = 0x0371_89E8;
const LURE_MR: u64 = 0x0371_89E9;
const JET_BOOSTER: u64 = 0x0371_8991;
const LASER_BLASTER: u64 = 0x0371_8992;

#[derive(Gui)]
struct Settings {
    /// General
    _general: Title,
    /// Auto Reset on Main Menu
    reset: bool,
    /// Splits when entering a story
    char_split: bool,
    /// ...But not when in trial
    char_split_ban_trial: bool,
    /// Don't start when in trial
    char_start_ban_trial: bool,

    /// Stages NOT To Split
    _stages: Title,
    /// Sky Chase Act 1
    #[default = true]
    ignore_sky_chase_1: bool,
    /// Sky Chase Act 2
    #[default = true]
    ignore_sky_chase_2: bool,

    /// Final Boss To Split
    _final_bosses: Title,
    /// Egg Viper
    #[default = true]
    egg_viper: bool,
    /// Egg Walker
    #[default = true]
    egg_walker: bool,
    /// Chaos 6
    #[default = true]
    chaos_6: bool,
    /// ZERO
    #[default = true]
    zero: bool,
    /// E-101 'Beta' mkII
    #[default = true]
    beta_mk2: bool,
    /// Perfect Chaos
    #[default = true]
    perfect_chaos: bool,

    /// Sonic Bosses
    _sonic_bosses: Title,
    /// Chaos 0
    #[default = true]
    zero_sonic: bool,
    /// Egg Hornet
    #[default = true]
    egg_hornet_sonic: bool,
    /// Knuckles
    #[default = true]
    knux_sonic: bool,
    /// Chaos 4
    #[default = true]
    four_sonic: bool,
    /// Gamma
    #[default = true]
    gamma_sonic: bool,
    /// Chaos 6
    #[default = true]
    six_sonic: bool,

    /// Tails Bosses
    _tails_bosses: Title,
    /// Egg Hornet
    #[default = true]
    egg_hornet_tails: bool,
    /// Knuckles
    #[default = true]
    knux_tails: bool,
    /// Chaos 4
    #[default = true]
    four_tails: bool,
    /// Gamma
    #[default = true]
    gamma_tails: bool,

    /// Knuckles Bosses
    _knuckles_bosses: Title,
    /// Chaos 2
    #[default = true]
    two_knuckles: bool,
    /// Sonic
    #[default = true]
    sonic_knuckles: bool,
    /// Chaos 4
    #[default = true]
    four_knuckles: bool,

    /// Gamma Bosses
    _gamma_bosses: Title,
    /// E101 BETA
    e101: bool,
    /// Sonic
    #[default = true]
    sonic_gamma: bool,

    /// Miscellaneous
    _global_misc: Title,
    /// Split between the Acts
    act_split: bool,

    /// Sonic Miscellaneous
    _sonic_misc: Title,
    /// Enter Sewers
    enter_sewer_sonic: bool,
    /// Enter Casino Area (To Red Mountain)
    enter_cas_hub_sonic: bool,
    /// Ship Transformation
    ship_transform: bool,
    /// Splits when going back to the present after Lost World
    back_from_past: bool,
    /// Enter Station Square from the Train Station
    enter_ss: bool,
    /// Enter Mystic Ruins from the Train Station
    enter_mr: bool,

    /// Knuckles Miscellaneous
    _knuckles_misc: Title,
    /// Death Warp Mystic Ruin
    death_mr_knux: bool,
    /// Enter Egg Carrier (Before Sky Deck)
    enter_eg_carrier_knux: bool,

    /// Amy Miscellaneous
    _amy_misc: Title,
    /// Enter Casino Area
    enter_cas_hub_amy: bool,
    /// Enter Jungle to Final Egg
    enter_jungle_to_fe_amy: bool,
    /// Exit Jungle from Final Egg
    exit_jungle_to_ec_amy: bool,
    /// Enter Egg Carrier (Before Zero)
    enter_eg_carrier_amy: bool,

    /// Gamma Miscellaneous
    _gamma_misc: Title,
    /// Death Mystic Ruin (Windy Valley Flag)
    death_wv: bool,
    /// Death Warp (Post Red Mountain)
    death_mr_gamma: bool,
    /// Enter Egg Carrier (Before MKII)
    egg_carrier_mk2: bool,

    /// Super Sonic Miscellaneous
    _super_sonic_misc: Title,
    /// Enter Cave
    enter_cave: bool,
    /// Angel Island Cutscene
    angel_island: bool,
    /// Tikal Cutscene
    tikal: bool,
    /// Death Warp post Tikal Cutscene
    death_ss: bool,
    /// Enter Jungle
    jungle_ss: bool,

    /// Enter Boss
    _boss_enter: Title,
    /// Enter Egg Hornet
    enter_egg_hornet: bool,
    /// Enter Chaos 2
    enter_chaos_2: bool,
    /// Enter Chaos 4
    enter_chaos_4: bool,
    /// Enter Chaos 6
    enter_chaos_6: bool,
    /// Enter E-101 'Beta' mkI
    enter_beta_1: bool,
    /// Enter E-101 'Beta' mkII
    enter_beta_2: bool,

    /// Enter Stage
    _stage_enter: Title,
    /// Enter Emerald Coast
    eec: bool,
    /// Enter Windy Valley
    ewv: bool,
    /// Enter Casino
    ec: bool,
    /// Enter Ice Cap
    eic: bool,
    /// Enter Twinkle Park
    etp: bool,
    /// Enter Speed Highway
    esh: bool,
    /// Enter Red Mountain
    erm: bool,
    /// Enter Sky Deck
    esd: bool,
    /// Enter Lost World
    elw: bool,
    /// Enter Final Egg
    efe: bool,
    /// Enter Hot Shelter
    ehs: bool,

    /// Upgrades
    _upgrades: Title,
    /// Light Speed Shoes
    sonic_powerup1: bool,
    /// Crystal Ring
    sonic_powerup2: bool,
    /// Ancient Light
    sonic_powerup3: bool,
    /// Jet Anklet
    tails_powerup1: bool,
    /// Rhythm Badge
    tails_powerup2: bool,
    /// Shovel Claw
    knux_powerup1: bool,
    /// Fighting Gloves
    knux_powerup2: bool,
    /// Warrior Feather
    amy_powerup1: bool,
    /// Long Hammer
    amy_powerup2: bool,
    /// Power Rod
    big_powerup1: bool,
    /// Life Belt
    big_powerup2: bool,
    /// Lure
    big_powerup3: bool,
    /// Jet Booster
    gamma_powerup1: bool,
    /// Laser Blaster
    gamma_powerup2: bool,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct GameState {
    global_frame_count: i32,
    output_frame_count: i32,
    in_cutscene: i32,
    game_status: u8,
    game_mode: u8,
    in_prerendered_movie: i32,
    curr_character: u8,
    last_story: u8,
    timer_start: u8,
    demo_playing: u8,
    boss_health: u8,
    level: u8,
    level_copy: u8,
    act: u8,
    lives: u8,
    e101_flag: u8,
    adventure_data: u8,
    rm_flag: u8,
    mk2_value: u8,
    walker_value: u8,
    selected_character: u8,
    music: u8,
    lw_flag: u8,
    emblem_bytes: [u8; 17],
    event_bytes: [u8; 353],
    lightshoes: u8,
    crystalring: u8,
    ancientlight: u8,
    jetanklet: u8,
    rhythmbadge: u8,
    shovelclaw: u8,
    fightinggloves: u8,
    warriorfeather: u8,
    longhammer: u8,
    lifebelt: u8,
    powerrod: u8,
    lureec: u8,
    lureic: u8,
    luress: u8,
    luremr: u8,
    jetbooster: u8,
    laserblaster: u8,
}

#[derive(Clone, Copy)]
struct RunState {
    total_frames: i64,
    prerendered_movie_rta: Duration,
    credits_counter: i32,
    boss_started: bool,
    delay: i32,
    last_tick: Instant,
}

impl RunState {
    fn new(now: Instant) -> Self {
        Self {
            total_frames: 0,
            prerendered_movie_rta: Duration::ZERO,
            credits_counter: -1,
            boss_started: false,
            delay: 0,
            last_tick: now,
        }
    }

    fn reset_for_start(&mut self, now: Instant) {
        *self = Self::new(now);
    }
}

async fn main() {
    let mut settings = Settings::register();

    loop {
        asr::set_tick_rate(1.0);
        let (process, process_name) = retry(|| {
            Process::attach("sonic.exe")
                .map(|process| (process, "sonic.exe"))
                .or_else(|| {
                    Process::attach("Sonic Adventure DX.exe")
                        .map(|process| (process, "Sonic Adventure DX.exe"))
                })
        })
        .await;

        process
            .until_closes(async {
                asr::set_tick_rate(120.0);
                let module = retry(|| process.get_module_address(process_name)).await;
                let mut run = RunState::new(Instant::now());
                let mut old = retry(|| read_state(&process, module)).await;

                loop {
                    next_tick().await;
                    settings.update();

                    let Ok(current) = read_state(&process, module) else {
                        continue;
                    };

                    update_game_time(&current, &old, &mut run);
                    timer::set_game_time(game_time(&run));
                    timer::pause_game_time();

                    match timer::state() {
                        TimerState::NotRunning => {
                            if should_start(&settings, &current, &old) {
                                run.reset_for_start(Instant::now());
                                timer::start();
                                timer::pause_game_time();
                            }
                        }
                        TimerState::Running | TimerState::Paused => {
                            if should_reset(&settings, &current, &old) {
                                timer::reset();
                            } else if should_split(&settings, &current, &old, &mut run) {
                                timer::split();
                            }
                        }
                        TimerState::Ended => {}
                        _ => {}
                    }

                    old = current;
                }
            })
            .await;
    }
}

fn read<T: Pod>(process: &Process, module: Address, offset: u64) -> Result<T, asr::Error> {
    process.read::<T>(module + offset)
}

fn read_state(process: &Process, module: Address) -> Result<GameState, asr::Error> {
    Ok(GameState {
        global_frame_count: read(process, module, GLOBAL_FRAME_COUNT)?,
        output_frame_count: read(process, module, OUTPUT_FRAME_COUNT)?,
        in_cutscene: read(process, module, IN_CUTSCENE)?,
        game_status: read(process, module, GAME_STATUS)?,
        game_mode: read(process, module, GAME_MODE)?,
        in_prerendered_movie: read(process, module, IN_PRERENDERED_MOVIE)?,
        curr_character: read(process, module, CURR_CHARACTER)?,
        last_story: read(process, module, LAST_STORY)?,
        timer_start: read(process, module, TIMER_START)?,
        demo_playing: read(process, module, DEMO_PLAYING)?,
        boss_health: read(process, module, BOSS_HEALTH)?,
        level: read(process, module, LEVEL)?,
        level_copy: read(process, module, LEVEL_COPY)?,
        act: read(process, module, ACT)?,
        lives: read(process, module, LIVES)?,
        e101_flag: read(process, module, E101_FLAG)?,
        adventure_data: read(process, module, ADVENTURE_DATA)?,
        rm_flag: read(process, module, RM_FLAG)?,
        mk2_value: read(process, module, MK2_VALUE)?,
        walker_value: read(process, module, WALKER_VALUE)?,
        selected_character: read(process, module, SELECTED_CHARACTER)?,
        music: read(process, module, MUSIC)?,
        lw_flag: read(process, module, LW_FLAG)?,
        emblem_bytes: read(process, module, EMBLEM_BYTES)?,
        event_bytes: read(process, module, EVENT_BYTES)?,
        lightshoes: read(process, module, LIGHT_SHOES)?,
        crystalring: read(process, module, CRYSTAL_RING)?,
        ancientlight: read(process, module, ANCIENT_LIGHT)?,
        jetanklet: read(process, module, JET_ANKLET)?,
        rhythmbadge: read(process, module, RHYTHM_BADGE)?,
        shovelclaw: read(process, module, SHOVEL_CLAW)?,
        fightinggloves: read(process, module, FIGHTING_GLOVES)?,
        warriorfeather: read(process, module, WARRIOR_FEATHER)?,
        longhammer: read(process, module, LONG_HAMMER)?,
        lifebelt: read(process, module, LIFE_BELT)?,
        powerrod: read(process, module, POWER_ROD)?,
        lureec: read(process, module, LURE_EC)?,
        lureic: read(process, module, LURE_IC)?,
        luress: read(process, module, LURE_SS)?,
        luremr: read(process, module, LURE_MR)?,
        jetbooster: read(process, module, JET_BOOSTER)?,
        laserblaster: read(process, module, LASER_BLASTER)?,
    })
}

fn story_enter(current: &GameState, old: &GameState) -> bool {
    current.demo_playing != 1
        && old.game_status == 21
        && current.game_mode != 12
        && current.game_mode != 20
        && (old.game_mode == 12 || old.game_mode == 20)
}

fn should_start(settings: &Settings, current: &GameState, old: &GameState) -> bool {
    let start = story_enter(current, old);
    if settings.char_start_ban_trial {
        start && current.game_mode != 9
    } else {
        start
    }
}

fn should_reset(settings: &Settings, current: &GameState, old: &GameState) -> bool {
    settings.reset && current.game_mode == 12 && old.curr_character != 6 && current.level_copy != 32
}

fn should_split(
    settings: &Settings,
    current: &GameState,
    old: &GameState,
    run: &mut RunState,
) -> bool {
    if settings.char_split {
        let split = story_enter(current, old);
        if split && (!settings.char_split_ban_trial || current.game_mode != 9) {
            return true;
        }
    }

    if (settings.ignore_sky_chase_1 && current.level == 36)
        || (settings.ignore_sky_chase_2 && current.level == 37)
    {
        return false;
    }

    if event_split(settings, current, old) {
        return true;
    }

    if current.in_cutscene != 0
        && old.in_cutscene == 0
        && current.level == 29
        && current.curr_character == 7
    {
        return true;
    }

    if settings.e101 && current.curr_character == 6 && current.e101_flag > old.e101_flag {
        return true;
    }

    if final_boss_split(settings, current, old) || boss_timing_split(settings, current, old, run) {
        return true;
    }

    if upgrade_split(settings, current, old)
        || boss_enter_split(settings, current, old)
        || stage_enter_split(settings, current, old)
        || misc_split(settings, current, old)
    {
        return true;
    }

    emblem_split(current, old, run)
}

fn event_split(settings: &Settings, current: &GameState, old: &GameState) -> bool {
    for i in 0..353 {
        if event_enabled(settings, i)
            && current.game_status != 21
            && current.event_bytes[i] != old.event_bytes[i]
        {
            return true;
        }
    }
    false
}

fn event_enabled(settings: &Settings, index: usize) -> bool {
    match index {
        36 => settings.gamma_sonic,
        37 => settings.knux_sonic,
        48 => settings.zero_sonic,
        49 => settings.four_sonic,
        50 => settings.six_sonic,
        51 => settings.egg_hornet_sonic,
        95 => settings.gamma_tails,
        96 => settings.knux_tails,
        103 => settings.four_tails,
        105 => settings.egg_hornet_tails,
        158 => settings.sonic_knuckles,
        163 => settings.two_knuckles,
        165 => settings.four_knuckles,
        282 => settings.sonic_gamma,
        _ => false,
    }
}

fn final_boss_split(settings: &Settings, current: &GameState, old: &GameState) -> bool {
    if current.level == 25 && current.mk2_value == 255 {
        return settings.beta_mk2
            && current.boss_health == 0
            && current.timer_start == 0
            && old.timer_start == 1;
    }
    if current.level == 19 {
        return settings.perfect_chaos && current.boss_health == 0 && old.boss_health == 2;
    }
    if current.level == 23 {
        return settings.zero && current.boss_health == 0 && old.boss_health == 1;
    }
    if current.level == 21 && current.walker_value == 9 {
        return settings.egg_walker
            && current.boss_health == 0
            && current.timer_start == 0
            && old.timer_start == 1;
    }
    false
}

fn boss_timing_split(
    settings: &Settings,
    current: &GameState,
    old: &GameState,
    run: &mut RunState,
) -> bool {
    let tracked_boss =
        (settings.chaos_6 && current.level == 18) || (settings.egg_viper && current.level == 22);
    if !tracked_boss {
        return false;
    }

    if current.boss_health == 0 && current.timer_start == 0 {
        run.boss_started = true;
    }

    if run.boss_started
        && current.in_cutscene == 0
        && old.in_cutscene == 0
        && current.boss_health == 0
        && current.game_status == 15
        && current.timer_start == 0
        && old.timer_start == 1
    {
        return !(current.level == 18 && current.curr_character == 0);
    }

    false
}

fn upgrade_split(settings: &Settings, current: &GameState, old: &GameState) -> bool {
    current.level != 0
        && ((settings.sonic_powerup1 && current.lightshoes > old.lightshoes)
            || (settings.sonic_powerup2 && current.crystalring > old.crystalring)
            || (settings.sonic_powerup3 && current.ancientlight > old.ancientlight)
            || (settings.tails_powerup1 && current.jetanklet > old.jetanklet)
            || (settings.tails_powerup2 && current.rhythmbadge > old.rhythmbadge)
            || (settings.knux_powerup1 && current.shovelclaw > old.shovelclaw)
            || (settings.knux_powerup2 && current.fightinggloves > old.fightinggloves)
            || (settings.amy_powerup1 && current.warriorfeather > old.warriorfeather)
            || (settings.amy_powerup2 && current.longhammer > old.longhammer)
            || (settings.big_powerup1 && current.powerrod > old.powerrod)
            || (settings.big_powerup2 && current.lifebelt > old.lifebelt)
            || (settings.big_powerup3
                && (current.luress > old.luress
                    || current.lureic != old.lureic
                    || current.lureec != old.lureec
                    || current.luremr != old.luremr))
            || (settings.gamma_powerup1 && current.jetbooster > old.jetbooster)
            || (settings.gamma_powerup2 && current.laserblaster > old.laserblaster))
}

fn boss_enter_split(settings: &Settings, current: &GameState, old: &GameState) -> bool {
    (settings.enter_egg_hornet && current.level == 20 && old.level == 33)
        || (settings.enter_chaos_2 && current.level == 16 && old.level == 26)
        || (settings.enter_chaos_4 && current.level == 17 && old.level == 33)
        || (settings.enter_chaos_6 && current.level == 18 && old.level == 29)
        || (settings.enter_beta_1 && current.level == 24 && old.level == 33)
        || (settings.enter_beta_2 && current.level == 25 && old.level == 29)
}

fn stage_enter_split(settings: &Settings, current: &GameState, old: &GameState) -> bool {
    (settings.eec && current.level == 1 && old.level == 26)
        || (settings.ewv && current.level == 2 && old.level == 33)
        || (settings.ec && current.level == 9 && old.level == 26)
        || (settings.eic && current.level == 8 && old.level == 33)
        || (settings.etp && current.level == 3 && old.level == 26)
        || (settings.esh && current.level == 4 && old.level == 26)
        || (settings.erm && current.level == 5 && old.level == 33)
        || (settings.esd && current.level == 6 && old.level == 29)
        || (settings.elw && current.level == 7 && old.level == 33)
        || (settings.efe && current.level == 10 && old.level == 33)
        || (settings.ehs && current.level == 12 && old.level == 32)
}

fn misc_split(settings: &Settings, current: &GameState, old: &GameState) -> bool {
    if settings.act_split
        && current.level != 0
        && current.level != 15
        && current.level != 22
        && current.level != 26
        && current.level != 29
        && current.level != 32
        && current.level != 33
        && current.level != 37
        && current.act != old.act
    {
        return true;
    }

    if settings.enter_sewer_sonic
        && current.level == 26
        && current.act == 2
        && old.act == 0
        && current.curr_character == 0
    {
        return true;
    }
    if settings.enter_cas_hub_sonic
        && current.level == 26
        && ((current.emblem_bytes[8] & 0x8) >> 3) == 1
        && ((current.emblem_bytes[8] & 0x10) >> 4) == 0
        && current.act == 1
        && (old.act == 3 || old.act == 4)
        && current.curr_character == 0
    {
        return true;
    }
    if settings.ship_transform && current.level == 29 && current.act == 6 && old.act == 3 {
        return true;
    }
    if settings.back_from_past
        && current.level == 33
        && current.act == 2
        && old.level == 34
        && old.act == 2
    {
        return true;
    }
    if settings.enter_ss
        && current.level == 26
        && current.act == 1
        && old.level == 33
        && old.act == 0
    {
        return true;
    }
    if settings.enter_mr
        && current.level == 33
        && current.act == 0
        && old.level == 26
        && old.act == 1
    {
        return true;
    }

    if settings.death_wv
        && current.level == 33
        && current.act == 0
        && current.adventure_data != old.adventure_data
        && current.curr_character == 6
    {
        return true;
    }
    if settings.death_mr_gamma
        && current.level == 33
        && current.act == 1
        && current.lives < old.lives
        && current.curr_character == 6
        && current.adventure_data == 2
    {
        return true;
    }
    if settings.egg_carrier_mk2
        && current.level == 29
        && old.level == 33
        && current.rm_flag == 1
        && current.curr_character == 6
    {
        return true;
    }

    if settings.death_mr_knux
        && current.level == 33
        && current.act == 1
        && current.lw_flag == 1
        && current.lives < old.lives
        && current.curr_character == 3
    {
        return true;
    }
    if settings.enter_eg_carrier_knux
        && current.level == 29
        && old.level == 33
        && current.curr_character == 3
    {
        return true;
    }

    if settings.enter_eg_carrier_amy
        && current.level == 29
        && old.level == 33
        && current.curr_character == 5
    {
        return true;
    }
    if settings.enter_cas_hub_amy
        && ((current.emblem_bytes[0xA] & 0x10) >> 4) == 0
        && current.level == 26
        && current.act == 1
        && old.act == 3
        && current.curr_character == 5
    {
        return true;
    }
    if settings.enter_jungle_to_fe_amy
        && current.level == 33
        && current.act == 2
        && old.act == 0
        && current.curr_character == 5
    {
        return true;
    }
    if settings.exit_jungle_to_ec_amy
        && current.level == 33
        && current.act == 0
        && old.act == 2
        && current.curr_character == 5
    {
        return true;
    }

    current.selected_character == 6
        && ((settings.enter_cave && current.level == 33 && current.act == 1 && old.act == 0)
            || (settings.angel_island
                && current.level == 33
                && current.act == 1
                && current.music == 40
                && old.music != 40)
            || (settings.tikal && current.level == 33 && old.level == 34)
            || (settings.death_ss
                && current.level == 33
                && current.act == 1
                && current.lives < old.lives)
            || (settings.jungle_ss
                && current.level == 33
                && old.level != 0
                && current.act == 2
                && old.act == 0))
}

fn emblem_split(current: &GameState, old: &GameState, run: &mut RunState) -> bool {
    for i in 0..current.emblem_bytes.len() {
        if current.game_mode != 12 && current.emblem_bytes[i] != old.emblem_bytes[i] {
            if current.level < 15 {
                run.delay = current.global_frame_count + 190;
            } else {
                return true;
            }
        }
    }

    if run.delay > 0 && (run.delay - current.global_frame_count).abs() <= 2 {
        run.delay = 0;
        return true;
    }

    false
}

fn update_game_time(current: &GameState, old: &GameState, run: &mut RunState) {
    let now = Instant::now();
    let tick_delta = now.duration_since(run.last_tick);

    if current.in_prerendered_movie != 0 {
        let tick_delta = Duration::nanoseconds_i128(tick_delta.as_nanos() as i128);
        run.prerendered_movie_rta = run.prerendered_movie_rta.saturating_add(tick_delta);
    }

    if current.game_mode == 1
        || current.game_mode == 12
        || current.game_mode == 18
        || current.game_mode == 20
        || current.game_status == 19
        || current.game_status == 16
    {
        run.total_frames += i64::from(current.output_frame_count - old.output_frame_count);
    }

    if current.game_mode == 22 {
        if old.game_mode != 22 {
            run.credits_counter = 3600;
        }

        run.credits_counter -= 1;

        if run.credits_counter == 0 {
            run.total_frames += match current.curr_character {
                0 if current.last_story == 0 => 16_804,
                0 => 17_360,
                2 => 14_657,
                3 => 16_804,
                5 => 18_950,
                6 => 17_545,
                7 => 15_333,
                _ => 0,
            };
        }
    }

    let mut delta_frames = current.global_frame_count - old.global_frame_count;
    if current.in_cutscene != 0 {
        delta_frames *= 2;
    }

    run.total_frames += i64::from(delta_frames);
    run.last_tick = now;
}

fn game_time(run: &RunState) -> Duration {
    Duration::saturating_seconds_f64(run.total_frames as f64 / 60.0)
        .saturating_add(run.prerendered_movie_rta)
}
