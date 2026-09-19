use log::info;
use screeps::game;
use wasm_bindgen::prelude::*;

mod logging;

static INIT_LOGGING: std::sync::Once = std::sync::Once::new();

#[wasm_bindgen(js_name = loop)]
pub fn game_loop() {
    INIT_LOGGING.call_once(|| {
        logging::setup_logging(logging::Info);
    });

    let tick = game::time();

    info!("Hello from Rust! Tick: {}", tick);
}
