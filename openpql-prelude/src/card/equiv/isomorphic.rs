//! Suit-isomorphic canonicalisation for 3/4/5-card boards.
//!
//! - Monotone flop (Case A) collapses *all* non-flop suits to a single
//!   canonical suit. Distinct turn/river suits with no flop overlap end
//!   up with the same canonical label.
//! - Position-aware: the turn and river slots are not interchangeable.
//!   `cards!("6s8h9dQdQc")` and `cards!("6s8h9dQcQd")` produce **different**
//!   canonical 5-card boards.

use crate::{Board, Card, HandN, Rank, Suit, SuitMapping};

impl<const N: usize> HandN<N> {
    /// Creates a suit-isomorphic hand, populating `mapping` with the
    /// implied forward suit substitution.
    ///
    /// `N` must be in `3..=5` and `mapping` must be empty.
    ///
    /// # Panics
    /// Panics if `cards.len() < N`.
    pub fn new_iso_with_mapping(
        cards: &[Card],
        mapping: &mut SuitMapping,
    ) -> Self {
        debug_assert!(
            (3..=5).contains(&N),
            "HandN::new_iso_with_mapping supports N in 3..=5",
        );
        debug_assert!(mapping.is_empty(), "mapping must be empty");
        let mut arr: [Card; N] = cards[..N].try_into().unwrap();
        optimize_suits(&mut arr, mapping);
        Self(arr)
    }

    /// Creates a suit-isomorphic hand along with its suit mapping.
    #[must_use]
    pub fn new_iso(cards: &[Card]) -> (Self, SuitMapping) {
        let mut mapping = SuitMapping::default();
        let iso = Self::new_iso_with_mapping(cards, &mut mapping);
        (iso, mapping)
    }
}

impl Board {
    /// Creates a suit-isomorphic board, populating `mapping` with the
    /// implied forward suit substitution.
    ///
    /// `cards.len()` must be 0 (preflop) or in `3..=5`. `mapping` must
    /// be empty.
    pub fn new_iso_with_mapping(
        cards: &[Card],
        mapping: &mut SuitMapping,
    ) -> Self {
        debug_assert!(mapping.is_empty(), "mapping must be empty");
        let n = cards.len();
        let mut board = Self::default();
        if n < Self::N_FLOP {
            return board;
        }
        debug_assert!(n <= Self::N_RIVER, "Board takes at most 5 cards");

        let mut buf = [Card::default(); Self::N_RIVER];
        buf[..n].copy_from_slice(cards);
        optimize_suits(&mut buf[..n], mapping);
        board.flop = Some(HandN::<3>([buf[0], buf[1], buf[2]]));
        if n >= Self::N_TURN {
            board.turn = Some(buf[3]);
        }
        if n >= Self::N_RIVER {
            board.river = Some(buf[4]);
        }
        board
    }

    /// Creates a suit-isomorphic board along with its suit mapping.
    #[must_use]
    pub fn new_iso(cards: &[Card]) -> (Self, SuitMapping) {
        let mut mapping = SuitMapping::default();
        let iso = Self::new_iso_with_mapping(cards, &mut mapping);
        (iso, mapping)
    }
}

