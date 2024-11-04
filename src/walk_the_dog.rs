mod barrier;
pub mod game;
mod obstacle;
mod platform;
pub mod redhatboy;
pub mod redhatboy_state;
#[cfg(test)]
pub(super) mod test_browser;
mod walk_the_dog_state;
mod walk_the_dog_state_machine;

pub(super) use barrier::*;
pub(super) use game::WalkTheDog;
pub(super) use obstacle::*;
pub(super) use platform::*;
pub(super) use redhatboy::RedHatBoy;
pub(in crate::walk_the_dog) use redhatboy_state::*;
pub(in crate::walk_the_dog) use walk_the_dog_state_machine::*;

#[cfg(not(test))]
pub(super) use crate::browser;
#[cfg(test)]
pub(super) use test_browser as browser;
