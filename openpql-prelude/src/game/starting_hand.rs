use std::sync::LazyLock;

use crate::{Card, Game, HandN};

const N_SD: usize = 2;
const N_HOLDEM: usize = 2;
const N_OMAHA: usize = 4;

static ALL_HANDS_SHORTDECK: LazyLock<Vec<Vec<Card>>> = LazyLock::new(|| {
    HandN::<N_SD>::iter_all::<true>()
        .map(|h| h.to_vec())
        .collect()
});

static ALL_HANDS_HOLDEM: LazyLock<Vec<Vec<Card>>> = LazyLock::new(|| {
    HandN::<N_HOLDEM>::iter_all::<false>()
        .map(|h| h.to_vec())
        .collect()
});

static ALL_HANDS_OMAHA: LazyLock<Vec<Vec<Card>>> = LazyLock::new(|| {
    HandN::<N_OMAHA>::iter_all::<false>()
        .map(|h| h.to_vec())
        .collect()
});

fn all_hands(game: Game) -> &'static [Vec<Card>] {
    match game {
        Game::Holdem => &ALL_HANDS_HOLDEM,
        Game::Omaha => &ALL_HANDS_OMAHA,
        Game::ShortDeck => &ALL_HANDS_SHORTDECK,
    }
}

impl Game {
    pub fn starting_hands(self) -> &'static [Vec<Card>] {
        all_hands(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starting_hands() {
        assert_eq!(Game::ShortDeck.starting_hands().len(), 630);
        assert_eq!(Game::Holdem.starting_hands().len(), 1326);
        assert_eq!(Game::Omaha.starting_hands().len(), 270_725);
    }
}
