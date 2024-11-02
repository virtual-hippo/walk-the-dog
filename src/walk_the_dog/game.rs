use crate::browser;
use crate::engine::{self, Game, Image, Point, Rect, Renderer, Sheet, SpriteSheet};
use crate::walk_the_dog::*;

use anyhow::anyhow;
use anyhow::Result;
use async_trait::async_trait;
use std::rc::Rc;

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
    backgrounds: [Image; 2],
    obstacle_sheet: Rc<SpriteSheet>,
    obstacles: Vec<Box<dyn Obstacle>>,
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
impl Game for WalkTheDog {
    async fn initialize(&self) -> Result<Box<dyn Game>> {
        match self {
            WalkTheDog::Loading => {
                let background = engine::load_image("BG.png").await?;
                let background_width = background.width() as i16;
                let backgrounds = [
                    Image::new(background.clone(), Point { x: 0, y: 0 }),
                    Image::new(
                        background,
                        Point {
                            x: background_width,
                            y: 0,
                        },
                    ),
                ];

                let stone_image = Image::new(
                    engine::load_image("Stone.png").await?,
                    Point { x: 150, y: 546 },
                );

                let tiles = browser::fetch_json("tiles.json").await?;
                let obstacle_sheet = Rc::new(SpriteSheet::new(
                    tiles.into_serde::<Sheet>()?,
                    engine::load_image("tiles.png").await?,
                ));

                let platform = Platform::new(
                    obstacle_sheet.clone(),
                    Point {
                        x: FIRST_PLATFORM,
                        y: LOW_PLATFORM,
                    },
                    &["13.png", "14.png", "15.png"],
                    &[
                        Rect::new_from_x_y(0, 0, 60, 54),
                        Rect::new_from_x_y(60, 0, 384 - (60 * 2), 93),
                        Rect::new_from_x_y(384 - 60, 0, 60, 54),
                    ],
                );

                let boy = RedHatBoy::new(
                    browser::fetch_json("rhb.json")
                        .await?
                        .into_serde::<Sheet>()?,
                    engine::load_image("rhb.png").await?,
                );

                let obstacles: Vec<Box<dyn Obstacle>> =
                    vec![Box::new(Barrier::new(stone_image)), Box::new(platform)];

                Ok(Box::new(WalkTheDog::Loaded(Walk {
                    boy,
                    backgrounds,
                    obstacle_sheet,
                    obstacles,
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

            walk.obstacles.retain(|obstacle| obstacle.right() > 0);
            walk.obstacles.iter_mut().for_each(|obstacle| {
                obstacle.move_horizontally(velocity);
                obstacle.check_intersection(&mut walk.boy);
            });
        }
    }

    fn draw(&mut self, renderer: &Renderer) {
        renderer.clear(&&Rect::new_from_x_y(0, 0, 600, 600));
        if let WalkTheDog::Loaded(walk) = self {
            walk.backgrounds.iter().for_each(|bg| bg.draw(renderer));
            walk.boy.draw(renderer);
            walk.obstacles.iter().for_each(|obstacle| {
                obstacle.draw(renderer);
            });
        }
    }
}
