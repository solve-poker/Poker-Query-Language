use super::*;

#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, From, Into, AsRef, AsMut,
)]
pub struct DeadCards(Card64);

impl From<&[Card]> for DeadCards {
    fn from(cs: &[Card]) -> Self {
        Card64::from(cs).into()
    }
}

pub type PQLDeadCards = DeadCards;
