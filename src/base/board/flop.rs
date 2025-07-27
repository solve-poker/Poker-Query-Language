use super::{mem, Card, Card64, From, Into, PQLCardCount, Rank, Rank16};

#[derive(Clone, Copy, Debug, Into, From, Default, Eq, PartialEq)]
pub struct Flop(pub Card, pub Card, pub Card);

impl Flop {
    pub(crate) fn count_by_rank(self, r: Rank) -> PQLCardCount {
        u8::from(self.0.r == r)
            + u8::from(self.1.r == r)
            + u8::from(self.2.r == r)
    }

    pub(crate) fn sorted_ranks(self) -> (Rank, Rank, Rank) {
        let (mut x, mut y, mut z) = (self.0.r, self.1.r, self.2.r);

        if x < y {
            mem::swap(&mut x, &mut y);
        }

        if y < z {
            mem::swap(&mut y, &mut z);
        }

        if x < y {
            mem::swap(&mut x, &mut y);
        }

        (x, y, z)
    }
}

impl From<Flop> for Card64 {
    fn from(flop: Flop) -> Self {
        let mut c = Self::empty();

        c.set(flop.0);
        c.set(flop.1);
        c.set(flop.2);

        c
    }
}

impl From<Flop> for Rank16 {
    fn from(flop: Flop) -> Self {
        let mut r = Self::empty();

        r.set(flop.0.r);
        r.set(flop.1.r);
        r.set(flop.2.r);

        r
    }
}

impl From<[Card; 3]> for Flop {
    fn from(a: [Card; 3]) -> Self {
        Self(a[0], a[1], a[2])
    }
}

impl From<Flop> for [Card; 3] {
    fn from(flop: Flop) -> Self {
        [flop.0, flop.1, flop.2]
    }
}

#[cfg(any(test, feature = "benchmark"))]
impl From<&[Card]> for Flop {
    fn from(cs: &[Card]) -> Self {
        Self(cs[0], cs[1], cs[2])
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::*;

    impl Arbitrary for Flop {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let cards = CardN::<3>::arbitrary(g);

            Self(cards[0], cards[1], cards[2])
        }
    }

    #[quickcheck]
    fn test_into_rank16_and_array(flop: Flop) {
        let r: Rank16 = flop.into();
        let arr: [_; 3] = flop.into();

        assert!(r.contains_rank(flop.0.r));
        assert!(r.contains_rank(flop.1.r));
        assert!(r.contains_rank(flop.2.r));

        assert_eq!(arr[0], flop.0);
        assert_eq!(arr[1], flop.1);
        assert_eq!(arr[2], flop.2);
    }
}
