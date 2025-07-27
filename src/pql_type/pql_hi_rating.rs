use super::*;

/// Hand Ranking
/// # Overview
/// this is used for holdem omaha and shortdeck
/// there are 10 hand types:
/// * `RoyalFlush`
/// * `StraigtFlush`
/// * `Quads`
/// * `FullHouse`
/// * `Flush`
/// * `Straight`
/// * `Trips`
/// * `TwoPair`
/// * `Pair`
/// * `HighCard`
///
/// with some variations of shortdeck the order of handtype can vary:
/// * FFH
///   * Flush > `FullHouse` > Straight > Trips
/// * TS
///   * Flush > `FullHouse` > Trips > Straight
///
/// # Rank representation:
/// * Rank as index value
///   * 0b0000 is Duece and 0b1100 is Ace
/// * Combination of two ranks
///   * nCr(13, 2) is 78 so the index of combination can be fitted in 7 bits
/// * Combination of three ranks
///   * nCr(13, 3) is 286 so the index of combination can be fitted in 9 bits
/// * Combination of five ranks
///   * we just use the 13 bit flags
///
/// # Memory Layout:
/// ```text
/// positive i16 values:
///
/// `RoyalFlush`/`StraightFlush`
/// [15, 0]:   011ssss0 00000000 // s: rank of highest card
///
/// `Quads`:
/// [15, 0]:   01100000 qqqqkkkk // q: rank of quads; k: rank of kicker
///
/// `FullHouse`:
/// [15, 0]:   01000000 ttttpppp // q: rank of trips; k: rank of pair
///
/// `Flush`:
/// [15, 0]:   001rrrrr rrrrrrrr // r: set bit of 5 cards and zeros of the rest
///
/// `Straight`:
/// [15, 0]:   000ssss0 00000000 // s: rank of highest card
///
/// negative i16 values:
///
/// `Trips`:
/// [15, 0]:   1110tttt 0kkkkkkk // t: rank of trips; k: index of combination
///
/// `TwoPair`:
/// [15, 0]:   11000ppp ppppkkkk // p: index of combination; k: rank of kicker
///
/// `Pair`:
/// [15, 0]:   101ppppk kkkkkkkk // p: rank of pair; k: index of combination
///
/// `HighCard`:
/// [15, 0]:   100rrrrr rrrrrrrr // r: bit flags of 5 cards
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PQLHiRating(HandRatingInt);

impl PQLHiRating {
    // TODO: change it to RoyalFlush
    pub const MAX: Self = Self(HandRatingInt::MAX);
    pub const MIN: Self = Self(HandRatingInt::MIN);

    pub const fn new(i: HandRatingInt) -> Self {
        Self(i)
    }

    pub const fn to_i16(self) -> i16 {
        self.0
    }

    pub fn to_hand_type(self, game: PQLGame) -> PQLHandType {
        let ht = HandTypeOrd::from(game).i16_to_hand_type(self.0);
        (ht, game).into()
    }

    pub fn to_hand_type_and_low_high_ranks(
        self,
        game: PQLGame,
    ) -> (PQLHandType, PQLRankSet, PQLRankSet) {
        let ht = self.to_hand_type(game);
        let layout = ht.ht.to_layout();

        let bytes = self.0.to_le_bytes();
        let (l, h) = layout.masks_to_ranks(bytes[0], bytes[1]);

        (ht, PQLRankSet::from_u16(l), PQLRankSet::from_u16(h))
    }
}

impl Default for PQLHiRating {
    fn default() -> Self {
        Self::MIN
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Arbitrary for PQLHiRating {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            // TODO: change to real rating
            Self(HandRatingInt::arbitrary(g))
        }
    }

    #[test]
    fn test_default() {
        assert_eq!(
            PQLHiRating::default(),
            PQLHiRating::new(HandRatingInt::MIN)
        );
    }
}
