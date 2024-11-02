#[macro_use]
mod browser;
mod engine;
mod segment;
mod sound;
mod walk_the_dog;

use anyhow::Result;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    browser::spawn_local(async move {
        let game = walk_the_dog::WalkTheDog::new();
        engine::GameLoop::start(game)
            .await
            .expect("Coulid not start geame");
    });

    Ok(())
}
