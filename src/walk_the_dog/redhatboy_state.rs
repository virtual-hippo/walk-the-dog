use super::game::HEIGHT;
use crate::engine::{Audio, Point, Sound};

const FLOOR: i16 = 479;
const PLAYER_HEIGHT: i16 = HEIGHT - FLOOR;
const RUNNING_SPEED: i16 = 4;
const STARTING_POINT: i16 = -20;

const IDLE_FRAMES: u8 = 29;
const RUNNING_FRAMES: u8 = 23;
const JUMPING_FRAMES: u8 = 35;
const SLIDING_FRAMES: u8 = 14;
const FALLING_FRAMES: u8 = 29;
const IDLE_FRAME_NAME: &str = "Idle";
const RUN_FRAME_NAME: &str = "Run";
const SLIDING_FRAME_NAME: &str = "Slide";
const JUMPING_FRAME_NAME: &str = "Jump";
const FALLING_FRAME_NAME: &str = "Dead";

const JUMP_SPEED: i16 = -25;
const GRAVITY: i16 = 1;

const TERMINAL_VELOCITY: i16 = 20;

#[derive(Clone)]
pub(super) struct RedHatBoyState<S> {
    context: RedHatBoyContext,
    _state: S,
}

impl<S> RedHatBoyState<S> {
    pub(super) fn context(&self) -> &RedHatBoyContext {
        &self.context
    }

    fn update_context(&mut self, frames: u8) {
        self.context = self.context.clone().update(frames);
    }
}

#[derive(Clone)]
pub(super) struct Idle;

impl RedHatBoyState<Idle> {
    pub(super) fn new(audio: Audio, jump_sound: Sound) -> Self {
        RedHatBoyState {
            context: RedHatBoyContext {
                frame: 0,
                position: Point {
                    x: STARTING_POINT,
                    y: FLOOR,
                },
                velocity: Point { x: 0, y: 0 },
                audio,
                jump_sound,
            },
            _state: Idle {},
        }
    }

    pub(super) fn frame_name(&self) -> &str {
        IDLE_FRAME_NAME
    }

    pub(super) fn update(mut self) -> Self {
        self.update_context(IDLE_FRAMES);
        self
    }

    pub(super) fn run(self) -> RedHatBoyState<Running> {
        RedHatBoyState {
            context: self.context.reset_frame().run_right(),
            _state: Running {},
        }
    }
}

#[derive(Clone)]
pub(super) struct Running;

impl RedHatBoyState<Running> {
    pub(super) fn frame_name(&self) -> &str {
        RUN_FRAME_NAME
    }

    pub(super) fn update(mut self) -> Self {
        self.update_context(RUNNING_FRAMES);
        self
    }

    pub(super) fn jump(self) -> RedHatBoyState<Jumping> {
        RedHatBoyState {
            context: self
                .context
                .reset_frame()
                .set_vertical_velocity(JUMP_SPEED)
                .play_jump_sound(),
            _state: Jumping {},
        }
    }

    pub(super) fn slide(self) -> RedHatBoyState<Sliding> {
        RedHatBoyState {
            context: self.context.reset_frame(),
            _state: Sliding {},
        }
    }

    pub(super) fn knock_out(self) -> RedHatBoyState<Falling> {
        RedHatBoyState {
            context: self.context.reset_frame().stop(),
            _state: Falling {},
        }
    }

    pub(crate) fn land_on(self, position: i16) -> RedHatBoyState<Running> {
        RedHatBoyState {
            context: self.context.set_on(position),
            _state: Running {},
        }
    }
}

#[derive(Clone)]
pub(super) struct Jumping;

pub(super) enum JumpingEndState {
    Jumping(RedHatBoyState<Jumping>),
    Landing(RedHatBoyState<Running>),
}

impl RedHatBoyState<Jumping> {
    pub(super) fn frame_name(&self) -> &str {
        JUMPING_FRAME_NAME
    }

    pub(super) fn knock_out(self) -> RedHatBoyState<Falling> {
        RedHatBoyState {
            context: self.context.reset_frame().stop(),
            _state: Falling {},
        }
    }

