use super::nc2;

const RANK_A: u8 = 12;

/// returns the position of most signicicant set bit
#[inline]
const fn pos_msb(v: u16) -> u8 {
    15 - v.leading_zeros().to_le_bytes()[0]
}

#[inline]
const fn nc3(n: u8) -> u16 {
    nc2(n) as u16 * (n - 2) as u16 / 3 // (n-2)*(n-1)*n/(3*2)
}

/// returns the index combination of 2 ranks
/// input must be valid bit flags of 2 or more ranks
pub const fn combination_of_2_ranks_to_index(ranks: u16) -> u8 {
    let high = pos_msb(ranks);
    let low = pos_msb(ranks & !(1 << high));

    nc2(high) + low
}

/// returns the index combination of 3 ranks
/// input must be valid bit flags of 3 or more ranks
pub const fn combination_of_3_ranks_to_index(ranks: u16) -> u16 {
    let high = pos_msb(ranks);
    let mid = pos_msb(ranks & !(1 << high));
    let low = pos_msb(ranks & !(1 << high | 1 << mid));

    nc3(high) + nc2(mid) as u16 + low as u16
}

/// restore index to 2 ranks
pub fn combination_of_2_index_to_ranks(mut idx: u8) -> u16 {
    let mut r = 2;
    let mut res = 0;

    for n in (0..=RANK_A).rev() {
        let m = if r == 2 { nc2(n) } else { n };

        if m <= idx {
            idx -= m;
            r -= 1;
            res |= 1 << n;

            if r == 0 {
                break;
            }
        }
    }

    res
}

/// restore index to 3 ranks
pub fn combination_of_3_index_to_ranks(mut idx: u16) -> u16 {
    #[inline]
    const fn ncr(n: u8, r: u8) -> u16 {
        match r {
            3.. => nc3(n),
            2 => nc2(n) as u16,
            0 | 1 => n as u16,
        }
    }

    let mut r = 3;
    let mut res = 0;

    for n in (0..=RANK_A).rev() {
        let m = ncr(n, r);

        if m <= idx {
            idx -= m;
            r -= 1;
            res |= 1 << n;

            if r == 0 {
                break;
            }
        }
    }

    res
}

