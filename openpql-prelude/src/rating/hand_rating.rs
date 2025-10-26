use super::{
    HandRatingView, HandType, IdxThreeRanks, IdxTwoRanks, Rank16, RatingInner,
    fmt,
};

/// Hand Ranking
/// # Overview
/// this is used for holdem omaha and shortdeck
/// there are 10 hand types:
/// * `RoyalFlush`
/// * `StraightFlush`
/// * `Quads`
/// * `FullHouse`
/// * `Flush`
/// * `Straight`
/// * `Trips`
/// * `TwoPair`
/// * `Pair`
/// * `HighCard`
///
/// # Rank representation:
/// * Rank as index value
///   * 0b0000 is Deuce and 0b1100 is Ace
/// * Combination of two ranks
///   * nCr(13, 2) is 78 so the index of combination can be fitted in 7 bits
/// * Combination of three ranks
///   * nCr(13, 3) is 286 so the index of combination can be fitted in 9 bits
/// * Combination of five ranks
///   * we just use the 13 bit flags
///
/// # Memory Layout:
/// ```text
/// u16
///
/// `RoyalFlush`/`StraightFlush`
/// [15, 0]:   1110ssss 00000000 // s: rank of highest card
///
/// `Quads`:
/// [15, 0]:   11100000 qqqqkkkk // q: rank of quads; k: rank of kicker
///
/// `FullHouse`:
/// [15, 0]:   11011111 ttttpppp // q: rank of trips; k: rank of pair
/// [15, 0]:   10111111 ttttpppp // shortdeck
///
/// `Flush`:
/// [15, 0]:   101rrrrr rrrrrrrr // r: set bit of 5 cards and zeros of the rest
/// [15, 0]:   111rrrrr rrrrrrrr // shortdeck
///
/// `Straight`:
/// [15, 0]:   10000000 0000ssss // s: rank of highest card
///
/// `Trips`:
/// [15, 0]:   0110tttt 0kkkkkkk // t: rank of trips; k: index of combination
///
/// `TwoPair`:
/// [15, 0]:   01000ppp ppppkkkk // p: index of combination; k: rank of kicker
///
/// `Pair`:
/// [15, 0]:   001ppppk kkkkkkkk // p: rank of pair; k: index of combination
///
/// `HighCard`:
/// [15, 0]:   000rrrrr rrrrrrrr // r: bit flags of 5 cards
/// ```
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct HandRating(pub(crate) RatingInner);

/// returns highest rank index
/// input must have 1 or more ranks
#[allow(clippy::cast_possible_truncation)]
#[must_use]
#[inline]
const fn rank_idx(ranks: Rank16) -> RatingInner {
    TOTAL_LEADING_ZEROS - ranks.0.leading_zeros() as u16
}

#[inline]
const fn rev_rank_idx(idx: RatingInner) -> Rank16 {
    const MASK_RANK_IDX: RatingInner = 0b1111;
    Rank16(1 << (idx & MASK_RANK_IDX))
}

#[must_use]
#[inline]
const fn comb2(ranks: Rank16) -> RatingInner {
    IdxTwoRanks::from_r16(ranks).0 as RatingInner
}

#[inline]
fn rev_comb2(i: RatingInner) -> Rank16 {
    IdxTwoRanks(IdxTwoRanks::MASK_USED & i.to_le_bytes()[0]).to_r16()
}

#[must_use]
#[inline]
const fn comb3(ranks: Rank16) -> RatingInner {
    IdxThreeRanks::from_r16(ranks).0 as RatingInner
}

#[inline]
fn rev_comb3(i: RatingInner) -> Rank16 {
    IdxThreeRanks(IdxThreeRanks::MASK_USED & i).to_r16()
}

const TOTAL_LEADING_ZEROS: RatingInner = 15;
const OFFSET_RANK_IDX: usize = 4;
const OFFSET_COMB3: usize = 9;
const OFFSET_HI: usize = 8;
const MASK_FULLHOUSE_PADDING: RatingInner = 0b0001_1111_0000_0000;