    pub(super) fn update(mut self) -> JumpingEndState {
        self.update_context(JUMPING_FRAMES);

        if self.context.position.y >= FLOOR {
            JumpingEndState::Landing(self.land_on(HEIGHT))
        } else {
            JumpingEndState::Jumping(self)
        }
    }

    pub(crate) fn land_on(self, position: i16) -> RedHatBoyState<Running> {
        RedHatBoyState {
            context: self.context.reset_frame().set_on(position),
            _state: Running {},
        }
    }
}

#[derive(Clone)]
pub(super) struct Sliding;

pub(super) enum SlidingEndState {
    Sliding(RedHatBoyState<Sliding>),
    Complete(RedHatBoyState<Running>),
}

impl RedHatBoyState<Sliding> {
    pub(super) fn frame_name(&self) -> &str {
        SLIDING_FRAME_NAME
    }

    fn stand(self) -> RedHatBoyState<Running> {
        RedHatBoyState {
            context: self.context.reset_frame(),
            _state: Running {},
        }
    }

    pub(super) fn knock_out(self) -> RedHatBoyState<Falling> {
        RedHatBoyState {
            context: self.context.reset_frame().stop(),
            _state: Falling {},
        }
    }

    pub(super) fn update(mut self) -> SlidingEndState {
        self.update_context(SLIDING_FRAMES);

        if self.context.frame >= SLIDING_FRAMES {
            SlidingEndState::Complete(self.stand())
        } else {
            SlidingEndState::Sliding(self)
        }
    }

    pub(crate) fn land_on(self, position: i16) -> RedHatBoyState<Sliding> {
        RedHatBoyState {
            context: self.context.set_on(position),
            _state: Sliding {},
        }
    }
}

#[derive(Clone)]
pub(super) struct Falling;
impl RedHatBoyState<Falling> {
    pub(super) fn frame_name(&self) -> &str {
        FALLING_FRAME_NAME
    }

    fn end(&self) -> RedHatBoyState<KnockedOut> {
        RedHatBoyState {
            context: self.context.clone(),
            _state: KnockedOut {},
        }
    }

    pub(super) fn update(mut self) -> FallingEndState {
        self.update_context(FALLING_FRAMES);
        if self.context.frame >= FALLING_FRAMES {
            FallingEndState::Complete(self.end())
        } else {
            FallingEndState::Falling(self)
        }
    }
}

pub(super) enum FallingEndState {
    Complete(RedHatBoyState<KnockedOut>),
    Falling(RedHatBoyState<Falling>),
}

#[derive(Clone)]
pub(super) struct KnockedOut;

impl RedHatBoyState<KnockedOut> {
    pub(super) fn frame_name(&self) -> &str {
        FALLING_FRAME_NAME
    }
}

#[derive(Clone)]
pub(super) struct RedHatBoyContext {
    pub(super) frame: u8,
    pub(super) position: Point,
    pub(super) velocity: Point,
    pub(super) audio: Audio,
    pub(super) jump_sound: Sound,
}

impl RedHatBoyContext {
    pub(super) fn update(mut self, frame_count: u8) -> Self {
        if self.velocity.y < TERMINAL_VELOCITY {
            self.velocity.y += GRAVITY;
        }

        if self.frame < frame_count {
            self.frame += 1;
        } else {
            self.frame = 0;
        }

        self.position.y += self.velocity.y;

        if self.position.y > FLOOR {
            self.position.y = FLOOR;
        }

        self
    }

    fn reset_frame(mut self) -> Self {
        self.frame = 0;
        self
    }

    fn set_vertical_velocity(mut self, y: i16) -> Self {
        self.velocity.y = y;
        self
    }

    fn run_right(mut self) -> Self {
        self.velocity.x += RUNNING_SPEED;
        self
    }

    fn stop(mut self) -> Self {
        self.velocity.x = 0;
        self.velocity.y = 0;
        self
    }

    fn set_on(mut self, position: i16) -> Self {
        let position = position - PLAYER_HEIGHT;
        self.position.y = position;
        self
    }

    fn play_jump_sound(self) -> Self {
        if let Err(err) = self.audio.play_sound(&self.jump_sound) {
            log!("Error playing jump sound {:#?}", err);
        }
        self
    }
}
