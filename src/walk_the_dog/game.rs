use crate::browser;
use crate::engine::{self, Game, Image, Point, Rect, Renderer, Sheet, SpriteSheet};
use crate::segment::{platform_and_stone, stone_and_platform};
use crate::walk_the_dog::*;

use anyhow::anyhow;
use anyhow::Result;
use async_trait::async_trait;
use rand::prelude::*;
use std::rc::Rc;
use web_sys::HtmlImageElement;

pub(crate) const HEIGHT: i16 = 600;

const HIGH_PLATFORM: i16 = 375;

const TIMELINE_MINIMUM: i16 = 1000;
const OBSTACLE_BUFFER: i16 = 20;

pub(crate) enum WalkTheDog {
    Loading,
    Loaded(Walk),
}

pub(crate) struct Walk {
    boy: RedHatBoy,
    backgrounds: [Image; 2],
    obstacle_sheet: Rc<SpriteSheet>,
    obstacles: Vec<Box<dyn Obstacle>>,
    stone: HtmlImageElement,
    timeline: i16,
}

impl Walk {
    fn velocity(&self) -> i16 {
        -self.boy.walking_speed()
    }

    fn generate_next_segment(&mut self) {
        let mut rng = thread_rng();
        let next_segment = rng.gen_range(0..2);

        let mut next_obstacles = match next_segment {
            0 => stone_and_platform(
                self.stone.clone(),
                self.obstacle_sheet.clone(),
                self.timeline + OBSTACLE_BUFFER,
            ),
            1 => platform_and_stone(
                self.stone.clone(),
                self.obstacle_sheet.clone(),
                self.timeline + OBSTACLE_BUFFER,
            ),
            _ => vec![],
        };

        self.timeline = rightmost(&next_obstacles);
        self.obstacles.append(&mut next_obstacles);
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

                let stone = engine::load_image("Stone.png").await?;

                let tiles = browser::fetch_json("tiles.json").await?;
                let obstacle_sheet = Rc::new(SpriteSheet::new(
                    tiles.into_serde::<Sheet>()?,
                    engine::load_image("tiles.png").await?,
                ));

                let boy = RedHatBoy::new(
                    browser::fetch_json("rhb.json")
                        .await?
                        .into_serde::<Sheet>()?,
                    engine::load_image("rhb.png").await?,
                );

                let starting_obstacles =
                    stone_and_platform(stone.clone(), obstacle_sheet.clone(), 0);
                let timeline = rightmost(&starting_obstacles);

                Ok(Box::new(WalkTheDog::Loaded(Walk {
                    boy,
                    backgrounds,
                    obstacles: starting_obstacles,
                    obstacle_sheet,
                    stone,
                    timeline,
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

            if walk.timeline < TIMELINE_MINIMUM {
                walk.generate_next_segment();
            } else {
                walk.timeline += velocity;
            }
        }
    }

    fn draw(&mut self, renderer: &Renderer) {
        renderer.clear(&&Rect::new_from_x_y(0, 0, 600, HEIGHT));
        if let WalkTheDog::Loaded(walk) = self {
            walk.backgrounds.iter().for_each(|bg| bg.draw(renderer));
            walk.boy.draw(renderer);
            walk.obstacles.iter().for_each(|obstacle| {
                obstacle.draw(renderer);
            });
        }
    }
}

fn rightmost(obstacle_list: &Vec<Box<dyn Obstacle>>) -> i16 {
    obstacle_list
        .iter()
        .map(|obstacle| obstacle.right())
        .max_by(|a, b| a.cmp(b))
        .unwrap_or(0)
}
