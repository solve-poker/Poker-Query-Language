use super::*;

#[derive(Debug, Clone, Copy, From, TryInto, PartialEq, PartialOrd, Display)]
pub enum VmStackValue {
    #[display("TODO")]
    Ref(VmStoreVarIdx),
    #[display("TODO")]
    Num(VmStackValueNum),
    #[display("TODO")]
    Bool(PQLBoolean),
    #[display("TODO")]
    Player(PQLPlayer),
    #[display("TODO")]
    Street(PQLStreet),
    #[display("TODO")]
    Card(PQLCard),
    #[display("TODO")]
    Rank(Option<Rank>),
    #[display("TODO")]
    Ranks(PQLRankSet),
    #[display("{_0}")]
    FlopCategory(PQLFlopHandCategory),
    #[display("{_0}")]
    HandType(PQLHandType),
    #[display("TODO")]
    Rating(PQLHiRating),
}

macro_rules! impl_arith {
    ($fn_name:ident, $err:expr, $op:tt) => {
        pub fn $fn_name(self, other: Self) -> Result<Self, RuntimeError> {
            match (self, other) {
                (Self::Num(l), Self::Num(r)) => Ok(Self::Num(l $op r)),
                _ => Err($err),
            }
        }
    };
}

impl VmStackValue {
    impl_arith!(try_add, RuntimeError::AddFailed, +);
    impl_arith!(try_sub, RuntimeError::AddFailed, -);
    impl_arith!(try_mul, RuntimeError::AddFailed, *);
    impl_arith!(try_div, RuntimeError::AddFailed, /);

    pub const DBL_ZERO: Self = Self::Num(VmStackValueNum::Double(0.0));
    pub const INT_ZERO: Self = Self::Num(VmStackValueNum::Long(0));
    pub const INT_ONE: Self = Self::Num(VmStackValueNum::Long(1));

    pub(crate) fn default_of(t: PQLType) -> Result<Self, RuntimeError> {
        match t {
            PQLType::Card => Ok(PQLCard::default().into()),
            PQLType::FlopHandCategory => {
                Ok(PQLFlopHandCategory::default().into())
            }
            PQLType::HandType => Ok(PQLHandType::default().into()),
            // TODO: fix this
            PQLType::HiRating | PQLType::LoRating => {
                Ok(PQLHiRating::default().into())
            }
            PQLType::Player => Ok(PQLPlayer::default().into()),
            PQLType::Rank => Ok(Some(PQLRank::default()).into()),
            PQLType::RankSet => Ok(PQLRankSet::default().into()),
            PQLType::Street => Ok(PQLStreet::default().into()),

            PQLType::Boolean => Ok(PQLBoolean::default().into()),

            PQLType::PlayerCount | PQLType::CardCount => {
                Ok(PQLCardCount::default().into())
            }
            PQLType::Integer | PQLType::Long => Ok(PQLLong::default().into()),
            PQLType::Fraction
            | PQLType::Numeric
            | PQLType::Equity
            | PQLType::Double => Ok(PQLDouble::default().into()),
            _ => Err(RuntimeError::DefaultNotDefined),
        }
    }

    pub fn min_of(t: PQLType, g: PQLGame) -> Result<Self, RuntimeError> {
        match t {
            // TODO: fix this
            PQLType::Fraction
            | PQLType::Numeric
            | PQLType::Double
            | PQLType::Equity => Ok(PQLDouble::MIN.into()),

            PQLType::Integer | PQLType::Long => Ok(PQLLong::MIN.into()),

            PQLType::CardCount | PQLType::PlayerCount => {
                Ok(PQLCardCount::MIN.into())
            }

            PQLType::FlopHandCategory => {
                Ok(PQLFlopHandCategory::from((FlopHandCategory::MIN, g)).into())
            }
            PQLType::HandType => {
                Ok(PQLHandType::from((HandType::MIN, g)).into())
            }
            PQLType::HiRating => Ok(PQLHiRating::MIN.into()),
            _ => Err(RuntimeError::MinMaxNotDefined),
        }
    }

    pub fn max_of(t: PQLType, g: PQLGame) -> Result<Self, RuntimeError> {
        match t {
            // TODO: fix this
            PQLType::Fraction
            | PQLType::Numeric
            | PQLType::Double
            | PQLType::Equity => Ok(PQLDouble::MAX.into()),

            PQLType::Integer | PQLType::Long => Ok(PQLLong::MAX.into()),

            PQLType::CardCount | PQLType::PlayerCount => {
                Ok(PQLCardCount::MAX.into())
            }
            PQLType::FlopHandCategory => {
                Ok(PQLFlopHandCategory::from((FlopHandCategory::MAX, g)).into())
            }
            PQLType::HandType => {
                Ok(PQLHandType::from((HandType::MAX, g)).into())
            }
            PQLType::HiRating => Ok(PQLHiRating::MAX.into()),
            _ => Err(RuntimeError::MinMaxNotDefined),
        }
    }
}

