mod barrier;
pub mod game;
mod obstacle;
mod platform;
pub mod redhatboy;
pub mod redhatboy_state;

pub(in crate::walk_the_dog) use barrier::*;
pub(super) use game::WalkTheDog;
pub(in crate::walk_the_dog) use obstacle::*;
pub(in crate::walk_the_dog) use platform::*;
pub(super) use redhatboy::RedHatBoy;
pub(in crate::walk_the_dog) use redhatboy_state::*;