impl HandRating {
    pub(crate) const MASK_STRAIGHTFLUSH: RatingInner = 0b1110_0000_0000_0000;
    pub(crate) const MASK_QUADS: RatingInner = 0b1110_0000_0000_0000;
    pub(crate) const MASK_FULLHOUSE: RatingInner = 0b1100_0000_0000_0000;
    pub(crate) const MASK_FLUSH: RatingInner = 0b1010_0000_0000_0000;
    pub(crate) const MASK_STRAIGHT: RatingInner = 0b1000_0000_0000_0000;
    pub(crate) const MASK_TRIPS: RatingInner = 0b0110_0000_0000_0000;
    pub(crate) const MASK_TWOPAIR: RatingInner = 0b0100_0000_0000_0000;
    pub(crate) const MASK_PAIR: RatingInner = 0b0010_0000_0000_0000;
    pub(crate) const MASK_HIGHCARD: RatingInner = 0b0000_0000_0000_0000;

    pub(crate) const MASK_FULLHOUSE_SD: RatingInner = Self::MASK_FLUSH;
    pub(crate) const MASK_FLUSH_SD: RatingInner = Self::MASK_FULLHOUSE;

    pub(crate) const fn new_highcard(ranks: Rank16) -> Self {
        Self(Self::MASK_HIGHCARD | ranks.0)
    }

    pub(crate) const fn parse_highcard(self) -> Rank16 {
        Rank16(Rank16::ALL.0 & self.0)
    }

    pub(crate) const fn new_pair(pair: Rank16, kicker: Rank16) -> Self {
        Self(Self::MASK_PAIR | rank_idx(pair) << OFFSET_COMB3 | comb3(kicker))
    }

    pub(crate) fn parse_pair(self) -> (Rank16, Rank16) {
        (
            rev_rank_idx((Self::MASK_PAIR ^ self.0) >> OFFSET_COMB3),
            rev_comb3(self.0),
        )
    }

    pub(crate) const fn new_twopair(pairs: Rank16, kicker: Rank16) -> Self {
        Self(
            Self::MASK_TWOPAIR
                | comb2(pairs) << OFFSET_RANK_IDX
                | rank_idx(kicker),
        )
    }

    pub(crate) fn parse_twopair(self) -> (Rank16, Rank16) {
        (rev_comb2(self.0 >> OFFSET_RANK_IDX), rev_rank_idx(self.0))
    }

    pub(crate) const fn new_trips(trips: Rank16, kicker: Rank16) -> Self {
        Self(Self::MASK_TRIPS | rank_idx(trips) << OFFSET_HI | comb2(kicker))
    }

    pub(crate) fn parse_trips(self) -> (Rank16, Rank16) {
        (rev_rank_idx(self.0 >> OFFSET_HI), rev_comb2(self.0))
    }

    pub(crate) const fn new_straight(ranks: Rank16) -> Self {
        Self(Self::MASK_STRAIGHT | rank_idx(ranks))
    }

    pub(crate) const fn parse_straight(self) -> Rank16 {
        rev_rank_idx(self.0)
    }

    pub(crate) const fn new_flush(ranks: Rank16) -> Self {
        Self(Self::MASK_FLUSH | ranks.0)
    }

    pub(crate) const fn new_flush_sd(ranks: Rank16) -> Self {
        Self(Self::MASK_FLUSH_SD | ranks.0)
    }

    pub(crate) const fn parse_flush(self) -> Rank16 {
        self.parse_highcard()
    }

    pub(crate) const fn new_fullhouse(trips: Rank16, pairs: Rank16) -> Self {
        Self(
            Self::MASK_FULLHOUSE
                | MASK_FULLHOUSE_PADDING
                | rank_idx(trips) << OFFSET_RANK_IDX
                | rank_idx(pairs),
        )
    }

    pub(crate) const fn new_fullhouse_sd(trips: Rank16, pairs: Rank16) -> Self {
        Self(
            Self::MASK_FULLHOUSE_SD
                | MASK_FULLHOUSE_PADDING
                | rank_idx(trips) << OFFSET_RANK_IDX
                | rank_idx(pairs),
        )
    }

