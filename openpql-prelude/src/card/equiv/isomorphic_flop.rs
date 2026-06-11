//! Suit-isomorphic canonical form of a flop.

use crate::{Board, Card, Flop, IsomorphicCard, Suit, SuitMap, card::util::sort3};

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

        match (s0.const_eq(s1), s0.const_eq(s2), s1.const_eq(s2)) {
            (true, true, _) => Self::Monotone(s0),
            (true, false, _) => Self::Twotone(s0, s2),
            (false, true, _) => Self::Twotone(s0, s1),
            (false, false, true) => Self::Twotone(s1, s0),
            (false, false, false) => Self::Rainbow(s0, s1, s2),
        }
    }

    /// Builds the [`SuitMap`] that relabels this texture's suits canonically.
    #[inline]
    pub(super) const fn to_suit_map(self) -> SuitMap {
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
pub(super) struct IsomorphicFlop(pub(super) [IsomorphicCard; Board::N_FLOP]);

impl IsomorphicFlop {
    /// Canonical representative of `flop` and the [`SuitMap`] that produced it.
    #[inline]
    pub(super) const fn from_flop(flop: Flop) -> (Self, SuitMap) {
        let [f0, f1, f2] = flop.0;
        let map = FlopTexture::from_sorted(f0, f1, f2).to_suit_map();

        (Self::relabel(f0, f1, f2, map), map)
    }

    /// Relabels three flop cards through `map`, then rank-sorts them.
    #[inline]
    pub(super) const fn relabel(f0: Card, f1: Card, f2: Card, map: SuitMap) -> Self {
        Self::sorted(map.iso_card(f0), map.iso_card(f1), map.iso_card(f2))
    }

    /// Sorts three rank-sorted relabeled cards, breaking rank ties by suit label.
    const fn sorted(a: IsomorphicCard, b: IsomorphicCard, c: IsomorphicCard) -> Self {
        Self(sort3!(IsomorphicCard, a, b, c))
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    fn iso_flop(s: &str) -> IsomorphicFlop {
        let cs = isocards!(s);

        IsomorphicFlop([cs[0], cs[1], cs[2]])
    }

    fn assert_iso_flop(s: &str) {
        let (lhs, rhs) = s.split_once("->").unwrap();
        let got = IsomorphicFlop::from_flop(flop!(lhs)).0;

        assert_eq!(
            got,
            iso_flop(rhs),
            "{:?}->{rhs}; but got {got:?}",
            flop!(lhs)
        );
    }

    fn gen_iso_flop() -> FxHashSet<IsomorphicFlop> {
        Flop::iter_all::<true>()
            .map(|flop| IsomorphicFlop::from_flop(flop).0)
            .collect()
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
