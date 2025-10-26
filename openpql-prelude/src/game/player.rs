use super::{Add, AddAssign, Display, Into};

pub type PlayerIdx = u8;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    derive_more::From,
    Into,
    Display,
    Add,
    AddAssign,
)]
#[display("Player {_0}")]
pub struct Player(PlayerIdx);

impl Player {
    pub fn iter(n: PlayerIdx) -> impl Iterator<Item = Self> {
        (0..n).map(Self::from)
    }

    pub fn iter_opponents(self, n: PlayerIdx) -> impl Iterator<Item = Self> {
        Self::iter(n).filter(move |&p| p != self)
    }
}

impl From<usize> for Player {
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn from(i: usize) -> Self {
        debug_assert!(i < PlayerIdx::MAX.into());

        Self(i.to_le_bytes()[0])
    }
}

impl From<i32> for Player {
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn from(i: i32) -> Self {
        debug_assert!(i >= 0 && i < PlayerIdx::MAX.into());

        Self(i.to_le_bytes()[0])
    }
}

impl From<Player> for usize {
    fn from(player: Player) -> Self {
        player.0 as Self
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    impl Arbitrary for Player {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Self(PlayerIdx::arbitrary(g))
        }
    }

    #[test]
    fn test_conversion() {
        for i in 0..10 {
            let player = Player::from(i);
            assert_eq!(player.0, i);
            assert_eq!(usize::from(player), i as usize);
        }
    }

    #[quickcheck]
    fn test_iter(n: PlayerIdx) {
        let n_players = n % 10;
        let v: Vec<_> = Player::iter(n_players).collect();

        assert_eq!(v, (0..n_players).map(Player::from).collect::<Vec<_>>());
    }

    #[quickcheck]
    fn test_iter_opponents(n: PlayerIdx, player_idx: PlayerIdx) {
        let n_players = n % 10;
        if n_players == 0 {
            return;
        }

        let player = Player::from(player_idx % n_players);
        let opponents: Vec<_> = player.iter_opponents(n_players).collect();

        assert!(!opponents.contains(&player));
        assert_eq!(opponents.len(), (n_players - 1) as usize);

        let expected: Vec<_> = (0..n_players)
            .map(Player::from)
            .filter(|&p| p != player)
            .collect();
        assert_eq!(opponents, expected);
    }
}
