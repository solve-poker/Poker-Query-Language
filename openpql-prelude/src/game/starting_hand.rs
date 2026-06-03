use std::sync::LazyLock;

#[cfg(feature = "rayon")]
use rayon::prelude::*;
use rustc_hash::FxHashSet;

use crate::{Card, Game, HandN, IsomorphicCard, IsomorphicHandN};

const N_SD: usize = 2;
const N_HOLDEM: usize = 2;
const N_OMAHA: usize = 4;

fn collect_hands<const SD: bool, const N: usize>() -> Vec<Vec<Card>> {
    #[cfg(feature = "rayon")]
    {
        HandN::<N>::iter_all::<SD>()
            .into_par_iter()
            .map(|h| h.to_vec())
            .collect()
    }
    #[cfg(not(feature = "rayon"))]
    {
        HandN::<N>::iter_all::<SD>().map(|h| h.to_vec()).collect()
    }
}

static ALL_HANDS_SHORTDECK: LazyLock<Vec<Vec<Card>>> =
    LazyLock::new(collect_hands::<true, N_SD>);

static ALL_HANDS_HOLDEM: LazyLock<Vec<Vec<Card>>> =
    LazyLock::new(collect_hands::<false, N_HOLDEM>);

static ALL_HANDS_OMAHA: LazyLock<Vec<Vec<Card>>> =
    LazyLock::new(collect_hands::<false, N_OMAHA>);

fn all_hands(game: Game) -> &'static [Vec<Card>] {
    match game {
        Game::Holdem => &ALL_HANDS_HOLDEM,
        Game::Omaha => &ALL_HANDS_OMAHA,
        Game::ShortDeck => &ALL_HANDS_SHORTDECK,
    }
}

fn iso_hands<const N: usize>(
    hands: &[Vec<Card>],
    to_iso: impl Fn(&[Card]) -> IsomorphicHandN<N> + Sync,
) -> Vec<Vec<IsomorphicCard>> {
    #[cfg(feature = "rayon")]
    let isos: Vec<IsomorphicHandN<N>> =
        hands.par_iter().map(|h| to_iso(h)).collect();
    #[cfg(not(feature = "rayon"))]
    let isos: Vec<IsomorphicHandN<N>> =
        hands.iter().map(|h| to_iso(h)).collect();

    let mut seen = FxHashSet::default();
    isos.into_iter()
        .filter(|iso| seen.insert(*iso))
        .map(|iso| iso.0.to_vec())
        .collect()
}

static ALL_HANDS_SHORTDECK_ISO: LazyLock<Vec<Vec<IsomorphicCard>>> =
    LazyLock::new(|| {
        iso_hands::<N_SD>(
            &ALL_HANDS_SHORTDECK,
            IsomorphicHandN::<N_SD>::from_slice_preflop,
        )
    });

static ALL_HANDS_HOLDEM_ISO: LazyLock<Vec<Vec<IsomorphicCard>>> =
    LazyLock::new(|| {
        iso_hands::<N_HOLDEM>(
            &ALL_HANDS_HOLDEM,
            IsomorphicHandN::<N_HOLDEM>::from_slice_preflop,
        )
    });

static ALL_HANDS_OMAHA_ISO: LazyLock<Vec<Vec<IsomorphicCard>>> =
    LazyLock::new(|| {
        iso_hands::<N_OMAHA>(
            &ALL_HANDS_OMAHA,
            IsomorphicHandN::<N_OMAHA>::from_slice_preflop,
        )
    });

fn all_iso_hands(game: Game) -> &'static [Vec<IsomorphicCard>] {
    match game {
        Game::Holdem => &ALL_HANDS_HOLDEM_ISO,
        Game::Omaha => &ALL_HANDS_OMAHA_ISO,
        Game::ShortDeck => &ALL_HANDS_SHORTDECK_ISO,
    }
}

impl Game {
    /// Returns every legal starting hand for this variant.
    #[must_use]
    pub fn starting_hands(self) -> &'static [Vec<Card>] {
        all_hands(self)
    }

    #[must_use]
    pub fn starting_iso_hands(self) -> &'static [Vec<IsomorphicCard>] {
        all_iso_hands(self)
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

    #[test]
    fn test_starting_iso_hands() {
        for game in [Game::ShortDeck, Game::Holdem, Game::Omaha] {
            let n = match game {
                Game::Omaha => N_OMAHA,
                _ => N_HOLDEM,
            };
            assert!(game.starting_iso_hands().iter().all(|h| h.len() == n));
        }

        assert_eq!(Game::ShortDeck.starting_iso_hands().len(), 81);
        assert_eq!(Game::Holdem.starting_iso_hands().len(), 169);
        assert_eq!(Game::Omaha.starting_iso_hands().len(), 16_718);
    }
}
