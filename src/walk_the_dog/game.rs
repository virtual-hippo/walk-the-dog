use anyhow::{anyhow, Result};
use async_trait::async_trait;
use rand::prelude::*;
use std::rc::Rc;
use web_sys::HtmlImageElement;

use crate::{
    browser,
    engine::{self, Audio, Game, Image, KeyState, Point, Rect, Renderer, Sheet, SpriteSheet},
    segment::*,
    walk_the_dog::*,
};

pub(super) const HEIGHT: i16 = 600;
const TIMELINE_MINIMUM: i16 = 1000;
const OBSTACLE_BUFFER: i16 = 20;

enum WalkTheDogStateMachine {
    Ready(WalkTheDogState<Ready>),
    Walking(WalkTheDogState<Walking>),
    GameOver(WalkTheDogState<GameOver>),
}

impl WalkTheDogStateMachine {
    fn new(walk: Walk) -> Self {
        WalkTheDogStateMachine::Ready(WalkTheDogState::new(walk))
    }

    fn update(self, keystate: &KeyState) -> Self {
        match self {
            WalkTheDogStateMachine::Ready(state) => state.update(keystate).into(),
            WalkTheDogStateMachine::Walking(state) => state.update(keystate).into(),
            WalkTheDogStateMachine::GameOver(state) => state.update().into(),
        }
    }

    fn draw(&self, renderer: &Renderer) {
        match self {
            WalkTheDogStateMachine::Ready(state) => state.draw(renderer),
            WalkTheDogStateMachine::Walking(state) => state.draw(renderer),
            WalkTheDogStateMachine::GameOver(state) => state.draw(renderer),
        }
    }
}

impl From<WalkTheDogState<Ready>> for WalkTheDogStateMachine {
    fn from(state: WalkTheDogState<Ready>) -> Self {
        WalkTheDogStateMachine::Ready(state)
    }
}

impl From<WalkTheDogState<Walking>> for WalkTheDogStateMachine {
    fn from(state: WalkTheDogState<Walking>) -> Self {
        WalkTheDogStateMachine::Walking(state)
    }
}

impl From<WalkTheDogState<GameOver>> for WalkTheDogStateMachine {
    fn from(state: WalkTheDogState<GameOver>) -> Self {
        WalkTheDogStateMachine::GameOver(state)
    }
}

struct WalkTheDogState<S> {
    _state: S,
    walk: Walk,
}

impl<S> WalkTheDogState<S> {
    fn draw(&self, renderer: &Renderer) {
        self.walk.draw(renderer);
    }
}

struct Ready;

impl WalkTheDogState<Ready> {
    fn new(walk: Walk) -> Self {
        Self {
            _state: Ready,
            walk,
        }
    }

    fn update(mut self, keystate: &KeyState) -> ReadyEndState {
        self.walk.boy.update();
        if keystate.is_pressed("ArrowRight") {
            ReadyEndState::Complete(self.start_running())
        } else {
            ReadyEndState::Continue(self)
        }
    }

    fn start_running(mut self) -> WalkTheDogState<Walking> {
        self.run_right();
        WalkTheDogState {
            _state: Walking,
            walk: self.walk,
        }
    }

    fn run_right(&mut self) {
        self.walk.boy.run_right();
    }
}

enum ReadyEndState {
    Complete(WalkTheDogState<Walking>),
    Continue(WalkTheDogState<Ready>),
}

impl From<ReadyEndState> for WalkTheDogStateMachine {
    fn from(state: ReadyEndState) -> Self {
        match state {
            ReadyEndState::Complete(walking) => WalkTheDogStateMachine::Walking(walking),
            ReadyEndState::Continue(ready) => WalkTheDogStateMachine::Ready(ready),
        }
    }
}

struct Walking;

impl WalkTheDogState<Walking> {
    fn update(mut self, keystate: &KeyState) -> WalkTheDogState<Walking> {
        if keystate.is_pressed("Space") {
            self.walk.boy.jump();
        }

        if keystate.is_pressed("ArrowDown") {
            self.walk.boy.slide();
        }

        self.walk.boy.update();

        let walking_speed = self.walk.velocity();
        let [first_background, second_background] = &mut self.walk.backgrounds;
        first_background.move_horizontally(walking_speed);
        second_background.move_horizontally(walking_speed);

        if first_background.right() < 0 {
            first_background.set_x(second_background.right());
        }
        if second_background.right() < 0 {
            second_background.set_x(first_background.right());
        }

        self.walk.obstacles.retain(|obstacle| obstacle.right() > 0);

        self.walk.obstacles.iter_mut().for_each(|obstacle| {
            obstacle.move_horizontally(walking_speed);
            obstacle.check_intersection(&mut self.walk.boy);
        });

        if self.walk.timeline < TIMELINE_MINIMUM {
            self.walk.generate_next_segment();
        } else {
            self.walk.timeline += walking_speed;
        }
        self
    }
}

struct GameOver;
impl WalkTheDogState<GameOver> {
    fn update(self) -> WalkTheDogState<GameOver> {
        self
    }
}

pub(crate) struct WalkTheDog {
    machine: Option<WalkTheDogStateMachine>,
}

impl WalkTheDog {
    pub(crate) fn new() -> Self {
        WalkTheDog { machine: None }
    }
}

pub(crate) struct Walk {
    obstacle_sheet: Rc<SpriteSheet>,
    stone: HtmlImageElement,
    boy: RedHatBoy,
    backgrounds: [Image; 2],
    obstacles: Vec<Box<dyn Obstacle>>,
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

    fn draw(&self, renderer: &Renderer) {
        self.backgrounds
            .iter()
            .for_each(|background| background.draw(renderer));
        self.boy.draw(renderer);

        self.obstacles.iter().for_each(|obstacle| {
            obstacle.draw(renderer);
        });
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