fn optimize_suits(cards: &mut [Card], mapping: &mut SuitMapping) {
    debug_assert!(
        (3..=5).contains(&cards.len()),
        "optimize_suits expects 3..=5 cards",
    );

    // Step 1: sort the flop (indices 0..3) ascending by the indexed
    // key (rank-major, suit-minor; ace-low). The turn (idx 3) and
    // river (idx 4) stay in place.
    sort_first_three(&mut cards[..3]);

    let s0 = cards[0].suit;
    let s1 = cards[1].suit;
    let s2 = cards[2].suit;

    // Step 2: case-dispatch on the (sorted) flop suit structure.
    if s0 == s1 && s1 == s2 {
        // Case A: monotone.
        rewrite_case_a(cards, mapping, s0);
    } else if s0 == s1 {
        // Case B: paired lower-two.
        rewrite_case_bcd(cards, mapping, s0, s2, [Suit::H, Suit::H, Suit::D]);
    } else if s0 == s2 {
        // Case C: paired outer.
        rewrite_case_bcd(cards, mapping, s0, s1, [Suit::H, Suit::D, Suit::H]);
    } else if s1 == s2 {
        // Case D: paired upper-two.
        rewrite_case_bcd(cards, mapping, s1, s0, [Suit::D, Suit::H, Suit::H]);
    } else {
        // Case E: rainbow flop.
        rewrite_case_e(cards, mapping, [s0, s1, s2]);
    }

    // Step 3: re-sort the flop with the new suit codes.
    sort_first_three(&mut cards[..3]);
}

/// Case A — monotone flop. Flop collapses to C; turn/river map to C if
/// they match the flop suit, else collapse to H. Turn and river are not
/// distinguished here — two different non-flop suits both map to H, so
/// the mapping is not faithful in this case.
fn rewrite_case_a(
    cards: &mut [Card],
    mapping: &mut SuitMapping,
    flop_suit: Suit,
) {
    mapping.assign_pair(flop_suit, Suit::C);
    cards[0].suit = Suit::C;
    cards[1].suit = Suit::C;
    cards[2].suit = Suit::C;
    for c in &mut cards[3..] {
        let s = c.suit;
        let new = if s == flop_suit { Suit::C } else { Suit::H };
        c.suit = new;
        mapping.assign_pair(s, new);
    }
}

/// Shared rewrite for cases B/C/D: paired flop with one off-suit card.
/// `paired` is the flop suit that appears twice, `single` is the one
/// that appears once. `flop_new` gives the new suit code for each of
/// `cards[0..3]` in position order.
fn rewrite_case_bcd(
    cards: &mut [Card],
    mapping: &mut SuitMapping,
    paired: Suit,
    single: Suit,
    flop_new: [Suit; 3],
) {
    mapping.assign_pair(paired, Suit::H);
    mapping.assign_pair(single, Suit::D);
    cards[0].suit = flop_new[0];
    cards[1].suit = flop_new[1];
    cards[2].suit = flop_new[2];
    if cards.len() > 3 {
        let st = cards[3].suit;
        let new_t = if st == paired {
            Suit::H
        } else if st == single {
            Suit::D
        } else {
            Suit::C
        };
        cards[3].suit = new_t;
        mapping.assign_pair(st, new_t);
        if cards.len() > 4 {
            let sr = cards[4].suit;
            let new_r = if sr == paired {
                Suit::H
            } else if sr == st {
                new_t
            } else {
                Suit::C
            };
            cards[4].suit = new_r;
            mapping.assign_pair(sr, new_r);
        }
    }
}

/// Case E — rainbow flop. Flop suits (in sorted order) → H, C, S. Turn
/// matches against `s` → H/C/S, else D. River matches the *original*
/// turn suit → turn's new suit, else D.
fn rewrite_case_e(cards: &mut [Card], mapping: &mut SuitMapping, s: [Suit; 3]) {
    mapping.assign_pair(s[0], Suit::H);
    mapping.assign_pair(s[1], Suit::C);
    mapping.assign_pair(s[2], Suit::S);
    cards[0].suit = Suit::H;
    cards[1].suit = Suit::C;
    cards[2].suit = Suit::S;
    if cards.len() > 3 {
        let st = cards[3].suit;
        let new_t = if st == s[0] {
            Suit::H
        } else if st == s[1] {
            Suit::C
        } else if st == s[2] {
            Suit::S
        } else {
            Suit::D
        };
        cards[3].suit = new_t;
        mapping.assign_pair(st, new_t);
        if cards.len() > 4 {
            let sr = cards[4].suit;
            let new_r = if sr == st { new_t } else { Suit::D };
            cards[4].suit = new_r;
            mapping.assign_pair(sr, new_r);
        }
    }
}

