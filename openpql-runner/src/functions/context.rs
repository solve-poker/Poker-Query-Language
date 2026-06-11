use super::*;

#[derive(Clone, Debug)]
pub struct PQLFnContext<'vm> {
    pub game: PQLGame,
    pub sampled_cards: &'vm [PQLCard],
    pub n_players: PQLPlayerCount,
    pub cache: &'vm VmCache,
}

impl PQLFnContext<'_> {
    /// Cached [`PQLGame::eval_rating`]; hits are shared across functions,
    /// trials and threads via [`VmCache`].
    pub fn eval_current_rating(
        &self,
        player: PQLPlayer,
        street: PQLStreet,
    ) -> PQLHiRating {
        self.cache.rating_or_insert_with(
            self.game,
            self.get_player_slice(player),
            self.get_board(street),
        )
    }

    /// Cached rating for an arbitrary `player`/`board`, sharing hits via
    /// [`VmCache`].
    pub fn eval_rating(
        &self,
        player: &[PQLCard],
        board: PQLBoard,
    ) -> PQLHiRating {
        self.cache.rating_or_insert_with(self.game, player, board)
    }

    const fn n_holecards(&self) -> PQLCardCount {
        self.game.player_cards_len()
    }

    const fn get_idx_board_start(&self) -> usize {
        Self::idx_board_start(self.n_players, self.n_holecards())
    }

    pub const fn idx_board_start(
        n_players: PQLPlayerCount,
        n_holecards: PQLCardCount,
    ) -> usize {
        (n_players * n_holecards) as usize
    }

    pub const fn n_total_cards(
        n_players: PQLPlayerCount,
        n_holecards: PQLCardCount,
    ) -> usize {
        Self::idx_board_start(n_players, n_holecards) + PQLBoard::N_RIVER
    }

    pub fn get_player_slice(&self, player: PQLPlayer) -> &[PQLCard] {
        let i: usize = player.into();
        let n = self.n_holecards() as usize;

        &self.sampled_cards[i * n..(i + 1) * n]
    }

    pub fn get_c64_player(&self, player: PQLPlayer) -> PQLCardSet {
        self.get_player_slice(player).into()
    }

    /// All players' hole cards as a single set.
    pub fn get_c64_players(&self) -> PQLCardSet {
        self.sampled_cards[..self.get_idx_board_start()].into()
    }

    pub fn get_board(&self, street: PQLStreet) -> PQLBoard {
        self.get_board_slice(street).into()
    }

    pub fn get_board_slice(&self, street: PQLStreet) -> &[PQLCard] {
        let i = self.get_idx_board_start();
        let n = street.board_card_count() as usize;

        &self.sampled_cards[i..i + n]
    }

    pub fn get_c64_board(&self, street: PQLStreet) -> PQLCardSet {
        self.get_board_slice(street).into()
    }

    // TODO: remove vec init
    pub fn iter_c64_player(&self) -> Vec<PQLCardSet> {
        match self.game {
            PQLGame::Holdem => HandN::<2>::iter_all::<false>()
                .map(PQLCardSet::from)
                .collect(),
            PQLGame::Omaha => HandN::<4>::iter_all::<false>()
                .map(PQLCardSet::from)
                .collect(),

            PQLGame::ShortDeck => HandN::<2>::iter_all::<true>()
                .map(PQLCardSet::from)
                .collect(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::*;

    impl Default for PQLFnContext<'_> {
        // TODO: replace Box::leak
        fn default() -> Self {
            Self {
                game: PQLGame::default(),
                sampled_cards: PQLCard::all::<true>(),
                n_players: 2,
                cache: Box::leak(Box::default()),
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct TestPQLFnContext {
        game: PQLGame,
        sampled_cards: Vec<PQLCard>,
        n_players: PQLPlayerCount,
        cache: VmCache,
    }

    impl TestPQLFnContext {
        pub fn as_ctx(&self) -> PQLFnContext<'_> {
            PQLFnContext {
                game: self.game,
                sampled_cards: &self.sampled_cards,
                n_players: self.n_players,
                cache: &self.cache,
            }
        }

        pub fn from_board(cards: &[PQLCard]) -> Self {
            Self {
                game: PQLGame::default(),
                sampled_cards: cards.to_vec(),
                n_players: 0,
                cache: VmCache::default(),
            }
        }

        pub fn from_cards(game: PQLGame, cards: Vec<PQLCard>) -> Self {
            let n_players =
                PQLCardCount::try_from(cards.len() - PQLBoard::N_RIVER)
                    .unwrap()
                    / game.player_cards_len();

            Self {
                game,
                sampled_cards: cards,
                n_players,
                cache: VmCache::default(),
            }
        }
    }

    impl Arbitrary for TestPQLFnContext {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            const MAX_PLAYER: PQLPlayerCount = 10;
            fn random_cards<const SD: bool>(
                rng: &mut impl rand::Rng,
                n: usize,
            ) -> Vec<PQLCard> {
                PQLCard::all::<SD>()
                    .iter()
                    .copied()
                    .sample(rng, n)
                    .into_iter()
                    .collect()
            }

            let game = PQLGame::arbitrary(g);

            let rng = &mut rand::rng();
            let n_players = rng.random_range(1..=MAX_PLAYER);

            let n_cards = (game.player_cards_len() * n_players) as usize
                + PQLBoard::N_RIVER;

            let sampled_cards = match game {
                PQLGame::ShortDeck => random_cards::<true>(rng, n_cards),
                _ => random_cards::<false>(rng, n_cards),
            };

            Self {
                game,
                sampled_cards,
                n_players,
                cache: VmCache::default(),
            }
        }
    }
}
