use crate::engine::{Rect, Renderer};
use crate::walk_the_dog::*;

use web_sys::HtmlImageElement;

pub(super) struct RedHatBoy {
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
                self.state_machine.context().position.x.into(),
                self.state_machine.context().position.y.into(),
                splite.frame.w.into(),
                splite.frame.h.into(),
            ),
        );
    }

    pub(super) fn run_right(&mut self) {
        self.state_machine = self.state_machine.transition(Event::Run);
    }
}

#[derive(Clone, Copy)]
enum RedHatBoyStateMachine {
    Idle(RedHatBoyState<Idle>),
    Running(RedHatBoyState<Running>),
}

enum Event {
    Run,
}

impl From<RedHatBoyState<Running>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Running>) -> Self {
        RedHatBoyStateMachine::Running(state)
    }
}

impl RedHatBoyStateMachine {
    fn transition(self, event: Event) -> Self {
        match (self, event) {
            (RedHatBoyStateMachine::Idle(state), Event::Run) => state.run().into(),
            _ => self,
        }
    }

    fn frame_name(&self) -> &str {
        match self {
            RedHatBoyStateMachine::Idle(state) => &state.frame_name(),
            RedHatBoyStateMachine::Running(state) => &state.frame_name(),
        }
    }

    fn context(&self) -> &RedHatBoyContext {
        match self {
            RedHatBoyStateMachine::Idle(state) => &state.context(),
            RedHatBoyStateMachine::Running(state) => &state.context(),
        }
    }

    fn update(self) -> Self {
        match self {
            RedHatBoyStateMachine::Idle(mut state) => {
                state.update();
                RedHatBoyStateMachine::Idle(state)
            }
            RedHatBoyStateMachine::Running(mut state) => {
                state.update();
                RedHatBoyStateMachine::Running(state)
            }
        }
    }
}
