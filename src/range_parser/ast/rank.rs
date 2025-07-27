use super::{Error, LocInfo, ResultE};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Const {
    R2 = 0,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    RT,
    RJ,
    RQ,
    RK,
    RA,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Var {
    RB,
    RE,
    RF,
    RG,
    RI,
    RL,
    RM,
    RN,
    RO,
    RP,
    RR,
    RU,
    RV,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Rank {
    Const(Const),
    Var(Var),
    Any,
}

impl Rank {
    pub(crate) fn from_token(c: char, loc: LocInfo) -> ResultE<'static, Self> {
        match c {
            '*' => Ok(Self::Any),

            '2' => Ok(Self::Const(Const::R2)),
            '3' => Ok(Self::Const(Const::R3)),
            '4' => Ok(Self::Const(Const::R4)),
            '5' => Ok(Self::Const(Const::R5)),
            '6' => Ok(Self::Const(Const::R6)),
            '7' => Ok(Self::Const(Const::R7)),
            '8' => Ok(Self::Const(Const::R8)),
            '9' => Ok(Self::Const(Const::R9)),

            'T' | 't' => Ok(Self::Const(Const::RT)),
            'J' | 'j' => Ok(Self::Const(Const::RJ)),
            'Q' | 'q' => Ok(Self::Const(Const::RQ)),
            'K' | 'k' => Ok(Self::Const(Const::RK)),
            'A' | 'a' => Ok(Self::Const(Const::RA)),

            'B' | 'b' => Ok(Self::Var(Var::RB)),
            'E' | 'e' => Ok(Self::Var(Var::RE)),
            'F' | 'f' => Ok(Self::Var(Var::RF)),
            'G' | 'g' => Ok(Self::Var(Var::RG)),
            'I' | 'i' => Ok(Self::Var(Var::RI)),
            'L' | 'l' => Ok(Self::Var(Var::RL)),
            'M' | 'm' => Ok(Self::Var(Var::RM)),
            'N' | 'n' => Ok(Self::Var(Var::RN)),
            'O' | 'o' => Ok(Self::Var(Var::RO)),
            'P' | 'p' => Ok(Self::Var(Var::RP)),
            'R' | 'r' => Ok(Self::Var(Var::RR)),
            'U' | 'u' => Ok(Self::Var(Var::RU)),
            'V' | 'v' => Ok(Self::Var(Var::RV)),

            _ => Err(Error::InvalidRank(loc).into()),
        }
    }
}

#[cfg_attr(coverage_nightly, coverage(off))]
#[cfg(test)]
mod tests {
    use super::{
        super::{
            super::tests::parse_card as p, RangeCard as C, Rank,
            RankConst as RC, RankVar as RV,
        },
        *,
    };

    impl quickcheck::Arbitrary for Const {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            *g.choose(&[
                Self::R2,
                Self::R3,
                Self::R4,
                Self::R5,
                Self::R6,
                Self::R7,
                Self::R8,
                Self::R9,
                Self::RT,
                Self::RJ,
                Self::RQ,
                Self::RK,
                Self::RA,
            ])
            .unwrap()
        }
    }

    impl Const {
        pub fn to_src(&self) -> String {
            match self {
                Self::R2 => '2',
                Self::R3 => '3',
                Self::R4 => '4',
                Self::R5 => '5',
                Self::R6 => '6',
                Self::R7 => '7',
                Self::R8 => '8',
                Self::R9 => '9',
                Self::RT => 'T',
                Self::RJ => 'J',
                Self::RQ => 'Q',
                Self::RK => 'K',
                Self::RA => 'A',
            }
            .to_string()
        }
    }

    impl quickcheck::Arbitrary for Var {
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

    impl Var {
        pub fn to_src(&self) -> String {
            match self {
                Self::RB => 'B',
                Self::RE => 'E',
                Self::RF => 'F',
                Self::RG => 'G',
                Self::RI => 'I',
                Self::RL => 'L',
                Self::RM => 'M',
                Self::RN => 'N',
                Self::RO => 'O',
                Self::RP => 'P',
                Self::RR => 'R',
                Self::RU => 'U',
                Self::RV => 'V',
            }
            .to_string()
        }
    }

    #[test]
    fn test_rank_const_as_isize() {
        assert_eq!(RC::R2 as isize, 0);
        assert_eq!(RC::R3 as isize, 1);
        assert_eq!(RC::R4 as isize, 2);
        assert_eq!(RC::R5 as isize, 3);
        assert_eq!(RC::R6 as isize, 4);
        assert_eq!(RC::R7 as isize, 5);
        assert_eq!(RC::R8 as isize, 6);
        assert_eq!(RC::R9 as isize, 7);
        assert_eq!(RC::RT as isize, 8);
        assert_eq!(RC::RJ as isize, 9);
        assert_eq!(RC::RQ as isize, 10);
        assert_eq!(RC::RK as isize, 11);
        assert_eq!(RC::RA as isize, 12);
    }

    #[test]
    fn test_card_rank_const() {
        assert_eq!(p("2"), Ok(C::CA(RC::R2)));
        assert_eq!(p("3"), Ok(C::CA(RC::R3)));
        assert_eq!(p("4"), Ok(C::CA(RC::R4)));
        assert_eq!(p("5"), Ok(C::CA(RC::R5)));
        assert_eq!(p("6"), Ok(C::CA(RC::R6)));
        assert_eq!(p("7"), Ok(C::CA(RC::R7)));
        assert_eq!(p("8"), Ok(C::CA(RC::R8)));
        assert_eq!(p("9"), Ok(C::CA(RC::R9)));

        assert_eq!(p("T"), Ok(C::CA(RC::RT)));
        assert_eq!(p("J"), Ok(C::CA(RC::RJ)));
        assert_eq!(p("Q"), Ok(C::CA(RC::RQ)));
        assert_eq!(p("K"), Ok(C::CA(RC::RK)));
        assert_eq!(p("A"), Ok(C::CA(RC::RA)));

        assert_eq!(p("t"), Ok(C::CA(RC::RT)));
        assert_eq!(p("j"), Ok(C::CA(RC::RJ)));
        assert_eq!(p("q"), Ok(C::CA(RC::RQ)));
        assert_eq!(p("k"), Ok(C::CA(RC::RK)));
        assert_eq!(p("a"), Ok(C::CA(RC::RA)));
    }

    #[test]
    fn test_card_rank_var() {
        assert_eq!(p("B"), Ok(C::VA(RV::RB)));
        assert_eq!(p("E"), Ok(C::VA(RV::RE)));
        assert_eq!(p("F"), Ok(C::VA(RV::RF)));
        assert_eq!(p("G"), Ok(C::VA(RV::RG)));
        assert_eq!(p("I"), Ok(C::VA(RV::RI)));
        assert_eq!(p("L"), Ok(C::VA(RV::RL)));
        assert_eq!(p("M"), Ok(C::VA(RV::RM)));
        assert_eq!(p("N"), Ok(C::VA(RV::RN)));
        assert_eq!(p("O"), Ok(C::VA(RV::RO)));
        assert_eq!(p("P"), Ok(C::VA(RV::RP)));
        assert_eq!(p("R"), Ok(C::VA(RV::RR)));
        assert_eq!(p("U"), Ok(C::VA(RV::RU)));
        assert_eq!(p("V"), Ok(C::VA(RV::RV)));
    }

    #[test]
    fn test_card_rank_var_lower() {
        assert_eq!(p("b"), Ok(C::VA(RV::RB)));
        assert_eq!(p("e"), Ok(C::VA(RV::RE)));
        assert_eq!(p("f"), Ok(C::VA(RV::RF)));
        assert_eq!(p("g"), Ok(C::VA(RV::RG)));
        assert_eq!(p("i"), Ok(C::VA(RV::RI)));
        assert_eq!(p("l"), Ok(C::VA(RV::RL)));
        assert_eq!(p("m"), Ok(C::VA(RV::RM)));
        assert_eq!(p("n"), Ok(C::VA(RV::RN)));
        assert_eq!(p("o"), Ok(C::VA(RV::RO)));
        assert_eq!(p("p"), Ok(C::VA(RV::RP)));
        assert_eq!(p("r"), Ok(C::VA(RV::RR)));
        assert_eq!(p("u"), Ok(C::VA(RV::RU)));
        assert_eq!(p("v"), Ok(C::VA(RV::RV)));
    }

    #[test]
    fn test_from_token() {
        // guaranteed unreachable by lexer
        assert![Rank::from_token('?', (0, 1)).is_err()];
    }
}
