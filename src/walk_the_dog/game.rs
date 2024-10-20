use crate::browser;
use crate::engine;
use crate::walk_the_dog::*;

use anyhow::anyhow;
use anyhow::Result;
use async_trait::async_trait;

pub(crate) enum WalkTheDog {
    Loading,
    Loaded(Walk),
}

pub(crate) struct Walk {
    boy: RedHatBoy,
    background: engine::Image,
    stone: engine::Image,
}

impl WalkTheDog {
    pub(crate) fn new() -> Self {
        WalkTheDog::Loading
    }
}

#[async_trait(?Send)]
impl engine::Game for WalkTheDog {
    async fn initialize(&self) -> Result<Box<dyn engine::Game>> {
        match self {
            WalkTheDog::Loading => {
                let background = engine::Image::new(
                    engine::load_image("BG.png").await?,
                    engine::Point { x: 0, y: 0 },
                );
                let stone = engine::Image::new(
                    engine::load_image("Stone.png").await?,
                    engine::Point { x: 150, y: 546 },
                );
                let boy = RedHatBoy::new(
                    browser::fetch_json("rhb.json")
                        .await?
                        .into_serde::<Sheet>()?,
                    engine::load_image("rhb.png").await?,
                );
                Ok(Box::new(WalkTheDog::Loaded(Walk {
                    boy,
                    background,
                    stone,
                })))
            }
            WalkTheDog::Loaded(_) => Err(anyhow!("Error: Game is already initialized")),
        }
    }

    fn update(&mut self, keystate: &engine::KeyState) {
        if let WalkTheDog::Loaded(walk) = self {
            if keystate.is_pressed("ArrowDown") {
                walk.boy.slide();
            }
            if keystate.is_pressed("Space") {
                walk.boy.jump();
            }
            if keystate.is_pressed("ArrowRight") {
                walk.boy.run_right();
            }
            if keystate.is_pressed("ArrowLeft") {}

            walk.boy.update();
        }
    }

    fn draw(&mut self, renderer: &engine::Renderer) {
        renderer.clear(&engine::Rect::new(0.0, 0.0, 600.0, 600.0));
        if let WalkTheDog::Loaded(walk) = self {
            walk.background.draw(renderer);
            walk.stone.draw(renderer);
            walk.boy.draw(renderer);
        }
    }
}
