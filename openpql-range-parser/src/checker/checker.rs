use super::{Array, Card, Error, Expr, Idx, parse_expr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Checker<
    const N: usize = 2,
    const B: bool = false,
    const SD: bool = false,
> where
    [Idx; N]: Array<Item = Idx>,
{
    expr: Expr<N, B>,
}

impl<const N: usize, const B: bool, const SD: bool> Checker<N, B, SD>
where
    [Idx; N]: Array<Item = Idx>,
{
    pub fn from_src(src: &str) -> Result<Self, Error> {
        Ok(Self {
            expr: parse_expr(SD, src).and_then(|expr| Expr::try_from(*expr))?,
        })
    }

    #[inline]
    pub fn is_satisfied(&self, cs: &[Card]) -> bool {
        self.expr.is_satisfied(cs)
    }
}

impl<const N: usize, const B: bool, const SD: bool> Default
    for Checker<N, B, SD>
where
    [Idx; N]: Array<Item = Idx>,
{
    fn default() -> Self {
        Self::from_src("*").unwrap()
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    fn assert_checker<const N: usize, const B: bool>(
        s: &str,
        hands_in: &[&str],
        hands_not_in: &[&str],
    ) where
        [Idx; N]: Array<Item = Idx>,
    {
        let checker = Checker::<N, B, false>::from_src(s).unwrap();

        for hand in hands_in {
            assert!(
                checker.is_satisfied(&cards![hand]),
                "unexpected: {hand} not in {s}"
            );
        }

        for hand in hands_not_in {
            assert!(
                !checker.is_satisfied(&cards![hand]),
                "unexpected: {hand} in {s}"
            );
        }
    }

    #[test]
    fn test_default() {
        assert_eq!(
            Checker::default(),
            Checker::<4, false, false>::from_src("*").unwrap()
        );
    }

    #[test]
    fn test_card() {
        assert_checker::<4, false>("AwRsOyKd", &["AhJsTcKd"], &[]);
    }

    #[test]
    fn test_rank_const() {
        assert_checker::<2, false>("AK", &["As", "Ks", "As Ks"], &["As 2s"]);
    }

    #[test]
    fn test_rank_var() {
        assert_checker::<2, false>("RR", &["As Ah", "Ks Kh"], &["As Ks"]);
        assert_checker::<2, false>("RO", &["As Ks"], &["As Ah", "Ks Kh"]);
        assert_checker::<4, false>("AKQB", &["As Ks Qs 2s"], &["As Ks Qs Qh"]);
        assert_checker::<4, false>("AKBB", &["As Ks Qs Qh"], &["As Ks Qs Jh"]);
        assert_checker::<4, false>("RRRO", &["As Ah Ad Kc"], &["As Ah Ad Ac"]);
        assert_checker::<4, false>(
            "[A,K][4-]2sB",
            &["As 3s 2s Ah", "As 3s 2s 3h"],
            &[],
        );
    }

    #[test]
    fn test_suit_const() {
        assert_checker::<2, false>("sh", &["As", "Kh", "Ah Ks"], &["As 2s"]);
    }

    #[test]
    fn test_suit_var() {
        assert_checker::<2, false>("sw", &["As Ah"], &["As Ks"]);
        assert_checker::<2, false>("xx", &["As Ks", "Ah Kh"], &["As Kh"]);
        assert_checker::<2, false>("xy", &["As Kh"], &["As Ks", "Ah Kh"]);
        assert_checker::<4, false>("ssww", &["As Ks Qh Jh"], &["As Ks Qh Jd"]);
        assert_checker::<4, false>("xxxy", &["As Ks Qs Jh"], &["As Ks Qs Js"]);
        assert_checker::<4, false>(
            "[h][4s-]2dw",
            &["Ah 3s 2d Ts", "Ah 3s 2d Th", "Ah 3s 2d Tc"],
            &["Ah 3s 2d Td"],
        );
    }

    #[test]
    fn test_var() {
        assert_checker::<2, false>("AxRs", &["Ah Ks"], &["As Kh"]);
        assert_checker::<2, false>("RxOy", &["Ah Ks"], &["As Ah", "As Ks"]);
        assert_checker::<5, true>(
            "2xRsOyzN",
            &["2h 3s 4d 5c 6s"],
            &["2h 3s 4h 5c 6s"],
        );
    }

    #[test]
    fn test_span() {
        assert_checker::<2, false>("AKs-", &["As Ks", "3h"], &["As Kh"]);
        assert_checker::<2, false>("22+", &["As Ah"], &["As Kh"]);
        assert_checker::<4, false>(
            "AKQT-",
            &["As Ks", "Qs Th", "3h"],
            &["2s 3s 4s"],
        );
        assert_checker::<2, false>("AK-JT", &["Qs Jh"], &["Ts 9h"]);
    }

    #[test]
    fn test_list() {
        assert_checker::<2, false>(
            "[2c,A,s]Td",
            &["Td 2c", "Td Ah", "Td Ks"],
            &["Td 2d"],
        );
        assert_checker::<4, false>(
            "[2c,A,s]Td9d8d",
            &["Td9d8d 2c", "Td9d8d Ah", "Td9d8d Ks"],
            &["Td9d8d 2d"],
        );
        assert_checker::<4, false>(
            "[s][h][d][c]",
            &["2s 2h 3d 3c"],
            &["2s 2h 3c 3c"],
        );
    }

    #[test]
    fn test_board() {
        assert_checker::<5, true>(
            "AKQJ[T,3s]",
            &["As Ks Qs Js Ts", "Ks Qs As Js Ts", "As Ks Qs Js 3s"],
            &["As Ks Qs Ts Js", "Ts Ks Qs Js As", "As Ks Qs Js 3h"],
        );
        assert_checker::<5, true>(
            "AA,JJ",
            &["Js Jh 2d 2c 3s", "Js 2h Jd 2c 3s"],
            &["Js 2h 2s Jc Jd"],
        );
        assert_checker::<5, true>(
            "222[2]s",
            &["2s2h2d2c 3s"],
            &["2s2h2d2c 3h"],
        );
    }

    #[quickcheck]
    fn test_any(cards: CardN<5>) -> TestResult {
        fn to_str(cs: &[Card]) -> String {
            cs.iter().map(ToString::to_string).join("")
        }

        assert_checker::<2, false>("*", &[&to_str(&cards[..2])], &[]);
        assert_checker::<4, false>("*", &[&to_str(&cards[..4])], &[]);
        assert_checker::<5, true>("*", &[&to_str(&cards[..])], &[]);

        TestResult::passed()
    }

    #[test]
    fn test_not() {
        assert_checker::<2, false>("A!K", &["As Qs"], &["As Ks"]);
    }

    #[test]
    fn test_or() {
        assert_checker::<2, false>("AA,KK", &["As Ah", "Ks Kh"], &[]);
    }

    #[test]
    fn test_and() {
        assert_checker::<2, false>("A:K", &["As Kh"], &[]);
    }

    fn e<const N: usize>(s: &str) -> Error
    where
        [Idx; N]: Array<Item = Idx>,
    {
        Checker::<N, false, false>::from_src(s).unwrap_err()
    }

    #[test]
    fn test_error() {
        assert_eq!(e::<2>("AK*"), Error::TooManyCardsInRange((0, 3)));
        assert_eq!(e::<2>("*!AAA"), Error::TooManyCardsInRange((2, 5)));
        assert_eq!(e::<2>("*:AAA"), Error::TooManyCardsInRange((2, 5)));
        assert_eq!(e::<2>("*,AAA"), Error::TooManyCardsInRange((2, 5)));
        assert_eq!(e::<2>("AAA!*"), Error::TooManyCardsInRange((0, 3)));
        assert_eq!(e::<2>("AAA:*"), Error::TooManyCardsInRange((0, 3)));
        assert_eq!(e::<2>("AAA,*"), Error::TooManyCardsInRange((0, 3)));
    }
}
