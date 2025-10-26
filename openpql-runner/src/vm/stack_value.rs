use super::*;

#[cfg(test)]
#[macro_export]
macro_rules! sval {
    (@ref $val:expr) => {
        VmStackValue::Ref($val)
    };
    (@long $val:expr) => {
        VmStackValue::Long($val)
    };
    (@count $val:expr) => {
        VmStackValue::Count($val)
    };
    (@frac $a:expr, $b:expr) => {
        VmStackValue::Frac(PQLFraction::new($a, $b))
    };
    (@float $val:expr) => {
        VmStackValue::Double($val)
    };
    (@player $val:expr) => {
        VmStackValue::Player(($val as usize).into())
    };
    (@street preflop) => {
        VmStackValue::Street(PQLStreet::Preflop)
    };
    (@street flop) => {
        VmStackValue::Street(PQLStreet::Flop)
    };
    (@street turn) => {
        VmStackValue::Street(PQLStreet::Turn)
    };
    (@street river) => {
        VmStackValue::Street(PQLStreet::River)
    };
    (@card $val:expr) => {
        VmStackValue::Card(card!($val))
    };
    (@bool $val:expr) => {
        VmStackValue::Bool($val)
    };
    (@rank $val:expr) => {
        VmStackValue::Rank($val)
    };
    (@ranks $val:expr) => {
        VmStackValue::Ranks(r16!($val))
    };
    (@flopcat $val:expr) => {
        VmStackValue::FlopCategory($val)
    };
    (@handtype $val:expr) => {
        VmStackValue::HandType($val)
    };
    (@rating $val:expr) => {
        VmStackValue::Rating(mk_rating($val))
    };
}

/// `StackValue`
/// thin values that implement `Copy`
/// the size is 128 bits
#[derive(
    Clone, Debug, Copy, PartialEq, derive_more::From, Display, TryInto,
)]
pub enum VmStackValue {
    Ref(HeapIdx),
    Bool(PQLBoolean),
    Count(PQLCardCount),
    Long(PQLLong),
    Frac(PQLFraction),
    Double(PQLDouble),
    Card(PQLCard),
    Rank(PQLRank),
    Ranks(PQLRankSet),
    Street(PQLStreet),
    #[display("{_0}")]
    FlopCategory(PQLFlopHandCategory),
    HandType(PQLHandType),
    #[display("{_0:?}")]
    Rating(PQLHiRating),
    Player(PQLPlayer),
}

impl VmStackValue {
    pub(crate) const fn is_num(&self) -> bool {
        matches!(
            self,
            Self::Count(_) | Self::Long(_) | Self::Frac(_) | Self::Double(_)
        )
    }
}

impl TryFrom<VmStackValue> for PQLNumeric {
    type Error = InternalError;

    fn try_from(value: VmStackValue) -> Result<Self, Self::Error> {
        match value {
            VmStackValue::Count(v) => Ok(Self::Count(v)),
            VmStackValue::Long(v) => Ok(Self::Long(v)),
            VmStackValue::Double(v) => Ok(Self::Double(v)),
            VmStackValue::Frac(v) => Ok(Self::Frac(v)),
            _ => Err(InternalError::NonNumericStackValue),
        }
    }
}

impl From<VmStackValue> for PQLType {
    fn from(value: VmStackValue) -> Self {
        match value {
            VmStackValue::Ref(_) => Self::STRING,
            VmStackValue::Count(_) => Self::CARDCOUNT,
            VmStackValue::Long(_) => Self::LONG,
            VmStackValue::Double(_) => Self::DOUBLE,
            VmStackValue::Frac(_) => Self::FRACTION,
            VmStackValue::Bool(_) => Self::BOOLEAN,
            VmStackValue::Player(_) => Self::PLAYER,
            VmStackValue::Street(_) => Self::STREET,
            VmStackValue::Card(_) => Self::CARD,
            VmStackValue::Rank(_) => Self::RANK,
            VmStackValue::Ranks(_) => Self::RANKSET,
            VmStackValue::HandType(_) => Self::HANDTYPE,
            VmStackValue::Rating(_) => Self::HIRATING,
            VmStackValue::FlopCategory(_) => Self::FLOPHANDCATEGORY,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_size() {
        const MAX_BITS: usize = 128;

        assert!(mem::size_of::<VmStackValue>() <= MAX_BITS / 8);
    }

    #[test]
    fn test_to_type() {
        fn assert_type(lhs: VmStackValue, rhs: PQLType) {
            assert_eq!(PQLType::from(lhs), rhs);
        }

        assert_type(sval!(@ref 1), PQLType::STRING); // Not used
        assert_type(sval!(@bool true), PQLType::BOOLEAN);
        assert_type(sval!(@count 1), PQLType::CARDCOUNT);
        assert_type(sval!(@long 1), PQLType::LONG);
        assert_type(sval!(@frac 0, 1), PQLType::FRACTION);
        assert_type(sval!(@float 1.0), PQLType::DOUBLE);
        assert_type(sval!(@card "As"), PQLType::CARD);
        assert_type(sval!(@rank PQLRank::RA), PQLType::RANK);
        assert_type(sval!(@ranks "AK"), PQLType::RANKSET);
        assert_type(sval!(@street river), PQLType::STREET);
        assert_type(
            sval!(@flopcat PQLFlopHandCategory::Nothing),
            PQLType::FLOPHANDCATEGORY,
        );
        assert_type(sval!(@handtype PQLHandType::Trips), PQLType::HANDTYPE);
        assert_type(sval!(@rating "2s2h2cAsAh"), PQLType::HIRATING);
        assert_type(sval!(@player 1), PQLType::PLAYER);
    }
}
