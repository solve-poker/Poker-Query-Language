use super::*;

#[derive(Debug, Clone, Eq, PartialEq, From)]
pub enum PQLRange {
    Hand2(CachedChecker<2>),
    Hand4(CachedChecker<4>),
    Board(PQLBoardRange),
}

impl PQLRange {
    pub fn is_satisfied(&mut self, hand: &Hand) -> bool {
        match self {
            Self::Hand2(v) => v.is_satisfied(hand),
            Self::Hand4(v) => v.is_satisfied(hand),
            Self::Board(v) => v.is_satisfied(hand),
        }
    }

    pub(crate) fn from_src(
        src: &str,
        game: PQLGame,
    ) -> Result<Self, range_parser::Error> {
        match game {
            PQLGame::Holdem | PQLGame::ShortDeck => {
                Ok(Self::Hand2(CachedChecker::from_src(src)?))
            }
            PQLGame::Omaha => Ok(Self::Hand4(CachedChecker::from_src(src)?)),
        }
    }
}

impl Default for PQLRange {
    fn default() -> Self {
        Self::Hand2(CachedChecker::<2>::from_src("*").unwrap())
    }
}
