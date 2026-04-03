use rand::RngExt;

use crate::{Card, Card64, RankIdx, SuitIdx, card::Idx};

#[derive(Clone, Debug, Default)]
pub struct CardGen {
    init: Card64,
    used: Card64,
    unused: Card64,
}

impl CardGen {
    pub fn new<const SD: bool>(dead_cards: Card64) -> Self {
        let all = Card64::all::<SD>() & !(dead_cards);

        Self {
            init: all,
            used: Card64::default(),
            unused: all,
        }
    }

    /// # Panics
    /// no panics since `Card64::all::<SD>()` guarantees valid values
    #[allow(clippy::cast_possible_truncation)]
    pub fn deal(&mut self, rng: &mut impl rand::Rng) -> Option<Card> {
        if let Some(bit_idx) = random_set_bit_pos_64(self.unused.into(), rng) {
            let bit_idx = bit_idx as Idx;

            let suit_idx = bit_idx / Card64::OFFSET_SUIT;
            let rank_idx = bit_idx % Card64::OFFSET_SUIT;
            let card = Card::new(
                RankIdx(rank_idx).to_rank().unwrap(),
                SuitIdx(suit_idx).to_suit().unwrap(),
            );

            self.unused.unset(card);
            self.used.set(card);

            Some(card)
        } else {
            None
        }
    }

    pub fn unset(&mut self, c64: Card64) {
        for c in c64.iter() {
            self.unset_card(c);
        }
    }

    pub fn unset_card(&mut self, card: Card) {
        debug_assert!(!self.unused.contains_card(card));

        self.used.unset(card);
        self.unused.set(card);
    }

    pub const fn reset(&mut self) {
        self.unused = self.init;
    }
}

fn random_set_bit_pos_64(mask: u64, rng: &mut impl rand::Rng) -> Option<u32> {
    let n = mask.count_ones();
    if n == 0 {
        return None;
    }

    let idx = rng.random_range(0..n);
    let mut remaining = mask;
    let mut pos = 0;

    for _ in 0..=idx {
        let tz = remaining.trailing_zeros();
        remaining &= remaining - 1;
        pos = tz;
    }
    Some(pos)
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
pub mod tests {
    use super::*;

    fn mk_exhausted_card_gen() -> CardGen {
        let mut g = CardGen::new::<false>(Card64::default());
        (g.used, g.unused) = (g.unused, g.used);

        g
    }

    fn deal_all_to_c64(rng: &mut impl rand::Rng, g: &mut CardGen) -> Card64 {
        (0..Card::N_CARDS).filter_map(|_| g.deal(rng)).collect()
    }

    #[test]
    fn test_deal_all() {
        const SD: bool = false;
        let mut rng = rand::rng();
        let mut g = CardGen::new::<SD>(Card64::default());
        let mut dealt = Card64::default();

        for _ in 0..Card::N_CARDS {
            dealt.set(g.deal(&mut rng).unwrap());
        }

        assert_eq!(dealt, Card64::all::<SD>());
    }

    #[test]
    fn test_deal_all_sd() {
        const SD: bool = true;
        let mut rng = rand::rng();
        let mut g = CardGen::new::<SD>(Card64::default());
        let mut dealt = Card64::default();

        for _ in 0..Card::N_CARDS_SD {
            dealt.set(g.deal(&mut rng).unwrap());
        }

        assert_eq!(dealt, Card64::all::<SD>());
    }

    #[quickcheck]
    fn test_unset(cards: Card64) {
        let mut rng = rand::rng();

        let mut g = mk_exhausted_card_gen();

        g.unset(cards);

        assert_eq!(deal_all_to_c64(&mut rng, &mut g), cards);

        let mut g = mk_exhausted_card_gen();

        for card in cards.iter() {
            g.unset_card(card);
        }

        assert_eq!(deal_all_to_c64(&mut rng, &mut g), cards);
    }

    #[quickcheck]
    fn test_dead_and_reset(available: Card64) {
        let mut rng = rand::rng();

        let mut g = CardGen::new::<false>(!available);

        assert_eq!(deal_all_to_c64(&mut rng, &mut g), available);
        assert_eq!(deal_all_to_c64(&mut rng, &mut g), Card64::default());

        g.reset();

        assert_eq!(deal_all_to_c64(&mut rng, &mut g), available);
    }
}
