use crate::engine::Point;

const FLOOR: i16 = 475;

const IDLE_FRAME_NAME: &str = "Idle";
const RUN_FRAME_NAME: &str = "Run";

const IDLE_FRAMES: u8 = 29;
const RUNNING_FRAMES: u8 = 23;

const RUNNING_SPEED: i16 = 23;

#[derive(Clone, Copy)]
pub(super) struct Idle;

#[derive(Clone, Copy)]
pub(super) struct Running;

#[derive(Clone, Copy)]
pub(super) struct RedHatBoyContext {
    pub(super) frame: u8,
    pub(super) position: Point,
    pub(super) velocity: Point,
}

impl RedHatBoyContext {
    pub(super) fn update(mut self, frame_count: u8) -> Self {
        if self.frame < frame_count {
            self.frame += 1;
        } else {
            self.frame = 0;
        }

        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;

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
                position: Point { x: 0, y: FLOOR },
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

    pub(super) fn update(&mut self) {
        self.context = self.context.update(IDLE_FRAMES)
    }
}

impl RedHatBoyState<Running> {
    pub(super) fn frame_name(&self) -> &str {
        RUN_FRAME_NAME
    }

    pub(super) fn update(&mut self) {
        self.context = self.context.update(RUNNING_FRAMES)
    }
}
