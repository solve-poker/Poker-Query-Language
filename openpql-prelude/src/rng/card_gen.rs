use super::{Card, Card64, CardMask};
use crate::{RankIdx, SuitIdx};

#[derive(Clone, Debug, Default)]
pub struct CardGen {
    mask: CardMask,   // 可用牌掩码 == 初始可用牌掩码
    pub used: Card64, // 已用牌集
}

const EMPTY_MASK_U64: u64 = 0u64;
const OFFSET_SUIT: u32 = 16;

impl CardGen {
    pub fn new<const SD: bool>(dead_cards: Card64) -> Self {
        let all = Card64::all::<SD>();
        let mask = CardMask::from(all & !dead_cards);
        Self {
            mask,
            used: Card64::default(),
        }
    }

    pub fn deal(&mut self, rng: &mut impl rand::Rng) -> Option<Card> {
        let remaining = self.mask ^ self.used.0;
        if remaining == EMPTY_MASK_U64 {
            return None;
        }
        let pos = random_set_bit_pos_64(remaining, rng)?;
        let bit = 1u64 << pos;
        self.used.0 |= bit;
        Some(card_from_bit_position(pos))
    }

    pub fn unset(&mut self, c64: Card64) {
        self.used &= !c64;
    }

    pub fn unset_card(&mut self, card: Card) {
        let c64 = Card64::from(card);
        if self.used.contains_card(card) {
            self.unset(c64);
        }
    }

    pub fn reset(&mut self) {
        self.used = Card64::default();
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

type Idx = i8;

#[allow(clippy::cast_possible_truncation)]
const fn card_from_bit_position(pos: u32) -> Card {
    let suit_idx = (pos / OFFSET_SUIT) as Idx;
    let rank_idx = (pos % OFFSET_SUIT) as Idx;
    Card::new(
        RankIdx(rank_idx).to_rank().unwrap(),
        SuitIdx(suit_idx).to_suit().unwrap(),
    )
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
pub mod tests {
    use super::*;
    fn mk_exhausted_card_gen() -> CardGen {
        let mut g = CardGen::new::<false>(Card64::default());
        g.used = g.mask.into();
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
