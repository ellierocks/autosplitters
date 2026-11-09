//! Japanese PC 1.0 port of Elle / Joeyfordayz's LEGO Star Wars autosplitter.
#![no_std]

#[cfg(test)]
extern crate std;

mod logic;
mod memory;

use asr::{
    future::{next_tick, retry},
    settings::Gui,
    timer::{self, TimerState},
};
use logic::{Engine, Settings};

asr::async_main!(stable);
asr::panic_handler!();

async fn main() {
    let mut settings = Settings::register();
    loop {
        asr::set_tick_rate(1.0);
        let game = retry(memory::attach).await;
        asr::print_message("Attached to LEGO Star Wars Japanese PC 1.0 (experimental)");
        game.process
            .until_closes(async {
                asr::set_tick_rate(60.0);
                let mut engine = Engine::default();
                let mut old = None;
                loop {
                    settings.update();
                    if let Some(current) = game.poll() {
                        if let Some(previous) = old {
                            engine.update(&settings, &previous, &current);
                            match timer::state() {
                                TimerState::NotRunning => {
                                    if engine.should_start(&settings, &previous, &current) {
                                        timer::start();
                                    }
                                }
                                TimerState::Running | TimerState::Paused => {
                                    if engine.should_reset(&settings, &previous, &current) {
                                        timer::reset();
                                    } else if engine.should_split(&settings, &previous, &current) {
                                        timer::split();
                                    }
                                }
                                _ => {}
                            }
                        }
                        old = Some(current);
                    } else {
                        // Never compare a fresh room to a stale sample across a load/read failure.
                        old = None;
                    }
                    next_tick().await;
                }
            })
            .await;
    }
}
