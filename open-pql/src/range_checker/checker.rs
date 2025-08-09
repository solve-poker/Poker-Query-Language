use super::{Card, Error, Expr, parse_range};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Checker<const N: usize = 2, const B: bool = false> {
    expr: Expr<N, B>,
}

impl<const N: usize, const B: bool> Checker<N, B> {
    pub fn from_src(s: &str) -> Result<Self, Error> {
        Ok(Self {
            expr: Expr::try_from(*parse_range(s)?)?,
        })
    }

    #[inline]
    pub fn is_satisfied(&self, cs: &[Card]) -> bool {
        self.expr.is_satisfied(cs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    fn c<const N: usize, const B: bool>(s: &str) -> Checker<N, B> {
        Checker::<N, B>::from_src(s).unwrap()
    }

    fn e<const N: usize>(s: &str) -> range_parser::Error {
        Checker::<N, false>::from_src(s).unwrap_err()
    }

    #[test]
    fn test_card() {
        let e: Checker<4> = c("AwRsOyKd");

        assert!(e.is_satisfied(&cards!["AhJsTcKd"]));
    }

    #[test]
    fn test_rank_const() {
        let e: Checker<2> = c("AK");

        assert!(e.is_satisfied(&cards!["As"]));
        assert!(e.is_satisfied(&cards!["Ks"]));
        assert!(e.is_satisfied(&cards!["As Ks"]));
        assert!(!e.is_satisfied(&cards!["As 2s"]));
    }

    #[test]
    fn test_rank_var() {
        let e: Checker<4> = c("AKQB");

        assert!(e.is_satisfied(&cards!["As Ks Qs 2s"]));
        assert!(!e.is_satisfied(&cards!["As Ks Qs Qh"]));

        let e: Checker<4> = c("AKBB");

        assert!(e.is_satisfied(&cards!["As Ks Qs Qh"]));
        assert!(!e.is_satisfied(&cards!["As Ks Qs Jh"]));

        let e: Checker<4> = c("[A,K][4-]2sB");

        assert!(e.is_satisfied(&cards!["As 3s 2s Ah"]));
        assert!(e.is_satisfied(&cards!["As 3s 2s 3h"]));

        let e: Checker<3> = c("ARR");

        assert!(e.is_satisfied(&cards!["As Ks"]));
        assert!(!e.is_satisfied(&cards!["As Ah"]));

        let e: Checker<3> = c("ARO");

        assert!(e.is_satisfied(&cards!["As Ks Qs"]));
        assert!(e.is_satisfied(&cards!["As Ks"]));
        assert!(!e.is_satisfied(&cards!["As Ks Kh"]));
    }

    #[test]
    fn test_suit_const() {
        let e: Checker<2> = c("sh");

        assert!(e.is_satisfied(&cards!["As"]));
        assert!(e.is_satisfied(&cards!["Kh"]));
        assert!(e.is_satisfied(&cards!["Ah Ks"]));
        assert!(!e.is_satisfied(&cards!["As 2s"]));
    }

    #[test]
    fn test_suit_var() {
        let e: Checker<2> = c("sw");

        assert!(e.is_satisfied(&cards!["As Ah"]));
        assert!(!e.is_satisfied(&cards!["As Ks"]));

        let e: Checker<4> = c("ssww");

        assert!(e.is_satisfied(&cards!["As Ks Qh Jh"]));
        assert!(!e.is_satisfied(&cards!["As Ks Qh Jd"]));

        let e: Checker<4> = c("[h][4s-]2dw");

        assert!(e.is_satisfied(&cards!["Ah 3s 2d Ts"]));
        assert!(e.is_satisfied(&cards!["Ah 3s 2d Th"]));
        assert!(!e.is_satisfied(&cards!["Ah 3s 2d Td"]));
        assert!(e.is_satisfied(&cards!["Ah 3s 2d Tc"]));

        let e: Checker<3> = c("sxx");

        assert!(e.is_satisfied(&cards!["As Kh"]));
        assert!(!e.is_satisfied(&cards!["As Ks"]));

        let e: Checker<3> = c("sxy");

        assert!(e.is_satisfied(&cards!["As Kh Qd"]));
        assert!(e.is_satisfied(&cards!["As Kh"]));
        assert!(!e.is_satisfied(&cards!["As Kh Qh"]));
    }

    #[test]
    fn test_span() {
        let e: Checker<2> = c("AKs-");

        assert!(e.is_satisfied(&cards!["As Ks"]));
        assert!(!e.is_satisfied(&cards!["As Kh"]));
        assert!(e.is_satisfied(&cards!["3h"]));

        let e: Checker<4> = c("AKQT-");

        assert!(e.is_satisfied(&cards!["As Ks"]));
        assert!(e.is_satisfied(&cards!["Qs Th"]));
        assert!(e.is_satisfied(&cards!["3h"]));
        assert!(!e.is_satisfied(&cards!["2s 3s 4s"]));

        let e: Checker<3> = c("AKK+");

        assert!(e.is_satisfied(&cards!["As Ah"]));
        assert!(!e.is_satisfied(&cards!["As Ah Ks"]));

        let e: Checker<2> = c("AK-JT");

        assert!(e.is_satisfied(&cards!["Qs Jh"]));
        assert!(!e.is_satisfied(&cards!["Ts 9h"]));
    }

    #[test]
    fn test_list() {
        let e: Checker<2> = c("[2c,A,s]Td");

        assert!(e.is_satisfied(&cards!["Td 2c"]));
        assert!(e.is_satisfied(&cards!["Td Ah"]));
        assert!(e.is_satisfied(&cards!["Td Ks"]));
        assert!(!e.is_satisfied(&cards!["Td 2d"]));
    }

    #[test]
    fn test_board() {
        let e: Checker<5, true> = c("AKQJT");
        assert!(e.is_satisfied(&cards!["As Ks Qs Js Ts"]));
        assert!(e.is_satisfied(&cards!["Ks Qs As Js Ts"]));
        assert!(!e.is_satisfied(&cards!["As Ks Qs Ts Js"]));
        assert!(!e.is_satisfied(&cards!["Ts Ks Qs Js As"]));

        let e: Checker<5, true> = c("AA,JJ");
        assert!(e.is_satisfied(&cards!["Js Jh 2d 2c 3s"]));
        assert!(e.is_satisfied(&cards!["Js 2h Jd 2c 3s"]));
        assert!(!e.is_satisfied(&cards!["Js 2h 2s Jc Jd"]));
    }

    #[test]
    fn test_not() {
        let e: Checker<2> = c("A!K");

        assert!(e.is_satisfied(&cards!["As Qs"]));
        assert!(!e.is_satisfied(&cards!["As Ks"]));
    }

    #[test]
    fn test_or() {
        let e: Checker<2> = c("AA,KK");

        assert!(e.is_satisfied(&cards!["As Ah"]));
        assert!(e.is_satisfied(&cards!["Ks Kh"]));
    }

    #[test]
    fn test_and() {
        let e: Checker<2> = c("A:K");

        assert!(e.is_satisfied(&cards!["As Kh"]));
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
