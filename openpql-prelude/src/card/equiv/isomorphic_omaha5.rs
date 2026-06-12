//! Suit-isomorphic canonical form of a complete five-card board.

use crate::{
    Card, Card64, CardCount, IsomorphicCard, IsomorphicHandN, Rank16, Suit,
    card::{equiv::isomorphic_turn::TurnTexture, util::sort5},
};

const N_OMAHA5: usize = 5;

pub(super) type Hand5Texture = TurnTexture;

pub(super) const fn iso_hand5_preflop(cards: &[Card]) -> IsomorphicHandN<N_OMAHA5> {
    let map = hand5_texture(cards).to_suit_map();
    let inner = sort5![
        IsomorphicCard,
        map.iso_card(cards[0]),
        map.iso_card(cards[1]),
        map.iso_card(cards[2]),
        map.iso_card(cards[3]),
        map.iso_card(cards[4])
    ];

    IsomorphicHandN(inner)
}

#[inline]
const fn hand5_texture(cards: &[Card]) -> Hand5Texture {
    let mut c64 = Card64::EMPTY;
    c64.set(cards[0]);
    c64.set(cards[1]);
    c64.set(cards[2]);
    c64.set(cards[3]);
    c64.set(cards[4]);

    match compact(flush_ranks(c64)) {
        (Some(x), Some(y)) => {
            if x.0.const_lt(y.0) {
                Hand5Texture::DblFlushDraw(x.1, y.1)
            } else {
                Hand5Texture::DblFlushDraw(y.1, x.1)
            }
        }
        (Some(x), None) => Hand5Texture::FlushDraw(x.1),
        _ => unreachable!(), // LCOV_EXCL_LINE
    }
}

type RanksSuit = (Rank16, Suit);
type FlushInfo = [Option<RanksSuit>; Suit::N_SUITS as usize];

#[inline]
const fn flush_ranks(c64: Card64) -> FlushInfo {
    [
        flush_ranks_by_suit(c64, Suit::S),
        flush_ranks_by_suit(c64, Suit::H),
        flush_ranks_by_suit(c64, Suit::D),
        flush_ranks_by_suit(c64, Suit::C),
    ]
}

#[inline]
const fn flush_ranks_by_suit(c64: Card64, suit: Suit) -> Option<RanksSuit> {
    const N: CardCount = 2;
    let ranks = c64.ranks_by_suit(suit);

    if ranks.count() >= N {
        Some((ranks, suit))
    } else {
        None
    }
}

#[inline]
const fn compact(options: FlushInfo) -> (Option<RanksSuit>, Option<RanksSuit>) {
    match options {
        [Some(x), Some(y), _, _]
        | [Some(x), None, Some(y), _]
        | [Some(x), None, None, Some(y)]
        | [None, Some(x), Some(y), _]
        | [None, Some(x), None, Some(y)]
        | [None, None, Some(x), Some(y)] => (Some(x), Some(y)),

        [Some(x), None, None, None]
        | [None, Some(x), None, None]
        | [None, None, Some(x), None]
        | [None, None, None, Some(x)] => (Some(x), None),

        [None, None, None, None] => unreachable!(), // LCOV_EXCL_LINE
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    fn swap_suit(c: Card, a: Suit, b: Suit) -> Card {
        let suit = if c.suit == a {
            b
        } else if c.suit == b {
            a
        } else {
            c.suit
        };
        Card::new(c.rank, suit)
    }

    #[quickcheck]
    fn test_iso_hand5_preflop_suit_permutation_invariant(cs: CardN<5>, a: Suit, b: Suit) {
        let swapped: Vec<Card> = cs.as_slice().iter().map(|&c| swap_suit(c, a, b)).collect();
        assert_eq!(
            iso_hand5_preflop(cs.as_slice()),
            iso_hand5_preflop(&swapped),
            "{cs:?} vs {swapped:?}"
        );
    }

    #[test]
    fn test_iso_hand5_preflop_distinguishes_flush_shapes() {
        let double_suited = cards!("As Ks Qh Jh 2d");
        let single_suited = cards!("As Ks Qh Jd 2c");
        let rainbow = cards!("As Kh Qd Jc 2s");

        assert_ne!(
            iso_hand5_preflop(&double_suited),
            iso_hand5_preflop(&single_suited)
        );
        assert_ne!(
            iso_hand5_preflop(&single_suited),
            iso_hand5_preflop(&rainbow)
        );
    }
}
