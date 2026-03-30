// TODO refactor this file

use super::HandRating;

type PlayerCount = u8;
type Chip = u16;
type Utility = f64;

#[allow(clippy::missing_panics_doc)]
#[allow(clippy::cast_possible_truncation)]
pub fn calculate_payoffs(
    pot: &[Chip],
    ratings: &[HandRating],
    active: &[bool],
) -> Vec<Utility> {
    let mut remain: Vec<_> = pot.to_vec();
    let n = remain.len();
    let mut payoffs = vec![0.0; n];

    while remain.iter().any(|&chips| chips > 0) {
        let eligible: Vec<usize> = remain
            .iter()
            .enumerate()
            .filter(|&(i, &chips)| active[i] && chips > 0)
            .map(|(i, _)| i)
            .collect();

        let min_bet = eligible
            .iter()
            .map(|&i| remain[i])
            .min()
            .unwrap_or_else(|| panic!("no min bet: {remain:?}"));

        let max_strength = eligible.iter().map(|&i| ratings[i]).max().unwrap();

        let winners: Vec<usize> = eligible
            .into_iter()
            .filter(|&i| ratings[i] == max_strength)
            .collect();
        let winners_count = winners.len();

        let pot_size: Chip =
            remain.iter().map(|&chips| chips.min(min_bet)).sum();

        let share = Utility::from(pot_size)
            / Utility::from(winners_count as PlayerCount);
        for i in winners {
            payoffs[i] += share;
        }

        for rem in &mut remain {
            if *rem > 0 {
                *rem = rem.saturating_sub(min_bet);
            }
        }
    }

    for (i, v) in pot.iter().enumerate() {
        payoffs[i] -= Utility::from(*v);
    }

    payoffs
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use super::*;

    #[allow(clippy::absolute_paths)]
    fn to_rating(i: u16) -> HandRating {
        unsafe { std::mem::transmute(i) }
    }

    fn assert_fvec_eq(a: &[Utility], b: &[Utility]) {
        assert_abs_diff_eq!(a, b);
    }

    #[test]
    fn test_calculate_payoffs() {
        let pot = [30, 90, 30, 50, 90];
        let ratings = [2, 2, 3, 3, 1].map(to_rating);
        let res =
            calculate_payoffs(&pot, &ratings, &[true, true, true, true, true]);
        assert_fvec_eq(&res, &[-30.0, -10.0, 45.0, 85.0, -90.0]);

        let pot = [20, 20];
        let ratings = [1, 2].map(to_rating);
        let res = calculate_payoffs(&pot, &ratings, &[true, true]);
        assert_fvec_eq(&res, &[-20.0, 20.0]);
    }

    #[test]
    fn test_calculate_payoffs_folded() {
        let pot = [2, 20];
        let ratings = [10, 0].map(to_rating);
        let res = calculate_payoffs(&pot, &ratings, &[false, true]);
        assert_fvec_eq(&res, &[-2.0, 2.0]);
    }
}
