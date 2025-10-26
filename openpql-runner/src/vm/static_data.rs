use super::*;

type Trials = usize;

/// Static Data
/// stores necessary values for each simulation like game config and dead cards
#[derive(Clone, derive_more::Debug, derive_more::From)]
pub struct VmStaticData {
    pub game: PQLGame,
    pub player_names: Vec<PlayerName>,
    pub player_ranges: Vec<PQLRange>,
    pub board_range: PQLBoardRange,
    pub dead_card: PQLCardSet,
    pub n_players: PQLPlayerCount,
    pub n_trails: Trials,
}

impl VmStaticData {
    #[cfg(not(debug_assertions))]
    pub const DEFAULT_N_TRIALS: Trials = 60000;

    #[cfg(debug_assertions)]
    pub const DEFAULT_N_TRIALS: Trials = 100;

    #[allow(clippy::cast_possible_truncation)]
    pub(crate) fn find_player(&self, name: &str) -> Option<PQLPlayer> {
        self.player_names
            .iter()
            .position(|n| n == name)
            .map(|idx| PQLPlayer::from(idx as PlayerIdx))
    }

    fn get_game(fc: &ast::FromClause<'_>) -> PQLResult<PQLGame> {
        fc.get_game().map_or_else(
            || Ok(PQLGame::default()),
            |game| with_loc(game, || game.inner.parse::<PQLGame>()),
        )
    }

    fn get_board(
        game: PQLGame,
        fc: &ast::FromClause<'_>,
    ) -> PQLResult<PQLBoardRange> {
        fc.get_board_range().map_or_else(
            || Ok((game, "*").try_into().unwrap()),
            |e| with_loc(e, || (game, e.inner).try_into()),
        )
    }

    fn get_players(
        game: PQLGame,
        fc: &ast::FromClause<'_>,
    ) -> PQLResult<(Vec<PlayerName>, Vec<PQLRange>)> {
        fc.get_players()
            .into_iter()
            .map(|(name, range)| {
                with_loc(range, || (game, range.inner).try_into())
                    .map(|pql_range| (name.inner.to_string(), pql_range))
            })
            .collect()
    }

    fn get_deadcard(fc: &ast::FromClause<'_>) -> PQLResult<PQLCardSet> {
        fc.get_dead().map_or_else(
            || Ok(PQLCardSet::default()),
            |e| {
                with_loc(e, || {
                    parse_cards(e.inner).ok_or(PQLErrorKind::InvalidDeadcards)
                })
            },
        )
    }
}

impl Default for VmStaticData {
    fn default() -> Self {
        Self {
            game: PQLGame::default(),
            player_names: vec![],
            player_ranges: vec![],
            board_range: PQLBoardRange::default(),
            dead_card: PQLCardSet::default(),
            n_trails: Self::DEFAULT_N_TRIALS,
            n_players: PQLPlayerCount::default(),
        }
    }
}

