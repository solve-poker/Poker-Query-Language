use std::sync::Arc;

use openpql_prelude::IsomorphicHand;

use super::*;

mod sharded_map;

use sharded_map::ShardedMap;

type RatingKey = (IsomorphicHand, IsomorphicHand);

/// Per-statement cache of hand ratings keyed by (player hand, board),
/// shared by cloned [`Vm`]s across threads.
#[derive(Clone, Debug, Default)]
pub struct VmCache {
    ratings: Arc<ShardedMap<RatingKey, PQLHiRating>>,
}

impl VmCache {
    /// Returns the rating for `(player, board)`, computing it via `eval`
    /// on a miss.
    pub fn rating_or_insert_with(
        &self,
        game: PQLGame,
        player: &[PQLCard],
        board: PQLBoard,
    ) -> PQLHiRating {
        let (iso_board, map) = board.to_isomorphic_ev();
        let iso_player = IsomorphicHand::from_slice_and_map(player, map);
        let key = (iso_board, iso_player);

        self.ratings.get(&key).unwrap_or_else(|| {
            let rating = game.eval_rating(player.into(), board.to_card64());

            self.ratings.insert(key, rating);

            rating
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::atomic::{AtomicUsize, Ordering},
        thread,
    };

    use openpql_prelude::{board, cards};

    use super::*;

    #[quickcheck]
    fn test_shared_across_threads(cards: CardN<7>) {
        let cache = VmCache::default();
        let game = PQLGame::Holdem;
        let player = &cards[0..2];
        let board = PQLBoard::from_slice(&cards[2..]);
        let rating = game.eval_rating(player.into(), board.to_card64());

        thread::scope(|s| {
            for _ in 0..4 {
                let cache = cache.clone();

                s.spawn(move || {
                    for _ in 0..100 {
                        assert_eq!(
                            cache.rating_or_insert_with(game, player, board),
                            rating
                        );
                    }
                });
            }
        });
    }
}
