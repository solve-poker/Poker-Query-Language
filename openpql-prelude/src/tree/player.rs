use derive_more::Display;

pub use crate::PlayerIdx;

pub type PlayerCount = u8;

#[derive(
    Clone, Copy, Debug, Display, Default, Hash, PartialEq, Eq, derive_more::From,
)]
pub enum Player {
    #[default]
    Chance,
    Player(PlayerIdx),
    Terminal,
}
