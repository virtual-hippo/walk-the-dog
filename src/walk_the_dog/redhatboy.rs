use crate::engine::{Rect, Renderer};
use crate::walk_the_dog::*;

use web_sys::HtmlImageElement;

pub(crate) struct RedHatBoy {
    state_machine: RedHatBoyStateMachine,
    sprite_sheet: Sheet,
    image: HtmlImageElement,
}

impl RedHatBoy {
    pub(super) fn new(sprite_sheet: Sheet, image: HtmlImageElement) -> Self {
        RedHatBoy {
            state_machine: RedHatBoyStateMachine::Idle(RedHatBoyState::new()),
            sprite_sheet,
            image,
        }
    }

    pub(super) fn update(&mut self) {
        self.state_machine = self.state_machine.update();
    }

    pub(super) fn draw(&self, renderer: &Renderer) {
        let frame_name = format!(
            "{} ({}).png",
            self.state_machine.frame_name(),
            (self.state_machine.context().frame / 3) + 1
        );

        let splite = self
            .sprite_sheet
            .frames
            .get(&frame_name)
            .expect("Cell not found");

        let _ = renderer.draw_image(
            &self.image,
            &Rect::new(
                splite.frame.x.into(),
                splite.frame.y.into(),
                splite.frame.w.into(),
                splite.frame.h.into(),
            ),
            &Rect::new(
                (self.state_machine.context().position.x + splite.sprite_source_size.x as i16)
                    .into(),
                (self.state_machine.context().position.y + splite.sprite_source_size.y as i16)
                    .into(),
                splite.frame.w.into(),
                splite.frame.h.into(),
            ),
        );
    }

    pub(super) fn run_right(&mut self) {
        self.state_machine = self.state_machine.transition(Event::Run);
    }

    pub(super) fn slide(&mut self) {
        self.state_machine = self.state_machine.transition(Event::Slide);
    }

    pub(super) fn jump(&mut self) {
        self.state_machine = self.state_machine.transition(Event::Jump);
    }
}

#[derive(Clone, Copy)]
enum RedHatBoyStateMachine {
    Idle(RedHatBoyState<Idle>),
    Running(RedHatBoyState<Running>),
    Sliding(RedHatBoyState<Sliding>),
    Jumping(RedHatBoyState<Jumping>),
}

enum Event {
    Run,
    Slide,
    Update,
    Jump,
}

impl From<RedHatBoyState<Idle>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Idle>) -> Self {
        RedHatBoyStateMachine::Idle(state)
    }
}

impl From<RedHatBoyState<Running>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Running>) -> Self {
        RedHatBoyStateMachine::Running(state)
    }
}

impl From<RedHatBoyState<Sliding>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Sliding>) -> Self {
        RedHatBoyStateMachine::Sliding(state)
    }
}

impl From<SlidingEndState> for RedHatBoyStateMachine {
    fn from(end_state: SlidingEndState) -> Self {
        match end_state {
            SlidingEndState::Complete(state) => state.into(),
            SlidingEndState::Sliding(state) => state.into(),
        }
    }
}

impl From<RedHatBoyState<Jumping>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Jumping>) -> Self {
        RedHatBoyStateMachine::Jumping(state)
    }
}

impl From<JumpingEndState> for RedHatBoyStateMachine {
    fn from(end_state: JumpingEndState) -> Self {
        match end_state {
            JumpingEndState::Landing(state) => state.into(),
            JumpingEndState::Jumping(state) => state.into(),
        }
    }
}

impl RedHatBoyStateMachine {
    fn transition(self, event: Event) -> Self {
        match (self, event) {
            (RedHatBoyStateMachine::Idle(state), Event::Run) => state.run().into(),
            (RedHatBoyStateMachine::Running(state), Event::Slide) => state.slide().into(),
            (RedHatBoyStateMachine::Running(state), Event::Jump) => state.jump().into(),
            (RedHatBoyStateMachine::Idle(state), Event::Update) => state.update().into(),
            (RedHatBoyStateMachine::Running(state), Event::Update) => state.update().into(),
            (RedHatBoyStateMachine::Sliding(state), Event::Update) => state.update().into(),
            (RedHatBoyStateMachine::Jumping(state), Event::Update) => state.update().into(),
            _ => self,
        }
    }

    fn frame_name(&self) -> &str {
        match self {
            RedHatBoyStateMachine::Idle(state) => &state.frame_name(),
            RedHatBoyStateMachine::Running(state) => &state.frame_name(),
            RedHatBoyStateMachine::Sliding(state) => &state.frame_name(),
            RedHatBoyStateMachine::Jumping(state) => &state.frame_name(),
        }
    }

    fn context(&self) -> &RedHatBoyContext {
        match self {
            RedHatBoyStateMachine::Idle(state) => &state.context(),
            RedHatBoyStateMachine::Running(state) => &state.context(),
            RedHatBoyStateMachine::Sliding(state) => &state.context(),
            RedHatBoyStateMachine::Jumping(state) => &state.context(),
        }
    }

    fn update(self) -> Self {
        self.transition(Event::Update)
    }
}
