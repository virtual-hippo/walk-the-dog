#[macro_use]
mod browser;

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

        let (success_tx, success_rx) = futures::channel::oneshot::channel::<Result<(), JsValue>>();
        let success_tx = Rc::new(Mutex::new(Some(success_tx)));
        let error_tx = Rc::clone(&success_tx);

        let image = web_sys::HtmlImageElement::new().unwrap();

        let callback = Closure::once(move || {
            if let Some(sender) = success_tx.lock().ok().and_then(|mut opt| opt.take()) {
                let _ = sender.send(Ok(()));
            }
        });

        let error_callback = Closure::once(move |err| {
            if let Some(sender) = error_tx.lock().ok().and_then(|mut opt| opt.take()) {
                let _ = sender.send(Err(err));
            }
        });

        image.set_onload(Some(callback.as_ref().unchecked_ref()));
        image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));

        image.set_src("rhb.png");

        let _ = success_rx.await;

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
