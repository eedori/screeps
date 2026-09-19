use log::info;
use screeps::constants::Part;
use screeps::game;
use wasm_bindgen::prelude::*;

mod logging;

static INIT_LOGGING: std::sync::Once = std::sync::Once::new();

//webassembly glue
#[wasm_bindgen(js_name = loop)]
//gameloop - screeps runs this once per tick, except for the INIT. here we declare things that only need to run once.
pub fn game_loop() {
    INIT_LOGGING.call_once(|| {
        logging::setup_logging(logging::Info);
    });

    // let tick = game::time();
    //  info!("Hello from Rust! Tick: {}", tick);
    // this was the original test, for each tick say hi and count the tick

    let creep_count = game::creeps().values().count();
    let spawn_count = game::spawns().values().count();
    //figure out how many screeps we have, above gets creep_count and assigns it a value. this is useful.

    if creep_count < 1 {
        info!("less than one screep detected, bootstrap started");
        if spawn_count < 1 {
            info!("less than one spawner detected, cant spawn a screep.");
        } else {
            if let Some(spawn) = game::spawns().values().next() {
                let default_worker = [Part::Work, Part::Move, Part::Carry];
                let _douglas = spawn.spawn_creep(&default_worker, "Douglas");
            }
        }
    }
}
