pub type Chip = u16;

mod action;
mod annotated_action;
mod annotated_action_kind;
mod error;
mod functions;
mod history;
mod player;
mod street;

pub use action::*;
pub use annotated_action::*;
pub use annotated_action_kind::*;
pub use error::*;
pub use functions::*;
pub use history::*;
pub use player::*;
#[cfg(test)]
pub use tests::*;

fn to_pid(i: usize) -> PlayerIdx {
    PlayerCount::try_from(i).unwrap()
}

#[cfg(test)]
pub mod tests {
    pub use quickcheck_macros::quickcheck;

    use super::*;
    use crate::Street;

    pub const PREFLOP: AnnotatedAction =
        AnnotatedAction::Chance(Street::Preflop);
    pub const FLOP: AnnotatedAction = AnnotatedAction::Chance(Street::Flop);
    pub const RIVR: AnnotatedAction = AnnotatedAction::Chance(Street::River);
}
