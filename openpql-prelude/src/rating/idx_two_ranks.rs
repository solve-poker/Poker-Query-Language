use super::Rank16;

#[inline]
const fn nc2(n: u8) -> u8 {
    n * (n - 1) / 2
}

/// returns the position of most significant set bit
#[inline]
const fn pos_msb(v: u16) -> u8 {
    15 - v.leading_zeros().to_le_bytes()[0]
}

#[derive(Debug, Clone, Copy)]
pub struct IdxTwoRanks(pub(crate) u8);

impl IdxTwoRanks {
    pub(crate) const MASK_USED: u8 = 0b0111_1111;

    pub(crate) const fn from_r16(r16: Rank16) -> Self {
        let high = pos_msb(r16.0);
        let low = pos_msb(r16.0 & !(1 << high));

        Self(nc2(high) + low)
    }

    pub(crate) fn to_r16(self) -> Rank16 {
        const RANK_A: u8 = 12;
        let mut r = 2;
        let mut idx = self.0;
        let mut r16 = 0;

        for n in (0..=RANK_A).rev() {
            let m = if r == 2 { nc2(n) } else { n };

            if m <= idx {
                idx -= m;
                r -= 1;
                r16 |= 1 << n;

                if r == 0 {
                    break;
                }
            }
        }

        Rank16(r16)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_from_r16() {
        assert_eq!(IdxTwoRanks::from_r16(r16!("32")).0, 0);
        assert_eq!(IdxTwoRanks::from_r16(r16!("42")).0, 1);
        assert_eq!(IdxTwoRanks::from_r16(r16!("43")).0, 2);
        assert_eq!(IdxTwoRanks::from_r16(r16!("52")).0, 3);
        assert_eq!(IdxTwoRanks::from_r16(r16!("62")).0, 6);
        assert_eq!(IdxTwoRanks::from_r16(r16!("72")).0, 10);
        assert_eq!(IdxTwoRanks::from_r16(r16!("82")).0, 15);
        assert_eq!(IdxTwoRanks::from_r16(r16!("92")).0, 21);
        assert_eq!(IdxTwoRanks::from_r16(r16!("T2")).0, 28);
        assert_eq!(IdxTwoRanks::from_r16(r16!("J2")).0, 36);
        assert_eq!(IdxTwoRanks::from_r16(r16!("Q2")).0, 45);
        assert_eq!(IdxTwoRanks::from_r16(r16!("K2")).0, 55);
        assert_eq!(IdxTwoRanks::from_r16(r16!("A2")).0, 66);
        assert_eq!(IdxTwoRanks::from_r16(r16!("AK")).0, 77);
    }

    #[quickcheck]
    fn test_bijection(r1: Rank, r2: Rank) -> TestResult {
        if r1 == r2 {
            return TestResult::discard();
        }

        let r16 = Rank16::from([r1, r2].as_slice());
        let idx = IdxTwoRanks::from_r16(r16);

        TestResult::from_bool(r16 == idx.to_r16())
    }
}
