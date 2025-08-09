use super::{
    Rank, RankConst as RankC, RankVar as RankV, Suit, SuitConst as SuitC,
    SuitVar as SuitV,
};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum RangeCard {
    CC(RankC, SuitC),
    CV(RankC, SuitV),
    CA(RankC),
    VC(RankV, SuitC),
    VV(RankV, SuitV),
    VA(RankV),
    AC(SuitC),
    AV(SuitV),
    AA,
}

impl From<(Rank, Suit)> for RangeCard {
    fn from(t: (Rank, Suit)) -> Self {
        match t {
            (Rank::Const(r), Suit::Const(s)) => Self::CC(r, s),
            (Rank::Const(r), Suit::Var(s)) => Self::CV(r, s),
            (Rank::Var(r), Suit::Const(s)) => Self::VC(r, s),
            (Rank::Var(r), Suit::Var(s)) => Self::VV(r, s),
            (Rank::Any, Suit::Const(s)) => Self::AC(s),
            (Rank::Any, Suit::Var(s)) => Self::AV(s),
        }
    }
}

impl From<Rank> for RangeCard {
    fn from(r: Rank) -> Self {
        match r {
            Rank::Const(c) => Self::CA(c),
            Rank::Var(v) => Self::VA(v),
            Rank::Any => Self::AA,
        }
    }
}

impl From<Suit> for RangeCard {
    fn from(s: Suit) -> Self {
        match s {
            Suit::Const(c) => Self::AC(c),
            Suit::Var(v) => Self::AV(v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::super::tests::parse_card as p, RangeCard as C, RankC as RC,
        RankV as RV, SuitC as SC, SuitV as SV, *,
    };

    impl quickcheck::Arbitrary for RangeCard {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            match *g.choose(&[0u8, 1, 2, 3, 4, 5, 6, 7, 8]).unwrap() {
                0 => Self::CC(RC::arbitrary(g), SC::arbitrary(g)),
                1 => Self::CV(RC::arbitrary(g), SV::arbitrary(g)),
                2 => Self::CA(RC::arbitrary(g)),
                3 => Self::VC(RV::arbitrary(g), SC::arbitrary(g)),
                4 => Self::VV(RV::arbitrary(g), SV::arbitrary(g)),
                5 => Self::VA(RV::arbitrary(g)),
                6 => Self::AC(SC::arbitrary(g)),
                7 => Self::AV(SV::arbitrary(g)),
                8.. => Self::AA,
            }
        }
    }

    impl RangeCard {
        pub fn to_src(&self) -> String {
            match self {
                Self::CC(r, s) => format!("{}{}", r.to_src(), s.to_src()),
                Self::CV(r, s) => format!("{}{}", r.to_src(), s.to_src()),
                Self::CA(r) => r.to_src(),
                Self::VC(r, s) => format!("{}{}", r.to_src(), s.to_src()),
                Self::VV(r, s) => format!("{}{}", r.to_src(), s.to_src()),
                Self::VA(r) => r.to_src(),
                Self::AC(s) => s.to_src(),
                Self::AV(s) => s.to_src(),
                Self::AA => "*".into(),
            }
        }
    }

    #[test]
    fn test_card_cc() {
        assert_eq!(p("As"), Ok(C::CC(RC::RA, SC::S)));
    }

    #[test]
    fn test_card_cv() {
        assert_eq!(p("Aw"), Ok(C::CV(RC::RA, SV::W)));
    }

    #[test]
    fn test_card_ca() {
        assert_eq!(p("A"), Ok(C::CA(RC::RA)));
    }

    #[test]
    fn test_card_vc() {
        assert_eq!(p("Bs"), Ok(C::VC(RV::RB, SC::S)));
    }

    #[test]
    fn test_card_vv() {
        assert_eq!(p("Bw"), Ok(C::VV(RV::RB, SV::W)));
    }

    #[test]
    fn test_card_va() {
        assert_eq!(p("B"), Ok(C::VA(RV::RB)));
    }

    #[test]
    fn test_card_ac() {
        assert_eq!(p("s"), Ok(C::AC(SC::S)));
        assert_eq!(p("*s"), Ok(C::AC(SC::S)));
    }

    #[test]
    fn test_card_av() {
        assert_eq!(p("w"), Ok(C::AV(SV::W)));
        assert_eq!(p("*w"), Ok(C::AV(SV::W)));
    }

    #[test]
    fn test_card_aa() {
        assert_eq!(p("*"), Ok(C::AA));
    }
}