impl From<VmStackValue> for PQLType {
    fn from(v: VmStackValue) -> Self {
        match v {
            VmStackValue::Bool(_) => Self::Boolean,
            VmStackValue::Ref(_)
            | VmStackValue::Num(VmStackValueNum::Long(_)) => Self::Long,
            VmStackValue::Num(VmStackValueNum::CardCount(_)) => Self::CardCount,
            VmStackValue::Num(VmStackValueNum::Double(_)) => Self::Double,
            VmStackValue::Player(_) => Self::Player,
            VmStackValue::Street(_) => Self::String,
            VmStackValue::Card(_) => Self::Card,
            VmStackValue::Rank(_) => Self::Rank,
            VmStackValue::Ranks(_) => Self::RankSet,
            VmStackValue::FlopCategory(_) => Self::FlopHandCategory,
            VmStackValue::HandType(_) => Self::HandType,
            VmStackValue::Rating(_) => Self::HiRating,
        }
    }
}

impl From<PQLRank> for VmStackValue {
    fn from(r: PQLRank) -> Self {
        Self::Rank(Some(r))
    }
}

#[cfg_attr(coverage_nightly, coverage(off))]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    impl VmStackValue {
        pub const ONE: Self = Self::Num(VmStackValueNum::Long(1));

        pub fn strict_eq(&self, other: &Self) -> bool {
            use VmStackValue::*;

            match (self, other) {
                (Ref(l), Ref(r)) => l == r,
                (Bool(l), Bool(r)) => l == r,
                (Num(l), Num(r)) => l == r,
                (Player(l), Player(r)) => l == r,
                (Street(l), Street(r)) => l == r,
                (Card(l), Card(r)) => l == r,
                (Rank(l), Rank(r)) => l == r,
                (Ranks(l), Ranks(r)) => l == r,
                (FlopCategory(l), FlopCategory(r)) => l == r,
                (HandType(l), HandType(r)) => l == r,
                (Rating(l), Rating(r)) => l == r,
                _ => false,
            }
        }
    }

    impl Arbitrary for VmStackValue {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            match g.choose(&(0..=10).collect::<Vec<_>>()).unwrap() {
                0 => Self::Ref(VmStoreVarIdx::arbitrary(g)),
                1 => Self::Bool(PQLBoolean::arbitrary(g)),
                2 => Self::Num(VmStackValueNum::arbitrary(g)),
                3 => Self::Player(PQLPlayer::arbitrary(g)),
                4 => Self::Street(PQLStreet::arbitrary(g)),
                5 => Self::Card(PQLCard::arbitrary(g)),
                6 => Self::Rank(Some(PQLRank::arbitrary(g))),
                7 => Self::Ranks(PQLRankSet::arbitrary(g)),
                8 => Self::FlopCategory(PQLFlopHandCategory::arbitrary(g)),
                9 => Self::HandType(PQLHandType::arbitrary(g)),
                _ => Self::Rating(PQLHiRating::arbitrary(g)),
            }
        }
    }

    #[test]
    fn test_partial_eq() {
        let cs = cards!("As Ks");
        let card_a: VmStackValue = cs[0].into();
        let card_k: VmStackValue = cs[1].into();

        assert!(card_a == card_a);
        assert!(card_a != card_k);

        assert!(
            VmStackValue::Player(0.into()) != VmStackValue::Player(1.into())
        );

        assert!(
            VmStackValue::Street(PQLStreet::Flop)
                == VmStackValue::Street(PQLStreet::Flop)
        );
        assert!(
            VmStackValue::Rank(Some(Rank::R2))
                == VmStackValue::Rank(Some(Rank::R2))
        );
        assert!(
            VmStackValue::Ranks(r16!("AK")) == VmStackValue::Ranks(r16!("AK"))
        );

        assert!(
            VmStackValue::Rank(Some(Rank::R2))
                != VmStackValue::Street(PQLStreet::Flop)
        );
    }

    #[quickcheck]
    fn test_arith(nl: VmStackValueNum, nr: VmStackValueNum) {
        fn to_v(f: PQLDouble) -> VmStackValue {
            f.into()
        }
        let zero = VmStackValue::default_of(PQLType::Long).unwrap();

        let fl = nl.cast_dbl();
        let fr = nr.cast_dbl();

        let l: VmStackValue = nl.into();
        let r: VmStackValue = nr.into();

        assert_eq!(to_v(fl + fr), l.try_add(r).unwrap());
        assert_eq!(to_v(fl - fr), l.try_sub(r).unwrap());
        assert_eq!(to_v(fl * fr), l.try_mul(r).unwrap());

        if r != zero {
            assert_eq!(to_v(fl / fr), l.try_div(r).unwrap());
        }
    }

    #[test]
    fn test_arith_error() {
        let l = VmStackValue::default_of(PQLType::Card).unwrap();
        let r = VmStackValue::default_of(PQLType::Card).unwrap();

        assert!(l.try_add(r).is_err());
        assert!(l.try_sub(r).is_err());
        assert!(l.try_mul(r).is_err());
        assert!(l.try_div(r).is_err());
    }

    #[test]
    fn test_default_of() {
        fn assert_def_is(t: PQLType, v: VmStackValue) {
            assert!(VmStackValue::default_of(t).unwrap().strict_eq(&v));
        }
        assert_def_is(PQLType::PlayerCount, PQLCardCount::default().into());
        assert_def_is(PQLType::CardCount, PQLCardCount::default().into());

        assert_def_is(PQLType::Long, PQLLong::default().into());
        assert_def_is(PQLType::Integer, PQLLong::default().into());

        assert_def_is(PQLType::Double, PQLDouble::default().into());
        assert_def_is(PQLType::Equity, PQLDouble::default().into());

        //assert_def_is(PQLType::Fraction,         VmStackValue::Double(0.0));
        //assert_def_is(PQLType::Numeric,          VmStackValue::Double(0.0));

        //assert_def_is(PQLType::HandRanking,      VmStackValue::Double(0.0));
        //assert_def_is(PQLType::LoRating,         VmStackValue::Double(0.0));

        assert_def_is(PQLType::Boolean, bool::default().into());
        assert_def_is(PQLType::Card, Card::default().into());
        assert_def_is(
            PQLType::FlopHandCategory,
            PQLFlopHandCategory::default().into(),
        );
        assert_def_is(PQLType::HandType, PQLHandType::default().into());
        assert_def_is(PQLType::HiRating, PQLHiRating::default().into());
        assert_def_is(PQLType::Player, PQLPlayer::default().into());
        assert_def_is(PQLType::Rank, Some(PQLRank::default()).into());
        assert_def_is(PQLType::RankSet, PQLRankSet::default().into());
        assert_def_is(PQLType::Street, PQLStreet::default().into());

        assert!(VmStackValue::default_of(PQLType::Range).is_err());
        assert!(VmStackValue::default_of(PQLType::BoardRange).is_err());
        assert!(VmStackValue::default_of(PQLType::String).is_err());
    }

    #[test]
    fn test_min_of() {
        let g = PQLGame::default();

        assert_eq!(
            VmStackValue::min_of(PQLType::FlopHandCategory, g),
            Ok(PQLFlopHandCategory::min(g).into())
        );

        assert_eq!(
            VmStackValue::min_of(PQLType::HandType, g),
            Ok(PQLHandType::min(g).into())
        );

        assert_eq!(
            VmStackValue::min_of(PQLType::HiRating, g),
            Ok(PQLHiRating::MIN.into())
        );

        assert!(VmStackValue::min_of(PQLType::Card, g).is_err());
    }

    #[test]
    fn test_max_of() {
        let g = PQLGame::default();

        assert_eq!(
            VmStackValue::max_of(PQLType::FlopHandCategory, g),
            Ok(PQLFlopHandCategory::max(g).into())
        );

        assert_eq!(
            VmStackValue::max_of(PQLType::HandType, g),
            Ok(PQLHandType::max(g).into())
        );

        assert_eq!(
            VmStackValue::max_of(PQLType::HiRating, g),
            Ok(PQLHiRating::MAX.into())
        );

        assert!(VmStackValue::max_of(PQLType::Card, g).is_err());
    }

    #[quickcheck]
    fn test_to_type(v: VmStackValue) {
        let expected = match v {
            VmStackValue::Bool(_) => PQLType::Boolean,
            VmStackValue::Ref(_)
            | VmStackValue::Num(VmStackValueNum::Long(_)) => PQLType::Long,
            VmStackValue::Num(VmStackValueNum::CardCount(_)) => {
                PQLType::CardCount
            }
            VmStackValue::Num(VmStackValueNum::Double(_)) => PQLType::Double,
            VmStackValue::Player(_) => PQLType::Player,
            VmStackValue::Street(_) => PQLType::String,
            VmStackValue::Card(_) => PQLType::Card,
            VmStackValue::Rank(_) => PQLType::Rank,
            VmStackValue::Ranks(_) => PQLType::RankSet,
            VmStackValue::FlopCategory(_) => PQLType::FlopHandCategory,
            VmStackValue::HandType(_) => PQLType::HandType,
            VmStackValue::Rating(_) => PQLType::HiRating,
        };

        assert_eq!(expected, PQLType::from(v));
    }
}
