use super::game::HEIGHT;
use crate::engine::Point;

const FLOOR: i16 = 479;
const STARTING_POINT: i16 = -20;
const PLAYER_HEIGHT: i16 = HEIGHT - FLOOR;

const IDLE_FRAME_NAME: &str = "Idle";
const RUN_FRAME_NAME: &str = "Run";
const SLIDING_FRAME_NAME: &str = "Slide";
const JUMPING_FRAME_NAME: &str = "Jump";
const FALLING_FRAME_NAME: &str = "Dead";

const IDLE_FRAMES: u8 = 29;
const RUNNING_FRAMES: u8 = 23;
const SLIDING_FRAMES: u8 = 14;
const JUMPING_FRAMES: u8 = 35;
const FALLING_FRAMES: u8 = 29;

const RUNNING_SPEED: i16 = 3;
const JUMP_SPEED: i16 = -25;
const GRAVITY: i16 = 1;

const TERMINAL_VELOCITY: i16 = 20;

#[derive(Clone, Copy)]
pub(super) struct Idle;

#[derive(Clone, Copy)]
pub(super) struct Running;

#[derive(Clone, Copy)]
pub(super) struct Sliding;

#[derive(Clone, Copy)]
pub(super) struct Jumping;
#[derive(Clone, Copy)]
pub(super) struct Falling;
#[derive(Clone, Copy)]
pub(super) struct KnockedOut;

#[derive(Clone, Copy)]
pub(super) struct RedHatBoyContext {
    pub(super) frame: u8,
    pub(super) position: Point,
    pub(super) velocity: Point,
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

        self.position.x += self.velocity.x;
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

    fn run_right(mut self) -> Self {
        self.velocity.x += RUNNING_SPEED;
        self
    }

    fn set_on(mut self, position: i16) -> Self {
        let position = position - PLAYER_HEIGHT;
        self.position.y = position;
        self
    }

    fn set_vertical_velocity(mut self, y: i16) -> Self {
        self.velocity.y = y;
        self
    }

    fn stop(mut self) -> Self {
        self.velocity.x = 0;
        self.velocity.y = 0;
        self
    }
}

#[derive(Clone, Copy)]
pub(super) struct RedHatBoyState<S> {
    context: RedHatBoyContext,
    _state: S,
}

impl<S> RedHatBoyState<S> {
    pub(super) fn context(&self) -> &RedHatBoyContext {
        &self.context
    }
}

impl RedHatBoyState<Idle> {
    pub(super) fn new() -> Self {
        RedHatBoyState {
            context: RedHatBoyContext {
                frame: 0,
                position: Point {
                    x: STARTING_POINT,
                    y: FLOOR,
                },
                velocity: Point { x: 0, y: 0 },
            },
            _state: Idle {},
        }
    }

    pub(super) fn run(self) -> RedHatBoyState<Running> {
        RedHatBoyState {
            context: self.context.reset_frame().run_right(),
            _state: Running {},
        }
    }

    pub(super) fn frame_name(&self) -> &str {
        IDLE_FRAME_NAME
    }

    pub(super) fn update(mut self) -> Self {
        self.context = self.context.update(IDLE_FRAMES);
        self
    }
}

impl RedHatBoyState<Running> {
    pub(super) fn slide(self) -> RedHatBoyState<Sliding> {
        RedHatBoyState {
            context: self.context.reset_frame(),
            _state: Sliding {},
        }
    }

    pub(super) fn jump(self) -> RedHatBoyState<Jumping> {
        RedHatBoyState {
            context: self.context.set_vertical_velocity(JUMP_SPEED).reset_frame(),
            _state: Jumping {},
        }
    }

    pub(crate) fn land_on(self, position: f32) -> RedHatBoyState<Running> {
        RedHatBoyState {
            context: self.context.set_on(position as i16),
            _state: Running {},
        }
    }

    pub(super) fn frame_name(&self) -> &str {
        RUN_FRAME_NAME
    }

    pub(super) fn update(mut self) -> Self {
        self.context = self.context.update(RUNNING_FRAMES);
        self
    }

    pub(super) fn knock_out(self) -> RedHatBoyState<Falling> {
        RedHatBoyState {
            context: self.context.reset_frame().stop(),
            _state: Falling {},
        }
    }
}

pub(super) enum SlidingEndState {
    Complete(RedHatBoyState<Running>),
    Sliding(RedHatBoyState<Sliding>),
}

impl RedHatBoyState<Sliding> {
    fn stand(self) -> RedHatBoyState<Running> {
        RedHatBoyState {
            context: self.context.reset_frame(),
            _state: Running {},
        }
    }

    pub(crate) fn land_on(self, position: f32) -> RedHatBoyState<Sliding> {
        RedHatBoyState {
            context: self.context.set_on(position as i16),
            _state: Sliding {},
        }
    }

    pub(super) fn frame_name(&self) -> &str {
        SLIDING_FRAME_NAME
    }

    pub(super) fn update(mut self) -> SlidingEndState {
        self.context = self.context.update(SLIDING_FRAMES);

        if self.context.frame >= SLIDING_FRAMES {
            SlidingEndState::Complete(self.stand())
        } else {
            SlidingEndState::Sliding(self)
        }
    }

    pub(super) fn knock_out(self) -> RedHatBoyState<Falling> {
        RedHatBoyState {
            context: self.context.reset_frame().stop(),
            _state: Falling {},
        }
    }
}

pub(super) enum JumpingEndState {
    Landing(RedHatBoyState<Running>),
    Jumping(RedHatBoyState<Jumping>),
}

impl RedHatBoyState<Jumping> {
    pub(crate) fn land_on(self, position: f32) -> RedHatBoyState<Running> {
        RedHatBoyState {
            context: self.context.reset_frame().set_on(position as i16),
            _state: Running {},
        }
    }

    pub(super) fn frame_name(&self) -> &str {
        JUMPING_FRAME_NAME
    }

    pub(super) fn update(mut self) -> JumpingEndState {
        self.context = self.context.update(JUMPING_FRAMES);

        if self.context.position.y >= FLOOR {
            JumpingEndState::Landing(self.land_on(HEIGHT.into()))
        } else {
            JumpingEndState::Jumping(self)
        }
    }

    pub(super) fn knock_out(self) -> RedHatBoyState<Falling> {
        RedHatBoyState {
            context: self.context.reset_frame().stop(),
            _state: Falling {},
        }
    }
}

pub(super) enum FallingEndState {
    Falling(RedHatBoyState<Falling>),
    Complete(RedHatBoyState<KnockedOut>),
}
impl RedHatBoyState<Falling> {
    pub(super) fn frame_name(&self) -> &str {
        FALLING_FRAME_NAME
    }

    fn end(&self) -> RedHatBoyState<KnockedOut> {
        RedHatBoyState {
            context: self.context,
            _state: KnockedOut {},
        }
    }

    pub(super) fn update(mut self) -> FallingEndState {
        self.context = self.context.update(FALLING_FRAMES);

        if self.context.frame >= FALLING_FRAMES {
            FallingEndState::Complete(self.end())
        } else {
            FallingEndState::Falling(self)
        }
    }
}

impl RedHatBoyState<KnockedOut> {
    pub(super) fn frame_name(&self) -> &str {
        FALLING_FRAME_NAME
    }
}
