use crate::{
    browser,
    engine::{self, Audio, Game, Image, KeyState, Point, Rect, Renderer, Sheet, SpriteSheet},
    segment::*,
    walk_the_dog::*,
};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use rand::prelude::*;
use std::rc::Rc;
use web_sys::HtmlImageElement;

pub(super) const HEIGHT: i16 = 600;
const OBSTACLE_BUFFER: i16 = 20;

pub(crate) struct WalkTheDog {
    machine: Option<WalkTheDogStateMachine>,
}

impl WalkTheDog {
    pub(crate) fn new() -> Self {
        WalkTheDog { machine: None }
    }
}

pub(crate) struct Walk {
    pub(super) obstacle_sheet: Rc<SpriteSheet>,
    pub(super) stone: HtmlImageElement,
    pub(super) boy: RedHatBoy,
    pub(super) backgrounds: [Image; 2],
    pub(super) obstacles: Vec<Box<dyn Obstacle>>,
    pub(super) timeline: i16,
}

impl Walk {
    pub(super) fn velocity(&self) -> i16 {
        -self.boy.walking_speed()
    }

    pub(super) fn generate_next_segment(&mut self) {
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

    pub(super) fn draw(&self, renderer: &Renderer) {
        self.backgrounds
            .iter()
            .for_each(|background| background.draw(renderer));
        self.boy.draw(renderer);

        self.obstacles.iter().for_each(|obstacle| {
            obstacle.draw(renderer);
        });
    }

    pub(super) fn knocked_out(&self) -> bool {
        self.boy.knocked_out()
    }

    pub(super) fn reset(walk: Self) -> Self {
        let starting_obstacles =
            stone_and_platform(walk.stone.clone(), walk.obstacle_sheet.clone(), 0);
        let timeline = rightmost(&starting_obstacles);
        Walk {
            boy: RedHatBoy::reset(walk.boy),
            backgrounds: walk.backgrounds,
            obstacles: starting_obstacles,
            obstacle_sheet: walk.obstacle_sheet,
            stone: walk.stone,
            timeline,
        }
    }
}

#[async_trait(?Send)]
impl Game for WalkTheDog {
    async fn initialize(&self) -> Result<Box<dyn Game>> {
        match self.machine {
            None => {
                let rhb_sheet = browser::fetch_json("rhb.json")
                    .await?
                    .into_serde::<Sheet>()?;
                let background = engine::load_image("BG.png").await?;
                let stone = engine::load_image("Stone.png").await?;

                let tiles = browser::fetch_json("tiles.json").await?;

                let obstacle_sheet = Rc::new(SpriteSheet::new(
                    tiles.into_serde::<Sheet>()?,
                    engine::load_image("tiles.png").await?,
                ));

                let audio = Audio::new()?;
                let sound = audio.load_sound("SFX_Jump_23.mp3").await?;
                let background_music = audio.load_sound("background_song.mp3").await?;
                audio.play_looping_sound(&background_music)?;

                let boy = RedHatBoy::new(
                    rhb_sheet,
                    engine::load_image("rhb.png").await?,
                    audio,
                    sound,
                );

                let background_width = background.width() as i16;
                let starting_obstacles =
                    stone_and_platform(stone.clone(), obstacle_sheet.clone(), 0);
                let timeline = rightmost(&starting_obstacles);

                let machine = WalkTheDogStateMachine::new(Walk {
                    boy,
                    backgrounds: [
                        Image::new(background.clone(), Point { x: 0, y: 0 }),
                        Image::new(
                            background,
                            Point {
                                x: background_width,
                                y: 0,
                            },
                        ),
                    ],
                    obstacles: starting_obstacles,
                    obstacle_sheet,
                    stone,
                    timeline,
                });

                Ok(Box::new(WalkTheDog {
                    machine: Some(machine),
                }))
            }
            Some(_) => Err(anyhow!("Error: Game is already initialized")),
        }
    }

    fn update(&mut self, keystate: &KeyState) {
        if let Some(machine) = self.machine.take() {
            self.machine.replace(machine.update(keystate));
        }
        assert!(self.machine.is_some());
    }

    fn draw(&mut self, renderer: &Renderer) {
        renderer.clear(&&Rect::new_from_x_y(0, 0, 600, HEIGHT));

        if let Some(machine) = &self.machine {
            machine.draw(renderer);
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
