use super::{browser, *};
use crate::{
    engine::{self, Audio, Game, Image, KeyState, Point, Rect, Renderer, Sheet, SpriteSheet},
    segment::*,
};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use rand::prelude::*;
use serde_wasm_bindgen::from_value;
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
    async fn new() -> Result<Self> {
        let rhb_sheet = from_value::<Sheet>(browser::fetch_json("rhb.json").await?)
            .map_err(|e| anyhow!("Failed to converting json to Sheet {}:#?", e))?;

        let background = engine::load_image("BG.png").await?;
        let stone = engine::load_image("Stone.png").await?;

        let tiles = browser::fetch_json("tiles.json").await?;

        let obstacle_sheet = Rc::new(SpriteSheet::new(
            from_value::<Sheet>(tiles)
                .map_err(|e| anyhow!("Failed to converting json to Sheet {}:#?", e))?,
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
        let (obstacles, timeline) = Self::starting_obstacles_and_timeline(stone.clone(), 0);

        Ok(Self {
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
            obstacles,
            obstacle_sheet,
            stone,
            timeline,
        })
    }

    fn starting_obstacles_and_timeline(
        stone: HtmlImageElement,
        offset_x: i16,
    ) -> (Vec<Box<dyn Obstacle>>, i16) {
        const STARTING_TIMELINE_BUFFER: i16 = 200;
        let obstacles = one_stone(stone, offset_x + OBSTACLE_BUFFER);
        let timeline = rightmost(&obstacles) + STARTING_TIMELINE_BUFFER;
        (obstacles, timeline)
    }

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
        let (obstacles, timeline) = Self::starting_obstacles_and_timeline(walk.stone.clone(), 0);
        Walk {
            boy: RedHatBoy::reset(walk.boy),
            backgrounds: walk.backgrounds,
            obstacles,
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
                let machine = WalkTheDogStateMachine::new(Walk::new().await?);
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

#[cfg(test)]
mod tests {
    use super::*;
    use engine::Sound;
    use futures::channel::mpsc::unbounded;
    use std::collections::HashMap;
    use walk_the_dog_state::{GameOver, WalkTheDogState};
    use web_sys::{AudioBuffer, AudioBufferOptions};

    use wasm_bindgen_test::wasm_bindgen_test;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_transation_from_game_over_to_new_game() {
        let (_, receiver) = unbounded();
        let image = HtmlImageElement::new().unwrap();
        let audio = Audio::new().unwrap();
        let options = AudioBufferOptions::new(1, 3000.0);
        let sound = Sound {
            buffer: AudioBuffer::new(&options).unwrap(),
        };
        let rhb = RedHatBoy::new(
            Sheet {
                frames: HashMap::new(),
            },
            image.clone(),
            audio,
            sound,
        );

        let sprite_sheet = SpriteSheet::new(
            Sheet {
                frames: HashMap::new(),
            },
            image.clone(),
        );

        let walk = Walk {
            boy: rhb,
            backgrounds: [
                Image::new(image.clone(), Point { x: 0, y: 0 }),
                Image::new(image.clone(), Point { x: 0, y: 0 }),
            ],
            obstacles: vec![],
            obstacle_sheet: Rc::new(sprite_sheet),
            stone: image.clone(),
            timeline: 0,
        };

        let document = browser::document().unwrap();
        document
            .body()
            .unwrap()
            .insert_adjacent_html("afterbegin", "<div id='ui'></div>")
            .unwrap();
        browser::draw_ui("<p>This is the UI</p>").unwrap();
        let state = WalkTheDogState {
            _state: GameOver {
                new_game_event: receiver,
            },
            walk: walk,
        };

        state.new_game();

        let ui = browser::find_html_element_by_id("ui").unwrap();
        assert_eq!(ui.child_element_count(), 0);
    }
}