#[cfg_attr(coverage_nightly, coverage(off))]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    mod utils {
        pub fn factorial(n: usize) -> usize {
            (2..=n).fold(1, usize::wrapping_mul)
        }

        /// n choose r; returns 0 whenever n < r
        pub fn ncr(n: usize, r: usize) -> usize {
            if n == 0 {
                0
            } else {
                factorial(n) / factorial(n.saturating_sub(r)) / factorial(r)
            }
        }

        pub fn index_to_combination(
            mut n: usize,
            mut r: usize,
            mut idx: usize,
        ) -> Vec<usize> {
            let mut res = vec![];

            while r > 0 {
                n -= 1;

                if ncr(n, r) <= idx {
                    idx -= ncr(n, r);
                    r -= 1;

                    res.push(n);
                }
            }

            res
        }

        pub fn combination_to_index(mut xs: Vec<usize>) -> usize {
            let mut r = xs.len();
            let mut index = 0;

            xs.sort_unstable();

            while r > 0 {
                index += ncr(xs.pop().unwrap(), r);

                r -= 1;
            }

            index
        }
    }

    use utils::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(4), 24);
    }

    #[test]
    fn test_ncr() {
        assert_eq!(ncr(1, 1), 1);
        assert_eq!(ncr(5, 2), 10);
        assert_eq!(ncr(4, 2), 6);
        assert_eq!(ncr(0, 2), 0);
        assert_eq!(ncr(0, 1), 0);
        assert_eq!(ncr(1, 2), 0);
    }

    #[test]
    fn test_combination_index() {
        assert_eq!(combination_to_index(vec![1, 0]), 0);
        assert_eq!(combination_to_index(vec![2, 0]), 1);
        assert_eq!(combination_to_index(vec![2, 1]), 2);
        assert_eq!(combination_to_index(vec![3, 0]), 3);
        assert_eq!(combination_to_index(vec![3, 1]), 4);
        assert_eq!(combination_to_index(vec![3, 2]), 5);
        assert_eq!(combination_to_index(vec![4, 0]), 6);
        assert_eq!(combination_to_index(vec![4, 1]), 7);
        assert_eq!(combination_to_index(vec![4, 2]), 8);
        assert_eq!(combination_to_index(vec![4, 3]), 9);
    }

    #[test]
    fn test_index_to_combination() {
        assert_eq!(index_to_combination(5, 3, 0), vec![2, 1, 0]);
        assert_eq!(index_to_combination(5, 3, 1), vec![3, 1, 0]);
        assert_eq!(index_to_combination(5, 3, 2), vec![3, 2, 0]);
        assert_eq!(index_to_combination(5, 3, 3), vec![3, 2, 1]);
        assert_eq!(index_to_combination(5, 3, 4), vec![4, 1, 0]);
        assert_eq!(index_to_combination(5, 3, 5), vec![4, 2, 0]);
        assert_eq!(index_to_combination(5, 3, 6), vec![4, 2, 1]);
        assert_eq!(index_to_combination(5, 3, 7), vec![4, 3, 0]);
        assert_eq!(index_to_combination(5, 3, 8), vec![4, 3, 1]);
        assert_eq!(index_to_combination(5, 3, 9), vec![4, 3, 2]);
    }

    #[quickcheck]
    fn test_combination_of_3_ranks_to_index(
        r1: Rank,
        r2: Rank,
        r3: Rank,
    ) -> TestResult {
        let ranks = Rank16::from([r1, r2, r3].as_ref());

        if ranks.count() < 3 {
            return TestResult::discard();
        }

        let index =
            combination_to_index(vec![r1 as usize, r2 as usize, r3 as usize]);

        let int = ranks.to_u16();

        let maybe_more_than_three =
            int | 1 << int.trailing_zeros().saturating_sub(1);

        assert_eq!(index, usize::from(combination_of_3_ranks_to_index(int)));
        assert_eq!(
            combination_of_3_ranks_to_index(int),
            combination_of_3_ranks_to_index(maybe_more_than_three)
        );

        TestResult::passed()
    }

    #[quickcheck]
    fn test_combination_of_3_index_to_ranks(ranks: Rank16) -> TestResult {
        if ranks.count() < 3 {
            return TestResult::discard();
        }

        let ranks = ranks.to_u16();

        let index = combination_of_3_ranks_to_index(ranks);
        let res = combination_of_3_index_to_ranks(index);

        if (ranks & res == res) && (ranks & !res < res) {
            TestResult::passed()
        } else {
            TestResult::error(format!(
                "input: {:?} res: {:?}",
                Rank16::from_u16(ranks),
                Rank16::from_u16(res)
            ))
        }
    }

    #[quickcheck]
    fn test_combination_of_2_ranks_to_index(r1: Rank, r2: Rank) -> TestResult {
        if r1 == r2 {
            return TestResult::discard();
        }

        let index = combination_to_index(vec![r1 as usize, r2 as usize]);

        let ranks = Rank16::from([r1, r2].as_ref());

        let int = ranks.to_u16();

        let maybe_more_than_two =
            int | 1 << int.trailing_zeros().saturating_sub(1);

        assert_eq!(index, usize::from(combination_of_2_ranks_to_index(int)));
        assert_eq!(
            combination_of_2_ranks_to_index(int),
            combination_of_2_ranks_to_index(maybe_more_than_two)
        );

        TestResult::passed()
    }

    #[quickcheck]
    fn test_combination_of_2_index_to_ranks(ranks: Rank16) -> TestResult {
        if ranks.count() < 2 {
            return TestResult::discard();
        }

        let ranks = ranks.to_u16();

        let index = combination_of_2_ranks_to_index(ranks);
        let res = combination_of_2_index_to_ranks(index);

        if (ranks & res == res) && (ranks & !res < res) {
            TestResult::passed()
        } else {
            TestResult::error(format!(
                "input: {:?} res: {:?}",
                Rank16::from_u16(ranks),
                Rank16::from_u16(res)
            ))
        }
    }
}
