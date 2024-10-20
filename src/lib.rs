#[macro_use]
mod browser;
mod engine;
mod game;

use anyhow::Result;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    browser::spawn_local(async move {
        let game = game::WalkDog::new();
        engine::GameLoop::start(game)
            .await
            .expect("Coulid not start geame");
    });

    Ok(())
}
