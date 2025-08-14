// TODO: remove later
#![allow(clippy::fallible_impl_from)]

use super::*;

pub type PlayerHands = Vec<PlayerHand>;
pub type PlayerHand = Vec<Card>;

#[derive(Debug, Clone, Default)]
pub struct BufferRatings(Vec<PQLHiRating>);

impl BufferRatings {
    pub fn new(n: usize) -> Self {
        Self(vec![PQLHiRating::default(); n])
    }

    pub fn iter(&self) -> impl Iterator<Item = &PQLHiRating> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut PQLHiRating> {
        self.0.iter_mut()
    }

    pub fn max(&self) -> PQLHiRating {
        *self.0.iter().max().unwrap()
    }
}

impl Index<PQLPlayer> for BufferRatings {
    type Output = PQLHiRating;

    fn index(&self, index: PQLPlayer) -> &Self::Output {
        &self.0[index.to_usize()]
    }
}

#[derive(Debug, Clone, Default)]
pub struct VmBuffer {
    pub game: PQLGame,
    pub player_hands: PlayerHands,
    pub board: Board,
    pub dead: DeadCards,
    pub ratings: BufferRatings,
}

impl VmBuffer {
    pub fn new(n_players: usize, game: PQLGame) -> Self {
        Self {
            game,
            player_hands: vec![vec![]; n_players],
            board: Board::default(),
            dead: DeadCards::default(),
            ratings: BufferRatings::new(n_players),
        }
    }

    #[inline]
    pub fn player_hand(&self, player: PQLPlayer) -> &Hand {
        &self.player_hands[player.to_usize()]
    }
}

impl From<&VmBuffer> for Flop {
    fn from(buf: &VmBuffer) -> Self {
        buf.board.flop.unwrap()
    }
}

impl From<&VmBuffer> for Board {
    fn from(buf: &VmBuffer) -> Self {
        buf.board
    }
}

impl From<&VmBuffer> for PQLGame {
    fn from(buf: &VmBuffer) -> Self {
        buf.game
    }
}

impl From<&VmBuffer> for DeadCards {
    fn from(buf: &VmBuffer) -> Self {
        buf.dead
    }
}

impl From<&VmBuffer> for (PQLGame, Flop) {
    fn from(buf: &VmBuffer) -> Self {
        (buf.game, buf.board.flop.unwrap())
    }
}

impl From<&VmBuffer> for (PQLGame, Board) {
    fn from(buf: &VmBuffer) -> Self {
        (buf.game, buf.into())
    }
}

impl From<&VmBuffer> for (PQLGame, Board, DeadCards) {
    fn from(buf: &VmBuffer) -> Self {
        (buf.game, buf.into(), buf.dead)
    }
}

impl<'b> From<&'b mut VmBuffer>
    for (PQLGame, Board, &'b PlayerHands, &'b mut BufferRatings)
{
    fn from(buf: &'b mut VmBuffer) -> Self {
        (
            buf.game,
            (&*buf).into(),
            &buf.player_hands,
            &mut buf.ratings,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_new() {
        let buf = VmBuffer::new(2, PQLGame::Holdem);

        assert_eq!(2, buf.player_hands.len());
        assert_eq!(PQLGame::Holdem, (&buf).into());
        assert_eq!(DeadCards::default(), (&buf).into());
    }

    #[test]
    fn test_player_hand() {
        let mut buf = VmBuffer::new(2, PQLGame::Holdem);

        let cs = cards!("As2s");

        buf.player_hands[0] = cs.clone();

        assert_eq!(cs, buf.player_hand(0.into()));
    }
}
