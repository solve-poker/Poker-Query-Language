use super::*;

pub struct PQLRange(pub(crate) FnCheckRange, RangeSrc, PQLGame);

impl fmt::Debug for PQLRange {
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PQLRange")
            .field(&self.1)
            .field(&self.2)
            .finish()
    }
}

impl PQLRange {
    #[inline]
    pub fn is_satisfied(&self, cs: &[PQLCard]) -> bool {
        (self.0)(cs)
    }
}

/// # Panics
/// `PQLRange` ensures valid range text
impl Clone for PQLRange {
    fn clone(&self) -> Self {
        (self.2, self.1.as_str()).try_into().unwrap()
    }
}

#[cfg(test)]
impl PQLRange {
    /// AK != KA but good for assertions in tests
    pub fn src_eq(&self, other: &Self) -> bool {
        self.1 == other.1 && self.2 == other.2
    }
}

impl TryFrom<(PQLGame, &str)> for PQLRange {
    type Error = PQLErrorKind;

    fn try_from((game, src): (PQLGame, &str)) -> Result<Self, Self::Error> {
        fn create_range<const N: usize, const SD: bool>(
            checker: RangeChecker<N, SD>,
            src: &str,
            game: PQLGame,
        ) -> PQLRange
        where
            [u8; N]: smallvec::Array<Item = u8>,
        {
            PQLRange(
                Box::new(move |cs: &[PQLCard]| checker.is_satisfied(cs)),
                src.to_string(),
                game,
            )
        }

        match game {
            PQLGame::Holdem => Ok(create_range(
                RangeChecker::<2, false>::from_src(src)?,
                src,
                game,
            )),
            PQLGame::Omaha => Ok(create_range(
                RangeChecker::<4, false>::from_src(src)?,
                src,
                game,
            )),
            PQLGame::ShortDeck => Ok(create_range(
                RangeChecker::<2, true>::from_src(src)?,
                src,
                game,
            )),
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
            let res = PQLRange::try_from((game, "AAAAK")).unwrap_err();

            assert_eq!(
                res,
                RangeError::TooManyCardsInRange((0, "AAAAK".len())).into()
            );
        }
    }

    #[quickcheck]
    fn test_clone(cards: CardN<2, true>) {
        let res = PQLRange::try_from((PQLGame::default(), "BB")).unwrap();
        let cloned = res.clone();

        assert_eq!(
            res.is_satisfied(cards.as_slice()),
            cloned.is_satisfied(cards.as_slice()),
        );
    }
}
