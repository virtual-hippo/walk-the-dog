use crate::browser;
use crate::engine;

use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use std::collections::HashMap;
use web_sys::HtmlImageElement;

#[derive(Deserialize)]
struct SheetRect {
    x: i16,
    y: i16,
    w: i16,
    h: i16,
}

#[derive(Deserialize)]
struct Cell {
    frame: SheetRect,
}

#[derive(Deserialize)]
struct Sheet {
    frames: HashMap<String, Cell>,
}

pub struct WalkDog {
    image: Option<HtmlImageElement>,
    sheet: Option<Sheet>,
    frame: u8,
}

impl WalkDog {
    pub fn new() -> Self {
        WalkDog {
            image: None,
            sheet: None,
            frame: 0,
        }
    }
}

#[async_trait(?Send)]
impl engine::Game for WalkDog {
    async fn initialize(&self) -> Result<Box<dyn engine::Game>> {
        let sheet = browser::fetch_json("rhb.json").await?.into_serde()?;

        let image = engine::load_image("rhb.png").await?;

        Ok(Box::new(WalkDog {
            image: Some(image),
            sheet: Some(sheet),
            frame: self.frame,
        }))
    }

    fn update(&mut self) {
        if self.frame < 23 {
            self.frame += 1;
        } else {
            self.frame = 0;
        }
    }

    fn draw(&mut self, renderer: &engine::Renderer) {
        let current_sprite = (self.frame / 3) + 1;
        let frame_name = format!("Run ({}).png", current_sprite);
        let splite = self
            .sheet
            .as_ref()
            .and_then(|sheet| sheet.frames.get(&frame_name))
            .expect("Cell not found");

        renderer.clear(&engine::Rect::new(0.0, 0.0, 600.0, 600.0));

        self.image.as_ref().map(|image| {
            let _ = renderer.draw_image(
                image,
                &engine::Rect::new(
                    splite.frame.x.into(),
                    splite.frame.y.into(),
                    splite.frame.w.into(),
                    splite.frame.h.into(),
                ),
                &engine::Rect::new(300.0, 300.0, splite.frame.w.into(), splite.frame.h.into()),
            );
        });
    }
}
