use crate::browser;
use crate::engine;
use crate::walk_the_dog::*;

use anyhow::anyhow;
use anyhow::Result;
use async_trait::async_trait;

pub struct WalkTheDog {
    rhb: Option<RedHatBoy>,
}

impl WalkTheDog {
    pub fn new() -> Self {
        WalkTheDog { rhb: None }
    }
}

#[async_trait(?Send)]
impl engine::Game for WalkTheDog {
    async fn initialize(&self) -> Result<Box<dyn engine::Game>> {
        Ok(Box::new(WalkTheDog {
            rhb: Some(RedHatBoy::new(
                browser::fetch_json("rhb.json")
                    .await?
                    .into_serde::<Sheet>()?,
                engine::load_image("rhb.png").await?,
            )),
        }))
    }

    fn update(&mut self, keystate: &engine::KeyState) {
        if keystate.is_pressed("ArrowDown") {
            self.rhb.as_mut().unwrap().slide();
        }
        if keystate.is_pressed("ArrowUp") {}
        if keystate.is_pressed("ArrowRight") {
            self.rhb.as_mut().unwrap().run_right();
        }
        if keystate.is_pressed("ArrowLeft") {}

        self.rhb.as_mut().unwrap().update();
    }

    fn draw(&mut self, renderer: &engine::Renderer) {
        renderer.clear(&engine::Rect::new(0.0, 0.0, 600.0, 600.0));
        self.rhb.as_ref().unwrap().draw(renderer);
    }
}
