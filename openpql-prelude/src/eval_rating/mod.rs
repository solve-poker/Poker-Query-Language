use super::{Card64, CardCount, HandRating, Rank16, Suit, transmute};

mod holdem;
mod omaha;
mod shortdeck;

pub use holdem::eval_holdem;
use holdem::{
    eval_pair, eval_quads, eval_trips, eval_twopair, mk_straight_ranking,
};
pub use omaha::eval_omaha;
pub use shortdeck::eval_shortdeck;

const N_FLUSH: u32 = 5;

impl Rank16 {
    const D: Self = Self(0b1000_0000_0000_0000);
    const R5: Self = Self(0b0000_0000_0000_1000);

    #[inline]
    const fn retain_highest(self) -> Self {
        debug_assert!(!self.is_empty());

        Self(Self::D.0 >> self.0.leading_zeros())
    }

    const fn diff(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    const fn or(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    #[inline]
    const fn retain_highest2(self) -> Self {
        let head = self.retain_highest();

        Self(head.0 | self.diff(head).retain_highest().0)
    }

    #[inline]
    const fn retain_highest3(self) -> Self {
        let head = self.retain_highest2();

        Self(head.0 | self.diff(head).retain_highest().0)
    }

    #[inline]
    const fn retain_highest4(self) -> Self {
        let head = self.retain_highest3();

        Self(head.0 | self.diff(head).retain_highest().0)
    }

    #[inline]
    const fn retain_highest5(self) -> Self {
        let head = self.retain_highest4();

        Self(head.0 | self.diff(head).retain_highest().0)
    }
}

#[inline]
const fn count_ranks(c: Card64) -> [Rank16; 4] {
    // transmute is faster than calling to_le_bytes 4 times.
    unsafe {
        let [s, h, d, c]: [u16; 4] = transmute(c);

        let has4 = s & h & d & c;
        let has3 = s & h & d | s & h & c | s & d & c | h & d & c;

        let has2 = s & h | s & d | s & c | h & d | h & c | d & c;
        let has1 = s | h | d | c;

        transmute([has1, has2, has3, has4])
    }
}

#[inline]
const fn flush_ranks(c: Card64) -> Option<Rank16> {
    unsafe {
        let [s, h, d, c]: [u16; 4] = transmute(c);

        if s.count_ones() >= N_FLUSH {
            Some(Rank16(s))
        } else if h.count_ones() >= N_FLUSH {
            Some(Rank16(h))
        } else if d.count_ones() >= N_FLUSH {
            Some(Rank16(d))
        } else if c.count_ones() >= N_FLUSH {
            Some(Rank16(c))
        } else {
            None
        }
    }
}

#[inline]
const fn flush_ranks_omaha_by_suit(
    player: Card64,
    board: Card64,
    suit: Suit,
) -> Option<(Rank16, Rank16)> {
    const N_FLUSH_PLAYER: CardCount = 2;
    const N_FLUSH_BOARD: CardCount = 3;

    let p = player.ranks_by_suit(suit);
    if p.count() >= N_FLUSH_PLAYER {
        let b = board.ranks_by_suit(suit);
        if b.count() >= N_FLUSH_BOARD {
            return Some((p, b));
        }
    }

    None
}

#[inline]
const fn flush_ranks_omaha(
    player: Card64,
    board: Card64,
) -> Option<(Rank16, Rank16)> {
    // const fn over DRY...

    if let Some((p, b)) = flush_ranks_omaha_by_suit(player, board, Suit::S) {
        return Some((p, b));
    }

    if let Some((p, b)) = flush_ranks_omaha_by_suit(player, board, Suit::H) {
        return Some((p, b));
    }

    if let Some((p, b)) = flush_ranks_omaha_by_suit(player, board, Suit::D) {
        return Some((p, b));
    }

    if let Some((p, b)) = flush_ranks_omaha_by_suit(player, board, Suit::C) {
        return Some((p, b));
    }

    None
}
