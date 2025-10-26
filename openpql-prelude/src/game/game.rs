use super::{
    Board, Card64, CardCount, FlopHandCategory, FromStr, HandRating,
    ParseError, eval_flop_holdem, eval_flop_omaha, eval_holdem, eval_omaha,
    eval_shortdeck,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Game {
    #[default]
    Holdem,
    Omaha,
    ShortDeck,
}

impl Game {
    pub const fn player_cards_len(self) -> CardCount {
        match self {
            Self::Holdem | Self::ShortDeck => 2,
            Self::Omaha => 4,
        }
    }

    pub const fn is_shortdeck(self) -> bool {
        matches!(self, Self::ShortDeck)
    }

    pub fn eval_rating(self, player: Card64, board: Card64) -> HandRating {
        match self {
            Self::Holdem => eval_holdem(player | board),
            Self::ShortDeck => eval_shortdeck(player | board),
            Self::Omaha => eval_omaha(player, board),
        }
    }

    pub fn eval_flop_category(
        self,
        player: Card64,
        board: Board,
    ) -> FlopHandCategory {
        match self {
            Self::Holdem | Self::ShortDeck => eval_flop_holdem(player, board),
            Self::Omaha => eval_flop_omaha(player, board),
        }
    }
}

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().trim() {
            "holdem" => Ok(Self::Holdem),
            "omaha" => Ok(Self::Omaha),
            "shortdeck" => Ok(Self::ShortDeck),
            _ => Err(ParseError::InvalidGame(s.into())),
        }
    }
}

#[cfg(any(test, feature = "quickcheck"))]
impl quickcheck::Arbitrary for Game {
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        #[allow(unused)]
        const fn completeness_check(e: Game) {
            match e {
                Game::Holdem | Game::Omaha | Game::ShortDeck => (),
            }
        }

        *g.choose(&[Self::Holdem, Self::Omaha, Self::ShortDeck])
            .unwrap()
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_player_cards_len() {
        assert_eq!(2, Game::Holdem.player_cards_len());
        assert_eq!(4, Game::Omaha.player_cards_len());
        assert_eq!(2, Game::ShortDeck.player_cards_len());
    }

    #[test]
    fn test_is_shortdeck() {
        assert!(!Game::Holdem.is_shortdeck());
        assert!(!Game::Omaha.is_shortdeck());
        assert!(Game::ShortDeck.is_shortdeck());
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Ok(Game::Holdem), " HoldEM ".parse());

        assert_eq!(Ok(Game::Omaha), "omaha".parse());
        assert_eq!(Ok(Game::ShortDeck), "shortdeck".parse());

        assert_eq!(
            Err(ParseError::InvalidGame("unknown".into())),
            "unknown".parse::<Game>()
        );
    }

    #[quickcheck]
    fn test_eval_rating_args(cs: CardN<5>) {
        let c5 = Card64::from(cs.as_slice());
        let (c3, c2): (CardN<3>, CardN<2>) = cs.into();
        let c3 = Card64::from(c3);
        let c2 = Card64::from(c2);

        let r1 = Game::Holdem.eval_rating(c3, c2);
        let r2 = Game::Holdem.eval_rating(c5, Card64::default());
        let r3 = Game::Holdem.eval_rating(Card64::default(), c5);

        assert_eq!(r1, r2);
        assert_eq!(r1, r3);
    }

    #[test]
    fn test_eval_rating() {
        assert_eq!(
            Game::Omaha
                .eval_rating(c64!("Ks Qh 8s 9h"), c64!("7h 7c 7d As Ah")),
            mk_rating(HandType::Trips, "7", "KQ")
        );
        assert_eq!(
            Game::ShortDeck
                .eval_rating(c64!("Kh As Ah Ac Ad 6d 6c"), Card64::default()),
            mk_rating(HandType::Quads, "A", "K")
        );
    }

    #[test]
    fn test_eval_flop_cat() {
        assert_eq!(
            Game::Holdem.eval_flop_category(c64!("7c 8c"), board!("7s 8h Tc")),
            FlopHandCategory::BottomTwo,
        );

        assert_eq!(
            Game::Omaha
                .eval_flop_category(c64!("7c 8c 2s 3s"), board!("7s 8h Tc")),
            FlopHandCategory::BottomTwo,
        );
    }
}
