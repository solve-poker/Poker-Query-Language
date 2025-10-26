use super::{Card, Card64, Vec};

#[derive(Clone, Debug, Default)]
pub struct CardGen {
    init: Vec<Card>,
    used: Vec<Card>,
    unused: Vec<Card>,
}

impl CardGen {
    pub fn new<const SD: bool>(dead_cards: Card64) -> Self {
        let mut unused = vec![];

        for card in Card64::all::<SD>().iter() {
            if !dead_cards.contains_card(card) {
                unused.push(card);
            }
        }

        Self {
            init: unused.clone(),
            used: vec![],
            unused,
        }
    }

    pub fn deal(&mut self, rng: &mut impl rand::Rng) -> Option<Card> {
        let n = self.unused.len();
        if n == 0 {
            return None;
        }

        let idx = rng.random_range(0..n);
        let card = self.unused.remove(idx);
        self.used.push(card);

        Some(card)
    }

    pub fn unset(&mut self, c64: Card64) {
        for c in c64.iter() {
            self.unset_card(c);
        }
    }

    pub fn unset_card(&mut self, card: Card) {
        debug_assert!(!self.unused.contains(&card));

        self.used.retain(|&c| c != card);
        self.unused.push(card);
    }

    pub fn reset(&mut self) {
        self.unused = self.init.clone();
    }
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