impl TryFrom<&ast::FromClause<'_>> for VmStaticData {
    type Error = PQLError;

    fn try_from(expr: &ast::FromClause) -> Result<Self, Self::Error> {
        let game = Self::get_game(expr)?;
        let (player_names, player_ranges) = Self::get_players(game, expr)?;
        let board_range = Self::get_board(game, expr)?;
        let dead_card = Self::get_deadcard(expr)?;
        let n_usize = player_names.len();
        let n_players = if n_usize <= 10 {
            PQLPlayerCount::try_from(n_usize).unwrap()
        } else {
            return Err((
                expr.loc,
                PQLErrorKind::ExceededMaximumPlayers(n_usize),
            )
                .into());
        };

        Ok(Self {
            game,
            player_names,
            player_ranges,
            board_range,
            dead_card,
            n_players,
            ..Default::default()
        })
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
pub mod tests {
    use std::default;

    use super::*;
    use crate::*;

    #[test]
    fn test_game() {
        assert_eq!(
            VmStaticData::get_game(
                &parse_from_clause("from game='  HOLDEM '").unwrap()
            ),
            Ok(PQLGame::Holdem),
            "should trim and ignore case"
        );

        assert_eq!(
            VmStaticData::get_game(
                &parse_from_clause("from key='val'").unwrap()
            ),
            Ok(PQLGame::Holdem),
            "should default to holdem"
        );
    }

    #[quickcheck]
    fn test_board_default(cards: CardN<5>) {
        let fc = &parse_from_clause("from game='holdem'").unwrap();
        let board_range = VmStaticData::get_board(PQLGame::Holdem, fc).unwrap();
        let board_range_sd =
            VmStaticData::get_board(PQLGame::ShortDeck, fc).unwrap();

        if cards.as_slice().iter().all(|c| c.rank >= PQLRank::R6) {
            assert!(board_range_sd.is_satisfied(cards.as_slice()));
        }

        assert!(board_range.is_satisfied(cards.as_slice()));
    }

    #[test]
    fn test_board() {
        let fc =
            &parse_from_clause("from game='holdem', board='AKQJT'").unwrap();
        let board_range = VmStaticData::get_board(PQLGame::Holdem, fc).unwrap();

        assert!(board_range.is_satisfied(&cards!("As Ks Qs Js Ts")));
        assert!(board_range.is_satisfied(&cards!("Qs Ks As Js Ts")));
        assert!(!board_range.is_satisfied(&cards!("Qs Ks As Ts Js")));
    }

    #[test]
    fn test_players() {
        let game = PQLGame::Holdem;
        let res = VmStaticData::get_players(
            game,
            &parse_from_clause("from p1='AA', p2='KK'").unwrap(),
        )
        .unwrap();

        assert!(res.0.contains(&"p1".into()));
        assert!(res.0.contains(&"p2".into()));

        let (ranges, _) = mk_ranges(game, &["AA", "KK"], "*");
        assert!(
            res.1
                .iter()
                .all(|range| range.src_eq(&ranges[0])
                    || range.src_eq(&ranges[1]))
        );
    }

    #[test]
    fn test_deadcards() {
        fn assert_invalid(src: &str) {
            assert_eq!(
                VmStaticData::get_deadcard(&parse_from_clause(src).unwrap(),)
                    .unwrap_err()
                    .kind,
                PQLErrorKind::InvalidDeadcards
            );
        }

        assert_invalid("from dead='A'");
        assert_invalid("from dead='BS'");

        assert_eq!(
            VmStaticData::get_deadcard(
                &parse_from_clause("from dead='AA'").unwrap(),
            ),
            Err(((10, 14), PQLErrorKind::InvalidDeadcards).into())
        );

        assert_eq!(
            VmStaticData::get_deadcard(
                &parse_from_clause("from game='holdem'").unwrap(),
            ),
            Ok(PQLCardSet::default())
        );

        assert_eq!(
            VmStaticData::get_deadcard(
                &parse_from_clause("from dead='As aH'").unwrap(),
            ),
            Ok(c64!("As Ah"))
        );
    }

    #[test]
    fn test_default() {
        let obj = VmStaticData::default();
        let default_board = (PQLGame::default(), "*").try_into().unwrap();

        assert!(obj.board_range.src_eq(&default_board));
    }

    #[test]
    fn test_from() {
        let expr = parse_from_clause(
            "
            from
              game = 'omaha',
              board = 'AAA',
              p0 = 'A',
              p1 = 'KK',
              dead = 'As',
            ",
        )
        .unwrap();

        let res = VmStaticData::try_from(&expr).unwrap();

        assert_eq!(res.game, PQLGame::Omaha);
    }

    fn assert_err<E>(src: &str, err: E, err_src: &str)
    where
        PQLErrorKind: From<E>,
    {
        let expr = parse_from_clause(src).unwrap();
        let res = VmStaticData::try_from(&expr);

        let pos_s = src.find(err_src).unwrap();
        let pos_e = pos_s + err_src.len();

        assert_eq!(res.unwrap_err(), ((pos_s, pos_e), err).into());
    }

    #[test]
    fn test_err() {
        assert_err(
            "from game='invalid'",
            ParseError::InvalidGame("invalid".to_string()),
            "'invalid'",
        );

        assert_err(
            "from board='AAAAKK'",
            RangeError::TooManyCardsInRange((0, 6)),
            "'AAAAKK'",
        );

        assert_err(
            "from p0='AA', p1='AAK'",
            RangeError::TooManyCardsInRange((0, 3)),
            "'AAK'",
        );

        assert_err(
            "from p0='AA', p1='KK', dead='BB'",
            PQLErrorKind::InvalidDeadcards,
            "'BB'",
        );

        let mut src = String::new();
        for i in 0..11 {
            write!(&mut src, "p{i}='*',").unwrap();
        }

        assert_err(
            &format!("from {src}",),
            PQLErrorKind::ExceededMaximumPlayers(11),
            &src,
        );
    }
}
