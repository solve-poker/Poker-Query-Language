use super::*;

pub struct PQLBoardRange(pub(crate) FnCheckRange, RangeSrc, PQLGame);

impl fmt::Debug for PQLBoardRange {
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PQLBoardRange")
            .field(&self.1)
            .field(&self.2)
            .finish()
    }
}

impl PQLBoardRange {
    #[inline]
    pub fn is_satisfied(&self, cs: &[PQLCard]) -> bool {
        (self.0)(cs)
    }
}

/// # Panics
/// `PQLBoardRange` ensures valid range text
impl Clone for PQLBoardRange {
    fn clone(&self) -> Self {
        (self.2, self.1.as_str()).try_into().unwrap()
    }
}

#[cfg(test)]
impl PQLBoardRange {
    /// AK != KA but good for assertions in tests
    pub fn src_eq(&self, other: &Self) -> bool {
        self.1 == other.1 && self.2 == other.2
    }
}

impl Default for PQLBoardRange {
    fn default() -> Self {
        Self(Box::new(|_| true), "*".into(), PQLGame::default())
    }
}

impl TryFrom<(PQLGame, &str)> for PQLBoardRange {
    type Error = PQLErrorKind;

    fn try_from((game, src): (PQLGame, &str)) -> Result<Self, Self::Error> {
        fn create_range<const SD: bool>(
            checker: BoardRangeChecker<SD>,
            src: &str,
            game: PQLGame,
        ) -> PQLBoardRange {
            PQLBoardRange(
                Box::new(move |cs: &[PQLCard]| checker.is_satisfied(cs)),
                src.to_string(),
                game,
            )
        }

        if game == PQLGame::ShortDeck {
            Ok(create_range(
                BoardRangeChecker::<true>::from_src(src)?,
                src,
                game,
            ))
        } else {
            Ok(create_range(
                BoardRangeChecker::<false>::from_src(src)?,
                src,
                game,
            ))
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_err() {
        for game in [PQLGame::Holdem, PQLGame::Omaha, PQLGame::ShortDeck] {
            let res = PQLBoardRange::try_from((game, "AAAAKK")).unwrap_err();

            assert_eq!(
                res,
                RangeError::TooManyCardsInRange((0, "AAAAKK".len())).into()
            );
        }
    }

    #[quickcheck]
    fn test_clone(cards: CardN<5, true>) {
        let res = PQLBoardRange::try_from((PQLGame::default(), "BB")).unwrap();
        let cloned = res.clone();

        assert_eq!(
            res.is_satisfied(cards.as_slice()),
            cloned.is_satisfied(cards.as_slice()),
        );

        let res = PQLBoardRange::try_from((PQLGame::ShortDeck, "BB")).unwrap();
        let cloned = res.clone();

        assert_eq!(
            res.is_satisfied(cards.as_slice()),
            cloned.is_satisfied(cards.as_slice()),
        );
    }
}
