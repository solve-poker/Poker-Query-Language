use super::*;

macro_rules! union {
    ($single:ident) => {
        PQLType::$single.bits()
    };
    ($first:ident, $($rest:ident),+) => {
        PQLType::$first.bits() | union!($($rest),+)
    };
}

bitflags! {
  #[derive(Clone, Copy, Debug, PartialEq, Eq, Display)]
  pub struct PQLType: u32 {
    const BOARDRANGE = 1;
    const BOOLEAN = 1 << 1;
    const CARD = 1 << 2;
    const CARDCOUNT = 1 << 3;
    const DOUBLE = 1 << 4;
    const EQUITY = union!(DOUBLE);
    const FLOPHANDCATEGORY = 1 << 6;
    const FRACTION = 1 << 7;
    const HANDRANKING = 1 << 8;
    const HANDTYPE = 1 << 9;
    const HIRATING = 1 << 10;
    const INTEGER = union!(LONG);
    const LONG = 1 << 12;
    const LORATING = 1 << 13;
    const NUMERIC = union!(CARDCOUNT, LONG, DOUBLE, FRACTION);
    const PLAYER = 1 << 15;
    const PLAYERCOUNT = union!(CARDCOUNT);
    const RANGE = 1 << 17;
    const RANK = 1 << 18;
    const RANKSET = 1 << 19;
    const STREET = 1 << 20;
    const STRING = 1 << 21;
  }
}

impl PQLType {
    pub const fn is_num(self) -> bool {
        Self::NUMERIC.contains(self)
    }

    pub const fn is_concrete(self) -> bool {
        self.bits().is_power_of_two()
    }
}

impl From<ast::SelectorKind> for PQLType {
    fn from(sel: ast::SelectorKind) -> Self {
        match sel {
            ast::SelectorKind::Avg => Self::NUMERIC,
            ast::SelectorKind::Count => Self::BOOLEAN,
            ast::SelectorKind::Max | ast::SelectorKind::Min => {
                Self::NUMERIC
                    | Self::FLOPHANDCATEGORY
                    | Self::HANDTYPE
                    | Self::HIRATING
                    | Self::RANK
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_type_alias() {
        assert_eq!(PQLType::PLAYERCOUNT, PQLType::CARDCOUNT);
        assert_eq!(PQLType::INTEGER, PQLType::LONG);
        assert_eq!(PQLType::EQUITY, PQLType::DOUBLE);
    }

    #[test]
    fn test_type_num() {
        for tp in [
            PQLType::PLAYERCOUNT,
            PQLType::CARDCOUNT,
            PQLType::INTEGER,
            PQLType::LONG,
            PQLType::EQUITY,
            PQLType::DOUBLE,
            PQLType::FRACTION,
        ] {
            assert!(PQLType::NUMERIC.contains(tp));
        }
    }

    #[test]
    fn test_type_for_selector() {
        use ast::SelectorKind::*;
        fn assert_match(sel: ast::SelectorKind, kind: PQLType) {
            assert!(PQLType::from(sel).contains(kind));
        }
        fn assert_match_max_min(kind: PQLType) {
            assert_match(Max, kind);
            assert_match(Min, kind);
        }

        assert_match(Avg, PQLType::CARDCOUNT);
        assert_match(Avg, PQLType::LONG);
        assert_match(Avg, PQLType::FRACTION);
        assert_match(Avg, PQLType::DOUBLE);

        assert_match(Count, PQLType::BOOLEAN);

        assert_match_max_min(PQLType::CARDCOUNT);
        assert_match_max_min(PQLType::LONG);
        assert_match_max_min(PQLType::FRACTION);
        assert_match_max_min(PQLType::DOUBLE);
        assert_match_max_min(PQLType::FLOPHANDCATEGORY);
        assert_match_max_min(PQLType::HANDTYPE);
        assert_match_max_min(PQLType::HIRATING);
        assert_match_max_min(PQLType::RANK);
    }
}
