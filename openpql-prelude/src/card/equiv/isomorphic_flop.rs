//! Suit-isomorphic canonical form of a flop.

use crate::{Board, Card, Flop, IsomorphicCard, Suit, SuitMap};

/// The suit appearing twice in a two-tone flop.
type DoubleSuit = Suit;

/// Flush-relevant suit pattern of a flop's three cards.
#[derive(Clone, Copy)]
pub(super) enum FlopTexture {
    /// All three cards share one suit.
    Monotone(Suit),
    /// Two cards share a suit; carries the doubled suit then the lone suit.
    Twotone(DoubleSuit, Suit),
    /// All three suits differ.
    Rainbow(Suit, Suit, Suit),
}

impl FlopTexture {
    /// Classifies the texture of three rank-sorted flop cards.
    #[inline]
    pub(super) const fn from_sorted(f0: Card, f1: Card, f2: Card) -> Self {
        let (s0, s1, s2) = (f0.suit, f1.suit, f2.suit);

        match (s0.eq(s1), s0.eq(s2), s1.eq(s2)) {
            (true, true, _) => Self::Monotone(s0),
            (true, false, _) => Self::Twotone(s0, s2),
            (false, true, _) => Self::Twotone(s0, s1),
            (false, false, true) => Self::Twotone(s1, s0),
            (false, false, false) => Self::Rainbow(s0, s1, s2),
        }
    }

    /// Classifies the texture of a flop.
    #[inline]
    const fn from_flop(flop: Flop) -> Self {
        Self::from_sorted(flop.0[0], flop.0[1], flop.0[2])
    }

    /// Builds the [`SuitMap`] that relabels this texture's suits canonically.
    #[inline]
    const fn to_suit_map(self) -> SuitMap {
        match self {
            Self::Monotone(s) => SuitMap::map1(s),
            Self::Twotone(dbl, s) => SuitMap::map2(dbl, s),
            Self::Rainbow(s0, s1, s2) => SuitMap::map3(s0, s1, s2),
        }
    }
}

/// Canonical suit-isomorphic representative of a flop.
#[derive(Clone, Copy, derive_more::Debug, PartialEq, Eq, Hash)]
#[debug("IsomorphicFlop({_0:?})")]
pub struct IsomorphicFlop(pub(crate) [IsomorphicCard; Board::N_FLOP]);

impl IsomorphicFlop {
    /// Builds the canonical representative of `flop`.
    pub const fn from_flop(flop: Flop) -> Self {
        let map = FlopTexture::from_flop(flop).to_suit_map();

        Self::from_3_cards(
            map.iso_card(flop.0[0]),
            map.iso_card(flop.0[1]),
            map.iso_card(flop.0[2]),
        )
    }

    /// Builds an `IsomorphicFlop` from three relabeled cards, sorting them.
    pub(crate) const fn from_3_cards(
        c0: IsomorphicCard,
        c1: IsomorphicCard,
        c2: IsomorphicCard,
    ) -> Self {
        Self(match (c0.lt(c1), c1.lt(c2), c0.lt(c2)) {
            (true, true, _) => [c0, c1, c2],
            (true, _, true) => [c0, c2, c1],
            (true, _, false) => [c2, c0, c1],
            (false, true, true) => [c1, c0, c2],
            (false, true, _) => [c1, c2, c0],
            (false, _, _) => [c2, c1, c0],
        })
    }
}

impl Flop {
    /// Returns the canonical suit-isomorphic representative of this flop.
    #[must_use]
    pub const fn to_isomorphic(self) -> IsomorphicFlop {
        IsomorphicFlop::from_flop(self)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::*;

    fn iso_flop(s: &str) -> IsomorphicFlop {
        let cs = isocards!(s);

        IsomorphicFlop([cs[0], cs[1], cs[2]])
    }

    fn assert_iso_flop(s: &str) {
        let (lhs, rhs) = s.split_once("->").unwrap();
        assert_eq!(
            flop!(lhs).to_isomorphic(),
            iso_flop(rhs),
            "{:?}->{rhs}; but got {:?}",
            flop!(lhs),
            flop!(lhs).to_isomorphic()
        );
    }

    pub fn gen_iso_flop() -> FxHashSet<IsomorphicFlop> {
        Flop::iter_all::<true>().map(Flop::to_isomorphic).collect()
    }

    #[test]
    fn test_iso_flop() {
        assert_iso_flop("6s6hAs -> 6x6yAx");
        assert_iso_flop("6s6hAh -> 6x6yAx");

        assert_eq!(gen_iso_flop().len(), 573);
    }

    #[test]
    fn test_trips_rainbow() {
        assert_iso_flop("AsAhAd -> AxAyAz");
        assert_iso_flop("AsAhAc -> AxAyAz");
        assert_iso_flop("AsAdAc -> AxAyAz");
        assert_iso_flop("AhAdAc -> AxAyAz");
    }

    #[test]
    fn test_paired_twotone() {
        assert_iso_flop("AsAhKh -> KxAxAy");
        assert_iso_flop("AhAsKs -> KxAxAy");
        assert_iso_flop("ThTdKd -> TxTyKx");
        assert_iso_flop("TcTsKc -> TxTyKx");

        assert_iso_flop("6s6hAs -> 6x6yAx");
        assert_iso_flop("6s6hAh -> 6x6yAx");
    }

    #[test]
    fn test_paired_rainbow() {
        assert_iso_flop("AsAhKd -> KxAyAz");
        assert_iso_flop("ThTcKd -> TxTyKz");
    }

    #[test]
    fn test_monotone() {
        assert_iso_flop("AsKsQs -> QxKxAx");
        assert_iso_flop("AhKhQh -> QxKxAx");
    }
}
