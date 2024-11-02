use crate::browser;
use crate::engine;
use crate::engine::Image;
use crate::engine::Point;
use crate::walk_the_dog::*;

use anyhow::anyhow;
use anyhow::Result;
use async_trait::async_trait;

pub(crate) const HEIGHT: i16 = 600;

pub(crate) const FIRST_PLATFORM: i16 = 370;
pub(crate) const LOW_PLATFORM: i16 = 420;
pub(crate) const HIGH_PLATFORM: i16 = 375;

pub(crate) enum WalkTheDog {
    Loading,
    Loaded(Walk),
}

pub(crate) struct Walk {
    boy: RedHatBoy,
    backgrounds: [engine::Image; 2],
    stone: engine::Image,
    platform: Platform,
}

impl Walk {
    fn velocity(&self) -> i16 {
        -self.boy.walking_speed()
    }
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
                let background = engine::load_image("BG.png").await?;
                let background_width = background.width() as i16;
                let backgrounds = [
                    Image::new(background.clone(), engine::Point { x: 0, y: 0 }),
                    Image::new(
                        background,
                        engine::Point {
                            x: background_width,
                            y: 0,
                        },
                    ),
                ];

                let stone = engine::Image::new(
                    engine::load_image("Stone.png").await?,
                    engine::Point { x: 150, y: 546 },
                );

                let platform = Platform::new(
                    browser::fetch_json("tiles.json")
                        .await?
                        .into_serde::<Sheet>()?,
                    engine::load_image("tiles.png").await?,
                    Point {
                        x: FIRST_PLATFORM,
                        y: LOW_PLATFORM,
                    },
                );

                let boy = RedHatBoy::new(
                    browser::fetch_json("rhb.json")
                        .await?
                        .into_serde::<Sheet>()?,
                    engine::load_image("rhb.png").await?,
                );
                Ok(Box::new(WalkTheDog::Loaded(Walk {
                    boy,
                    backgrounds,
                    stone,
                    platform,
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

            walk.platform.position.x += walk.velocity();
            walk.stone.move_horizontally(walk.velocity());

            let velocity = walk.velocity();
            let [first_bg, second_bg] = &mut walk.backgrounds;
            first_bg.move_horizontally(velocity);
            second_bg.move_horizontally(velocity);

            if first_bg.right() < 0 {
                first_bg.set_x(second_bg.right());
            }
            if second_bg.right() < 0 {
                second_bg.set_x(first_bg.right());
            }

            for bounding_box in &walk.platform.bounding_boxes() {
                if walk.boy.bounding_box().intersects(bounding_box) {
                    if walk.boy.velocity_y() > 0 && walk.boy.pos_y() < walk.platform.position.y {
                        walk.boy.land_on(bounding_box.y);
                    } else {
                        walk.boy.knock_out();
                    }
                }
            }

            // if walk
            //     .boy
            //     .bounding_box()
            //     .intersects(walk.stone.bounding_box())
            // {
            //     walk.boy.knock_out();
            // }
        }
    }

    fn draw(&mut self, renderer: &engine::Renderer) {
        renderer.clear(&engine::Rect::new(0.0, 0.0, 600.0, 600.0));
        if let WalkTheDog::Loaded(walk) = self {
            walk.backgrounds.iter().for_each(|bg| bg.draw(renderer));
            walk.boy.draw(renderer);
            walk.stone.draw(renderer);
            walk.platform.draw(renderer);
        }
    }
}
