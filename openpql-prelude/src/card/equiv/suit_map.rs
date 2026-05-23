//! Suit permutations that relabel a board to its isomorphic form.

use FlushingSuit::{N, X, Y, Z};

use crate::{Card, FlushingSuit, IsomorphicCard, Suit};

/// Permutation of the four [`Suit`]s used to map a board to its isomorphic form.
#[derive(Clone, Copy, Debug, Default)]
pub struct SuitMap(pub [FlushingSuit; 4]);

impl SuitMap {
    /// Returns the identity map, leaving every suit irrelevant ([`N`]).
    #[inline]
    pub(super) const fn new() -> Self {
        Self([N, N, N, N])
    }

    /// Returns the canonical suit that `suit` is mapped to.
    #[inline]
    pub const fn iso(&self, suit: Suit) -> FlushingSuit {
        self.0[suit as usize]
    }

    /// Relabels `card`'s suit through this map, yielding an [`IsomorphicCard`].
    #[inline]
    pub const fn iso_card(&self, card: Card) -> IsomorphicCard {
        IsomorphicCard {
            rank: card.rank,
            suit: self.iso(card.suit),
        }
    }

    /// Builds a map with no flush-relevant suits.
    #[inline]
    pub(crate) const fn map0() -> Self {
        Self::new()
    }

    /// Builds a map relabeling `s0` to [`X`], the one flush-relevant suit.
    #[inline]
    pub(crate) const fn map1(s0: Suit) -> Self {
        let mut res = Self::new();

        res.0[s0 as usize] = X;

        res
    }

    /// Builds a map relabeling `s0` to [`X`] and `s1` to [`Y`].
    #[inline]
    pub(crate) const fn map2(s0: Suit, s1: Suit) -> Self {
        let mut res = Self::new();

        res.0[s0 as usize] = X;
        res.0[s1 as usize] = Y;

        res
    }

    /// Builds a map relabeling `s0` to [`X`], `s1` to [`Y`], and `s2` to [`Z`].
    #[inline]
    pub(crate) const fn map3(s0: Suit, s1: Suit, s2: Suit) -> Self {
        let mut res = Self::new();

        res.0[s0 as usize] = X;
        res.0[s1 as usize] = Y;
        res.0[s2 as usize] = Z;

        res
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::Suit::*;

    #[test]
    fn test_default() {
        assert_eq!(SuitMap::default().0, [N, N, N, N]);
    }

    #[test]
    fn test_map1() {
        assert_eq!(SuitMap::map1(D).0, [N, N, X, N]);
    }

    #[test]
    fn test_map2() {
        assert_eq!(SuitMap::map2(H, C).0, [N, X, N, Y]);
    }

    #[test]
    fn test_map3() {
        assert_eq!(SuitMap::map3(C, D, H).0, [N, Z, Y, X]);
    }

    #[test]
    fn test_const_context() {
        const M: SuitMap = SuitMap::map3(C, D, H);
        assert_eq!(M.0, [N, Z, Y, X]);
    }
}