    pub(crate) const fn parse_fullhouse(self) -> (Rank16, Rank16) {
        (
            rev_rank_idx(self.0 >> OFFSET_RANK_IDX),
            rev_rank_idx(self.0),
        )
    }

    pub(crate) const fn new_quad(quad: Rank16, kicker: Rank16) -> Self {
        Self(
            Self::MASK_QUADS
                | rank_idx(quad) << OFFSET_RANK_IDX
                | rank_idx(kicker),
        )
    }

    pub(crate) const fn parse_quad(self) -> (Rank16, Rank16) {
        self.parse_fullhouse()
    }

    pub(crate) const fn new_straightflush(ranks: Rank16) -> Self {
        Self(Self::MASK_STRAIGHTFLUSH | rank_idx(ranks) << OFFSET_HI)
    }

    pub(crate) const fn parse_straightflush(self) -> Rank16 {
        rev_rank_idx(self.0 >> OFFSET_HI)
    }
}

impl fmt::Display for HandRating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let view = HandRatingView::from(*self);
        let ht = view.hand_type;

        match ht {
            HandType::HighCard
            | HandType::Straight
            | HandType::Flush
            | HandType::StraightFlush => write!(f, "{ht}({})", view.high),

            HandType::Pair
            | HandType::TwoPair
            | HandType::Trips
            | HandType::FullHouse
            | HandType::Quads => write!(f, "{ht}({}, {})", view.high, view.low),
        }
    }
}

impl fmt::Debug for HandRating {
    #![cfg_attr(coverage_nightly, coverage(off))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use HandType::*;

    use super::*;
    use crate::*;

    fn assert_str(text: &str, ht: HandType, hi: &str, lo: &str) {
        assert_eq!(text, mk_rating(ht, hi, lo).to_string());
    }

    #[test]
    fn test_display() {
        assert_str("STRAIGHT_FLUSH(5)", StraightFlush, "5", "");
        assert_str("QUADS(A, K)", Quads, "A", "K");
        assert_str("FULL_HOUSE(T, A)", FullHouse, "T", "A");
        assert_str("FLUSH(6789J)", Flush, "J6789", "");
        assert_str("STRAIGHT(9)", Straight, "9", "");
        assert_str("TRIPS(T, KA)", Trips, "T", "AK");
        assert_str("TWO_PAIR(6T, K)", TwoPair, "T6", "K");
        assert_str("PAIR(J, 89K)", Pair, "J", "K98");
        assert_str("HIGH_CARD(89JQK)", HighCard, "KQJ98", "");
    }

    #[test]
    fn test_default() {
        assert_eq!(HandRating::default().0, RatingInner::MIN);
    }

    #[quickcheck]
    fn test_bijection(ranks: Distinct<5, Rank>) {
        let r5 = Rank16::from(&ranks[..]);

        let hi = Rank16::from(ranks[0]);
        let lo = Rank16::from(ranks[1]);
        let r2 = Rank16::from(&ranks[..2]);
        let r3 = Rank16::from(&ranks[2..]);
        let tl = Rank16::from(ranks[4]);

        assert_eq!(r5, HandRating::new_highcard(r5).parse_highcard());
        assert_eq!((hi, r3), HandRating::new_pair(hi, r3).parse_pair());
        assert_eq!((r2, tl), HandRating::new_twopair(r2, tl).parse_twopair());
        assert_eq!((tl, r2), HandRating::new_trips(tl, r2).parse_trips());
        assert_eq!(lo, HandRating::new_straight(lo).parse_straight());
        assert_eq!(r5, HandRating::new_flush(r5).parse_flush());
        assert_eq!(r5, HandRating::new_flush_sd(r5).parse_flush());
        assert_eq!(
            (hi, lo),
            HandRating::new_fullhouse(hi, lo).parse_fullhouse()
        );
        assert_eq!(
            (hi, lo),
            HandRating::new_fullhouse_sd(hi, lo).parse_fullhouse()
        );
        assert_eq!((hi, lo), HandRating::new_quad(hi, lo).parse_quad());
        assert_eq!(hi, HandRating::new_straightflush(hi).parse_straightflush());
    }
}
