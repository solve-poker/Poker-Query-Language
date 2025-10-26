use super::{
    CardRank, CardSuit, Display, From, RankConst, RankVar, SuitConst, SuitVar,
};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Display)]
pub enum RangeCard {
    #[display("{_0}{_1}")]
    CC(RankConst, SuitConst),
    #[display("{_0}{_1}")]
    CV(RankConst, SuitVar),
    #[display("{_0}")]
    CA(RankConst),
    #[display("{_0}{_1}")]
    VC(RankVar, SuitConst),
    #[display("{_0}{_1}")]
    VV(RankVar, SuitVar),
    #[display("{_0}")]
    VA(RankVar),
    #[display("{_0}")]
    AC(SuitConst),
    #[display("{_0}")]
    AV(SuitVar),
    #[display("*")]
    AA,
}

impl From<(CardRank, CardSuit)> for RangeCard {
    fn from(t: (CardRank, CardSuit)) -> Self {
        match t {
            (CardRank::Const(r), CardSuit::Const(s)) => Self::CC(r, s),
            (CardRank::Const(r), CardSuit::Var(s)) => Self::CV(r, s),
            (CardRank::Var(r), CardSuit::Const(s)) => Self::VC(r, s),
            (CardRank::Var(r), CardSuit::Var(s)) => Self::VV(r, s),
            (CardRank::Any, CardSuit::Const(s)) => Self::AC(s),
            (CardRank::Any, CardSuit::Var(s)) => Self::AV(s),
        }
    }
}

impl From<CardRank> for RangeCard {
    fn from(r: CardRank) -> Self {
        match r {
            CardRank::Const(c) => Self::CA(c),
            CardRank::Var(v) => Self::VA(v),
            CardRank::Any => Self::AA,
        }
    }
}

impl From<CardSuit> for RangeCard {
    fn from(s: CardSuit) -> Self {
        match s {
            CardSuit::Const(c) => Self::AC(c),
            CardSuit::Var(v) => Self::AV(v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    impl Arbitrary for RangeCard {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            match *g.choose(&[0, 1, 2, 3, 4, 5, 6, 7, 8]).unwrap() {
                0 => Self::CC(RankConst::arbitrary(g), SuitConst::arbitrary(g)),
                1 => Self::CV(RankConst::arbitrary(g), SuitVar::arbitrary(g)),
                2 => Self::CA(RankConst::arbitrary(g)),
                3 => Self::VC(RankVar::arbitrary(g), SuitConst::arbitrary(g)),
                4 => Self::VV(RankVar::arbitrary(g), SuitVar::arbitrary(g)),
                5 => Self::VA(RankVar::arbitrary(g)),
                6 => Self::AC(SuitConst::arbitrary(g)),
                7 => Self::AV(SuitVar::arbitrary(g)),
                _ => Self::AA,
            }
        }
    }

    #[test]
    fn test_card_cc() {
        assert_range_card("As", "As");
    }

    #[test]
    fn test_card_cv() {
        assert_range_card("Aw", "Aw");
    }

    #[test]
    fn test_card_ca() {
        assert_range_card("A", "A");
    }

    #[test]
    fn test_card_vc() {
        assert_range_card("Bs", "Bs");
    }

    #[test]
    fn test_card_vv() {
        assert_range_card("Bw", "Bw");
    }

    #[test]
    fn test_card_va() {
        assert_range_card("B", "B");
    }

    #[test]
    fn test_card_ac() {
        assert_range_card("s", "s");
        assert_range_card("*s", "s");
    }

    #[test]
    fn test_card_av() {
        assert_range_card("w", "w");
        assert_range_card("*w", "w");
    }

    #[test]
    fn test_card_aa() {
        assert_range_card("*", "*");
    }

    #[quickcheck]
    fn test_shortdeck(card: RangeCard) {
        let src = card.to_string();
        if src.chars().any(|c| ['2', '3', '4', '5'].contains(&c)) {
            assert!(parse_card_sd(&src).is_err());
        } else {
            assert_eq!(Ok(card), parse_card_sd(&src));
        }
    }

    fn assert_range_card_sd_err(s: &str, expected: Error) {
        assert_eq!(parse_card_sd(s), Err(expected.into()));
    }

    #[test]
    fn test_shortdeck_error() {
        assert_range_card_sd_err("2", Error::InvalidRank((0, 1)));
        assert_range_card_sd_err("3", Error::InvalidRank((0, 1)));
        assert_range_card_sd_err("4", Error::InvalidRank((0, 1)));
        assert_range_card_sd_err("5", Error::InvalidRank((0, 1)));
    }
}