/// In-place sort of `cards[0..3]` ascending by the indexed key
/// (`rank * 6 + suit`, with `rank` 1-based and ace-low: A=1, K=13).
/// Sorting key respects rank-major, suit-minor.
fn sort_first_three(cards: &mut [Card]) {
    let key = |c: Card| -> u32 {
        u32::from(ace_low_rank(c.rank)) * 6 + c.suit as u32
    };
    // 3-element sorting network: cs(1,2), cs(0,2), cs(0,1).
    if key(cards[2]) < key(cards[1]) {
        cards.swap(1, 2);
    }
    if key(cards[2]) < key(cards[0]) {
        cards.swap(0, 2);
    }
    if key(cards[1]) < key(cards[0]) {
        cards.swap(0, 1);
    }
}

/// 1-based ace-low rank: A=1, 2=2, …, K=13.
#[inline]
const fn ace_low_rank(rank: Rank) -> u8 {
    match rank {
        Rank::RA => 1,
        Rank::R2 => 2,
        Rank::R3 => 3,
        Rank::R4 => 4,
        Rank::R5 => 5,
        Rank::R6 => 6,
        Rank::R7 => 7,
        Rank::R8 => 8,
        Rank::R9 => 9,
        Rank::RT => 10,
        Rank::RJ => 11,
        Rank::RQ => 12,
        Rank::RK => 13,
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {

    use super::*;
    use crate::*;

    fn canonicalise<const N: usize>(input: &[Card]) -> [Card; N] {
        let mut arr: [Card; N] = input.try_into().unwrap();
        let mut mapping = SuitMapping::default();
        optimize_suits(&mut arr, &mut mapping);
        arr
    }

    #[test]
    fn monotone_flop_with_two_distinct_off_suits_collapse() {
        // Case A: monotone-spades flop with distinct turn/river off-suits.
        // Case A collapses all non-flop suits to H, so both inputs match.
        let out1 = canonicalise::<5>(&cards!("AsKsQs2h3d"));
        let out2 = canonicalise::<5>(&cards!("AsKsQs2d3h"));
        assert_eq!(out1, out2);
    }

    #[test]
    fn turn_river_position_matters() {
        // The algorithm is position-aware on turn vs river. Two boards
        // differing only by a turn<->river suit swap (ranks match)
        // canonicalise differently.
        let a = canonicalise::<5>(&cards!("6s8h9dQdQc"));
        let b = canonicalise::<5>(&cards!("6s8h9dQcQd"));
        assert_ne!(a, b);
    }

    #[test]
    fn test_iso_position_aware() {
        // Same property surfaced via the public `HandN::new_iso` entry.
        assert_ne!(
            HandN::<5>::new_iso(&cards!("6s8h9dQdQc")).0,
            HandN::<5>::new_iso(&cards!("6s8h9dQcQd")).0
        );
    }

    #[test]
    fn test_iso_flop() {
        let mut res = FxHashMap::default();
        let mut iso_set = FxHashSet::default();

        for hand in Flop::iter_all::<true>() {
            let (iso, _) = HandN::<3>::new_iso(&hand.0);
            res.insert(hand, iso);
            iso_set.insert(iso);
        }

        assert_eq!(res.len(), 7140);
        assert_eq!(iso_set.len(), 573);
    }

    #[test]
    fn test_iso_board() {
        fn assert_iso_eq(lhs: &str, rhs: &str) {
            assert_eq!(
                Board::new_iso(&cards!(lhs)).0,
                Board::new_iso(&cards!(rhs)).0
            );
        }

        assert_iso_eq("", "");
        assert_iso_eq("AsKhQd", "AhKsQc");
        assert_iso_eq("AsKhQdJdTd", "AhKsQcJcTc"); // TODO: maybe ignore turn/river order?
    }
}
