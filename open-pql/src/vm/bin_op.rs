use VmBinOpArith::{Add, Div, Mul, Sub};
use VmBinOpCmp::{Ge, Gt, Le, Lt};

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, From)]
pub enum VmBinOp {
    Arith(VmBinOpArith),
    Eq,
    Cmp(VmBinOpCmp),
    MaxOf,
    MinOf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmBinOpCmp {
    Ge,
    Gt,
    Le,
    Lt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmBinOpArith {
    Add,
    Sub,
    Mul,
    Div,
}

impl VmBinOp {
    fn exec(
        self,
        l: VmStackValue,
        r: VmStackValue,
    ) -> Result<VmStackValue, RuntimeError> {
        match self {
            Self::Arith(Add) => l.try_add(r),
            Self::Arith(Sub) => l.try_sub(r),
            Self::Arith(Mul) => l.try_mul(r),
            Self::Arith(Div) => l.try_div(r),
            Self::Eq => Ok((l == r).into()),
            Self::Cmp(Ge) => Ok((l >= r).into()),
            Self::Cmp(Gt) => Ok((l > r).into()),
            Self::Cmp(Le) => Ok((l <= r).into()),
            Self::Cmp(Lt) => Ok((l < r).into()),
            Self::MaxOf => Ok(if l >= r { l } else { r }),
            Self::MinOf => Ok(if l <= r { l } else { r }),
        }
    }

    fn type_checking_exec(
        self,
        l: VmStackValue,
        r: VmStackValue,
    ) -> Result<VmStackValue, RuntimeError> {
        use VmStackValue::{
            Bool, Card, FlopCategory, HandType, Num, Player, Rank, Ranks,
            Rating, Street,
        };

        match self {
            Self::Arith(_) => self.exec(l, r),

            Self::Eq => match (l, r) {
                (Num(_), Num(_))
                | (Bool(_), Bool(_))
                | (Player(_), Player(_))
                | (Street(_), Street(_))
                | (Card(_), Card(_))
                | (Rank(_), Rank(_))
                | (Ranks(_), Ranks(_))
                | (FlopCategory(_), FlopCategory(_))
                | (HandType(_), HandType(_))
                | (Rating(_), Rating(_)) => self.exec(l, r),
                _ => Err(RuntimeError::CmpFailed),
            },

            Self::Cmp(_) | Self::MaxOf | Self::MinOf => match (l, r) {
                (Num(_), Num(_))
                | (Rank(_), Rank(_))
                | (FlopCategory(_), FlopCategory(_))
                | (HandType(_), HandType(_))
                | (Rating(_), Rating(_)) => self.exec(l, r),
                _ => Err(RuntimeError::CmpFailed),
            },
        }
    }

    pub(super) fn execute(
        self,
        _buffer: &VmBuffer,
        _store: &VmStore,
        stack: &mut VmStack,
    ) -> Result<(), PQLError> {
        let l = stack.pop()?;
        let r = stack.pop()?;

        stack.push(self.exec(l, r)?);

        Ok(())
    }

    pub(super) fn check_type(
        self,
        loc: LocInfo,
        lhs: PQLType,
        rhs: PQLType,
    ) -> Result<PQLType, PQLError> {
        fn nonzero(t: PQLType) -> Result<VmStackValue, RuntimeError> {
            match t {
                PQLType::PlayerCount
                | PQLType::CardCount
                | PQLType::Integer
                | PQLType::Long => {
                    Ok(VmStackValue::Num(VmStackValueNum::Long(1)))
                }

                PQLType::Double
                | PQLType::Equity
                | PQLType::Fraction
                | PQLType::Numeric => {
                    Ok(VmStackValue::Num(VmStackValueNum::Double(1.0)))
                }

                _ => VmStackValue::default_of(t),
            }
        }

        self.type_checking_exec(VmStackValue::default_of(lhs)?, nonzero(rhs)?)
            .map_or_else(
                |_| {
                    Err((
                        loc,
                        TypeError::BinOpError(self.to_symbol(), lhs, rhs),
                    )
                        .into())
                },
                |v| Ok(v.into()),
            )
    }

    fn to_symbol(self) -> String {
        match self {
            Self::Arith(Add) => "+",
            Self::Arith(Sub) => "-",
            Self::Arith(Mul) => "*",
            Self::Arith(Div) => "/",
            Self::Eq => "=",
            Self::Cmp(Ge) => ">=",
            Self::Cmp(Gt) => ">",
            Self::Cmp(Le) => "<=",
            Self::Cmp(Lt) => "<",
            Self::MaxOf => "<max>",
            Self::MinOf => "<min>",
        }
        .into()
    }
}

impl From<ast::BinOp> for VmBinOp {
    fn from(op: ast::BinOp) -> Self {
        match op {
            ast::BinOp::Add => Self::Arith(Add),
            ast::BinOp::Sub => Self::Arith(Sub),
            ast::BinOp::Mul => Self::Arith(Mul),
            ast::BinOp::Div => Self::Arith(Div),
            ast::BinOp::Eq => Self::Eq,
            ast::BinOp::Ge => Self::Cmp(Ge),
            ast::BinOp::Gt => Self::Cmp(Gt),
            ast::BinOp::Le => Self::Cmp(Le),
            ast::BinOp::Lt => Self::Cmp(Lt),
        }
    }
}

impl From<VmBinOpArith> for VmInstruction {
    fn from(op: VmBinOpArith) -> Self {
        Self::BinOp(VmBinOp::Arith(op))
    }
}

impl From<VmBinOpCmp> for VmInstruction {
    fn from(op: VmBinOpCmp) -> Self {
        Self::BinOp(VmBinOp::Cmp(op))
    }
}

#[cfg(test)]
mod tests {
    use PQLType::*;

    use super::*;

    #[test]
    fn test_from_ast() {
        assert_eq!(VmBinOp::Arith(Add), ast::BinOp::Add.into());
        assert_eq!(VmBinOp::Arith(Sub), ast::BinOp::Sub.into());
        assert_eq!(VmBinOp::Arith(Mul), ast::BinOp::Mul.into());
        assert_eq!(VmBinOp::Arith(Div), ast::BinOp::Div.into());
        assert_eq!(VmBinOp::Eq, ast::BinOp::Eq.into());
        assert_eq!(VmBinOp::Cmp(Ge), ast::BinOp::Ge.into());
        assert_eq!(VmBinOp::Cmp(Gt), ast::BinOp::Gt.into());
        assert_eq!(VmBinOp::Cmp(Le), ast::BinOp::Le.into());
        assert_eq!(VmBinOp::Cmp(Lt), ast::BinOp::Lt.into());
    }

    #[test]
    fn test_to_sym() {
        assert_eq!("+", VmBinOp::Arith(Add).to_symbol());
        assert_eq!("-", VmBinOp::Arith(Sub).to_symbol());
        assert_eq!("*", VmBinOp::Arith(Mul).to_symbol());
        assert_eq!("/", VmBinOp::Arith(Div).to_symbol());
        assert_eq!("=", VmBinOp::Eq.to_symbol());
        assert_eq!(">=", VmBinOp::Cmp(Ge).to_symbol());
        assert_eq!(">", VmBinOp::Cmp(Gt).to_symbol());
        assert_eq!("<=", VmBinOp::Cmp(Le).to_symbol());
        assert_eq!("<", VmBinOp::Cmp(Lt).to_symbol());
        assert_eq!("<max>", VmBinOp::MaxOf.to_symbol());
        assert_eq!("<min>", VmBinOp::MinOf.to_symbol());
    }

    fn exec<A1, A2, A3>(op: VmBinOp, l: A1, r: A2) -> A3
    where
        VmStackValue: From<A1> + From<A2>,
        A3: TryFrom<VmStackValue>,
    {
        let mut stack = VmStack::default();

        stack.push(VmStackValue::from(r));
        stack.push(VmStackValue::from(l));

        op.execute(&VmBuffer::default(), &VmStore::default(), &mut stack)
            .unwrap();

        stack.downcast_pop().unwrap()
    }

    #[allow(clippy::float_cmp)]
    #[test]
    fn test_arith() {
        assert_eq!(
            10.0,
            exec::<PQLLong, _, PQLDouble>(VmBinOp::Arith(Add), 5, 5.0)
        );

        assert_eq!(
            0.0,
            exec::<PQLLong, _, PQLDouble>(VmBinOp::Arith(Sub), 5, 5.0)
        );

        assert_eq!(
            25.0,
            exec::<PQLLong, _, PQLDouble>(VmBinOp::Arith(Mul), 5, 5.0)
        );

        assert_eq!(
            1.0,
            exec::<PQLLong, _, PQLDouble>(VmBinOp::Arith(Div), 5, 5.0)
        );
    }

    #[allow(clippy::float_cmp)]
    #[test]
    fn test_cmp() {
        assert!(exec::<PQLLong, _, PQLBoolean>(VmBinOp::Eq, 5, 5.0));
        assert!(exec::<PQLLong, _, PQLBoolean>(VmBinOp::Cmp(Ge), 5, 5.0));
        assert!(!exec::<PQLLong, _, PQLBoolean>(VmBinOp::Cmp(Gt), 5, 5.0));
        assert!(exec::<PQLLong, _, PQLBoolean>(VmBinOp::Cmp(Le), 5, 5.0));
        assert!(!exec::<PQLLong, _, PQLBoolean>(VmBinOp::Cmp(Lt), 5, 5.0));

        assert_eq!(
            5,
            exec::<PQLLong, PQLDouble, PQLLong>(VmBinOp::MaxOf, 5, 5.0),
            "should return the first argument"
        );
        assert_eq!(
            5.0,
            exec::<PQLDouble, PQLLong, PQLDouble>(VmBinOp::MaxOf, 5.0, 5),
            "should return the first argument"
        );

        assert_eq!(
            5,
            exec::<PQLLong, PQLDouble, PQLLong>(VmBinOp::MinOf, 5, 5.0),
            "should return the first argument"
        );
        assert_eq!(
            5.0,
            exec::<PQLDouble, PQLLong, PQLDouble>(VmBinOp::MinOf, 5.0, 5),
            "should return the first argument"
        );

        assert_eq!(1, exec::<_, PQLLong, PQLLong>(VmBinOp::MinOf, 5.0, 1));
        assert_eq!(5, exec::<_, PQLLong, PQLLong>(VmBinOp::MaxOf, 1.0, 5));
    }

    fn assert_check(expect_ok: bool, op: VmBinOp, l: PQLType, r: PQLType) {
        let loc = LocInfo::default();

        let res = op.check_type(loc, l, r);
        if expect_ok {
            assert!(res.is_ok(), "{l} {op:?} {r}; got {res:?}");
        } else {
            assert!(res.is_err(), "{l} {op:?} {r}; got {res:?}");
        }
    }

    const fn is_thin_type(t: PQLType) -> bool {
        // TODO: fix this
        !matches!(t, BoardRange | Range | String | HandRanking | LoRating)
    }

    #[quickcheck]
    fn test_check_type_arith(l: PQLType, r: PQLType) {
        let ok = l.is_num() && r.is_num();

        for op in [
            VmBinOp::Arith(Add),
            VmBinOp::Arith(Sub),
            VmBinOp::Arith(Mul),
            VmBinOp::Arith(Div),
        ] {
            assert_check(ok, op, l, r);
        }
    }

    #[quickcheck]
    fn test_check_type_eq(l: PQLType, r: PQLType) -> TestResult {
        if !(is_thin_type(l) && is_thin_type(r)) {
            return TestResult::discard();
        }

        let ok = l.is_num() && r.is_num() || l == r;

        assert_check(ok, VmBinOp::Eq, l, r);

        TestResult::passed()
    }

    const fn no_ord(t: PQLType) -> bool {
        matches!(t, String | Boolean | Card | Player | RankSet | Street)
    }

    #[quickcheck]
    fn test_check_type_cmp(l: PQLType, r: PQLType) -> TestResult {
        if !(is_thin_type(l) && is_thin_type(r)) {
            return TestResult::discard();
        }

        let ok = l.is_num() && r.is_num() || (l == r && !no_ord(l));

        for op in [
            VmBinOp::Cmp(Ge),
            VmBinOp::Cmp(Gt),
            VmBinOp::Cmp(Le),
            VmBinOp::Cmp(Lt),
            VmBinOp::MaxOf,
            VmBinOp::MinOf,
        ] {
            assert_check(ok, op, l, r);
        }

        TestResult::passed()
    }

    #[test]
    fn test_exec_error() {
        let buffer = VmBuffer::default();
        let store = VmStore::default();
        let mut stack = VmStack::default();

        assert!(VmBinOp::MaxOf.execute(&buffer, &store, &mut stack).is_err());

        stack.push(VmStackValue::default_of(Long).unwrap());
        assert!(VmBinOp::MaxOf.execute(&buffer, &store, &mut stack).is_err());

        stack.push(VmStackValue::default_of(Long).unwrap());
        stack.push(VmStackValue::default_of(Card).unwrap());
        assert!(
            VmBinOp::Arith(Add)
                .execute(&buffer, &store, &mut stack)
                .is_err()
        );
    }
}
