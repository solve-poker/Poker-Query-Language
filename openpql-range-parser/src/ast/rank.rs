use super::{Display, Error, LocInfo, ResultE};

pub type RankConst = openpql_prelude::Rank;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Display)]
pub enum RankVar {
    #[display("B")]
    RB,
    #[display("E")]
    RE,
    #[display("F")]
    RF,
    #[display("G")]
    RG,
    #[display("I")]
    RI,
    #[display("L")]
    RL,
    #[display("M")]
    RM,
    #[display("N")]
    RN,
    #[display("O")]
    RO,
    #[display("P")]
    RP,
    #[display("R")]
    RR,
    #[display("U")]
    RU,
    #[display("V")]
    RV,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Display)]
pub enum CardRank {
    #[display("{_0}")]
    Const(RankConst),
    #[display("{_0}")]
    Var(RankVar),
    #[display("*")]
    Any,
}

impl CardRank {
    pub(crate) fn from_token(
        is_shortdeck: bool,
        c: char,
        loc: LocInfo,
    ) -> ResultE<'static, Self> {
        if is_shortdeck {
            match c {
                '2' | '3' | '4' | '5' => {
                    return Err(Error::InvalidRank(loc).into());
                }
                _ => (),
            }
        }

        match c {
            '*' => Ok(Self::Any),

            '2' => Ok(Self::Const(RankConst::R2)),
            '3' => Ok(Self::Const(RankConst::R3)),
            '4' => Ok(Self::Const(RankConst::R4)),
            '5' => Ok(Self::Const(RankConst::R5)),
            '6' => Ok(Self::Const(RankConst::R6)),
            '7' => Ok(Self::Const(RankConst::R7)),
            '8' => Ok(Self::Const(RankConst::R8)),
            '9' => Ok(Self::Const(RankConst::R9)),

            'T' | 't' => Ok(Self::Const(RankConst::RT)),
            'J' | 'j' => Ok(Self::Const(RankConst::RJ)),
            'Q' | 'q' => Ok(Self::Const(RankConst::RQ)),
            'K' | 'k' => Ok(Self::Const(RankConst::RK)),
            'A' | 'a' => Ok(Self::Const(RankConst::RA)),

            'B' | 'b' => Ok(Self::Var(RankVar::RB)),
            'E' | 'e' => Ok(Self::Var(RankVar::RE)),
            'F' | 'f' => Ok(Self::Var(RankVar::RF)),
            'G' | 'g' => Ok(Self::Var(RankVar::RG)),
            'I' | 'i' => Ok(Self::Var(RankVar::RI)),
            'L' | 'l' => Ok(Self::Var(RankVar::RL)),
            'M' | 'm' => Ok(Self::Var(RankVar::RM)),
            'N' | 'n' => Ok(Self::Var(RankVar::RN)),
            'O' | 'o' => Ok(Self::Var(RankVar::RO)),
            'P' | 'p' => Ok(Self::Var(RankVar::RP)),
            'R' | 'r' => Ok(Self::Var(RankVar::RR)),
            'U' | 'u' => Ok(Self::Var(RankVar::RU)),
            'V' | 'v' => Ok(Self::Var(RankVar::RV)),

            _ => Err(Error::InvalidRank(loc).into()),
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    impl Arbitrary for RankVar {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            *g.choose(&[
                Self::RB,
                Self::RE,
                Self::RF,
                Self::RG,
                Self::RI,
                Self::RL,
                Self::RM,
                Self::RN,
                Self::RO,
                Self::RP,
                Self::RR,
                Self::RU,
                Self::RV,
            ])
            .unwrap()
        }
    }

    #[test]
    fn test_card_rank_const() {
        assert_range_card("2", "2");
        assert_range_card("3", "3");
        assert_range_card("4", "4");
        assert_range_card("5", "5");
        assert_range_card("6", "6");
        assert_range_card("7", "7");
        assert_range_card("8", "8");
        assert_range_card("9", "9");

        assert_range_card("T", "T");
        assert_range_card("J", "J");
        assert_range_card("Q", "Q");
        assert_range_card("K", "K");
        assert_range_card("A", "A");

        assert_range_card("t", "T");
        assert_range_card("j", "J");
        assert_range_card("q", "Q");
        assert_range_card("k", "K");
        assert_range_card("a", "A");
    }

    #[test]
    fn test_card_rank_var() {
        assert_range_card("B", "B");
        assert_range_card("E", "E");
        assert_range_card("F", "F");
        assert_range_card("G", "G");
        assert_range_card("I", "I");
        assert_range_card("L", "L");
        assert_range_card("M", "M");
        assert_range_card("N", "N");
        assert_range_card("O", "O");
        assert_range_card("P", "P");
        assert_range_card("R", "R");
        assert_range_card("U", "U");
        assert_range_card("V", "V");
    }

    #[test]
    fn test_card_rank_var_lower() {
        assert_range_card("b", "B");
        assert_range_card("e", "E");
        assert_range_card("f", "F");
        assert_range_card("g", "G");
        assert_range_card("i", "I");
        assert_range_card("l", "L");
        assert_range_card("m", "M");
        assert_range_card("n", "N");
        assert_range_card("o", "O");
        assert_range_card("p", "P");
        assert_range_card("r", "R");
        assert_range_card("u", "U");
        assert_range_card("v", "V");
    }

    #[test]
    fn test_from_token() {
        // guaranteed unreachable by lexer
        assert![CardRank::from_token(false, '?', (0, 1)).is_err()];
    }
}
