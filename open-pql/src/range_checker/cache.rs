use super::{Card, Card64, Checker, Error, FxHashMap, Itertools, PQLCardCount};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CachedChecker<const N: usize = 2, const B: bool = false> {
    cache: FxHashMap<u64, bool>,
    checker: Checker<N, B>,
}

impl<const N: usize, const B: bool> CachedChecker<N, B> {
    pub fn from_src(s: &str) -> Result<Self, Error> {
        Ok(Self {
            cache: FxHashMap::default(),
            checker: Checker::from_src(s)?,
        })
    }

    pub fn is_satisfied(&mut self, cs: &[Card]) -> bool {
        if !B || cs.len() <= 3 {
            self.is_satisfied_n(cs)
        } else if self.is_satisfied_n(&cs[..3]) {
            self.checker.is_satisfied(cs)
        } else {
            false
        }
    }

    fn is_satisfied_n(&mut self, cs: &[Card]) -> bool {
        let key = Card64::from(cs).to_u64();

        if let Some(b) = self.cache.get(&key) {
            *b
        } else {
            let b = self.checker.is_satisfied(cs);
            self.cache.insert(key, b);

            b
        }
    }

    pub fn init_cache(&mut self) {
        let mut mem = vec![];

        let n_cards = if B { 3 } else { N };

        'next_combination: for v in
            Card::ARR_ALL.into_iter().combinations(n_cards)
        {
            mem.clear();

            for c in &v {
                mem.push(*c);
                if !self.is_satisfied(&mem) {
                    continue 'next_combination;
                }
            }
        }
    }

    pub const fn n_cards(&self) -> PQLCardCount {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    fn c<const N: usize, const B: bool>(s: &str) -> CachedChecker<N, B> {
        CachedChecker::<N, B>::from_src(s).unwrap()
    }

    #[test]
    fn test_player() {
        let mut c = c::<2, false>("AA");
        c.init_cache();

        assert!(c.is_satisfied(&cards!["AsAh"]));
        assert_eq!(c.cache.get(&c64!("AsAh").to_u64()), Some(true).as_ref());
        assert_eq!(c.cache.get(&c64!("AsAd").to_u64()), Some(true).as_ref());
        assert_eq!(c.cache.get(&c64!("AsAc").to_u64()), Some(true).as_ref());
        assert_eq!(c.cache.get(&c64!("Kc").to_u64()), Some(false).as_ref());
    }

    #[test]
    fn test_board() {
        let mut c = c::<5, true>("2s3s4sAsKs");
        c.init_cache();

        assert!(c.is_satisfied(&cards!["2s3s4sAsKs"]));
        assert!(!c.is_satisfied(&cards!["2s3s4sKsAs"]));
    }
}
