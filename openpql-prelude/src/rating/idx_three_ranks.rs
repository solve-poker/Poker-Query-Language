use super::Rank16;

#[inline]
const fn nc2(n: u8) -> u16 {
    (n * (n - 1) / 2) as u16
}

#[inline]
const fn nc3(n: u8) -> u16 {
    nc2(n) * (n - 2) as u16 / 3 // (n-2)*(n-1)*n/(3*2)
}

#[inline]
const fn ncr(n: u8, r: u8) -> u16 {
    match r {
        3.. => nc3(n),
        2 => nc2(n),
        0 | 1 => n as u16,
    }
}

/// returns the position of most significant set bit
#[inline]
const fn pos_msb(v: u16) -> u8 {
    15 - v.leading_zeros().to_le_bytes()[0]
}

#[derive(Debug, Clone, Copy)]
pub struct IdxThreeRanks(pub(crate) u16);

impl IdxThreeRanks {
    pub(crate) const MASK_USED: u16 = 0b1_1111_1111;

    pub(crate) const fn from_r16(r16: Rank16) -> Self {
        let high = pos_msb(r16.0);
        let mid = pos_msb(r16.0 & !(1 << high));
        let low = pos_msb(r16.0 & !(1 << high | 1 << mid));

        Self(nc3(high) + nc2(mid) + low as u16)
    }

    pub(crate) fn to_r16(self) -> Rank16 {
        const RANK_A: u8 = 12;

        let mut r = 3;
        let mut idx = self.0;
        let mut r16 = 0;

        for n in (0..=RANK_A).rev() {
            let m = ncr(n, r);

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
        assert_eq!(IdxThreeRanks::from_r16(r16!("432")).0, 0);
        assert_eq!(IdxThreeRanks::from_r16(r16!("532")).0, 1);
        assert_eq!(IdxThreeRanks::from_r16(r16!("542")).0, 2);
        assert_eq!(IdxThreeRanks::from_r16(r16!("543")).0, 3);
        assert_eq!(IdxThreeRanks::from_r16(r16!("632")).0, 4);
        assert_eq!(IdxThreeRanks::from_r16(r16!("732")).0, 10);
        assert_eq!(IdxThreeRanks::from_r16(r16!("832")).0, 20);
        assert_eq!(IdxThreeRanks::from_r16(r16!("932")).0, 35);
        assert_eq!(IdxThreeRanks::from_r16(r16!("T32")).0, 56);
        assert_eq!(IdxThreeRanks::from_r16(r16!("J32")).0, 84);
        assert_eq!(IdxThreeRanks::from_r16(r16!("Q32")).0, 120);
        assert_eq!(IdxThreeRanks::from_r16(r16!("K32")).0, 165);
        assert_eq!(IdxThreeRanks::from_r16(r16!("A32")).0, 220);
        assert_eq!(IdxThreeRanks::from_r16(r16!("AKQ")).0, 285);
    }

    #[quickcheck]
    fn test_bijection(r1: Rank, r2: Rank, r3: Rank) -> TestResult {
        let r16 = Rank16::from([r1, r2, r3].as_slice());
        if r16.count() != 3 {
            return TestResult::discard();
        }

        let idx = IdxThreeRanks::from_r16(r16);

        TestResult::from_bool(r16 == idx.to_r16())
    }
}
