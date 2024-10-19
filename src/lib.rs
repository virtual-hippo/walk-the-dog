#[macro_use]
mod browser;
mod engine;

use std::{collections::HashMap, rc::Rc, sync::Mutex};

use anyhow::{anyhow, Result};
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Deserialize)]
struct Rect {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

#[derive(Deserialize)]
struct Cell {
    frame: Rect,
}

#[derive(Deserialize)]
struct Sheet {
    frames: HashMap<String, Cell>,
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let context = browser::context().expect("No found  context");

    browser::spawn_local(async move {
        let sheet: Sheet = browser::fetch_json("rhb.json")
            .await
            .expect("Could not fetch rhb.json")
            .into_serde()
            .expect("Could not convert rhb.json into a Sheet structure");

        let image = engine::load_image("rhb.png")
            .await
            .expect("Could not load rhb.png");

        let mut frame: u8 = 7;
        let interval_callback = Closure::wrap(Box::new(move || {
            context.clear_rect(0.0, 0.0, 600.0, 600.0);

            frame = (frame + 1) % 8;
            let frame_name = format!("Run ({}).png", frame + 1);
            let splite = sheet.frames.get(&frame_name).expect("Cell not found");

            let _ = context
                .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    &image,
                    splite.frame.x.into(),
                    splite.frame.y.into(),
                    splite.frame.w.into(),
                    splite.frame.h.into(),
                    300.0,
                    300.0,
                    splite.frame.w.into(),
                    splite.frame.h.into(),
                );
        }) as Box<dyn FnMut()>);

        browser::window()
            .unwrap()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                interval_callback.as_ref().unchecked_ref(),
                50,
            )
            .unwrap();
        interval_callback.forget();
    });

    Ok(())
}
