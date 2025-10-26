use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VmBinOpArith {
    Add,
    Sub,
    Mul,
    Div,
}

#[inline]
fn to_sval(v: PQLNumeric) -> VmStackValue {
    match v {
        PQLNumeric::Long(v) => v.into(),
        PQLNumeric::Double(v) => v.into(),
        _ => unreachable!(),
    }
}

impl VmBinOpArith {
    pub fn execute(self, ctx: &mut VmExecContext) -> Result<(), PQLErrorKind> {
        let rhs = ctx.stack.downcast_pop::<PQLNumeric>();
        let lhs = ctx.stack.downcast_pop::<PQLNumeric>();

        ctx.stack.push(to_sval(match self {
            Self::Add => lhs.try_add(rhs),
            Self::Sub => lhs.try_sub(rhs),
            Self::Mul => lhs.try_mul(rhs),
            Self::Div => lhs.try_div(rhs),
        }?));

        Ok(())
    }

    pub fn resolve_type(
        self,
        lhs_type: PQLType,
        rhs_type: PQLType,
    ) -> Result<PQLType, PQLErrorKind> {
        if !lhs_type.is_num() || !rhs_type.is_num() {
            return Err(PQLErrorKind::ArithmeticOperationUnsupported(
                lhs_type, rhs_type,
            ));
        }

        if self == Self::Div
            || lhs_type == PQLType::DOUBLE
            || rhs_type == PQLType::DOUBLE
            || lhs_type == PQLType::FRACTION
            || rhs_type == PQLType::FRACTION
        {
            Ok(PQLType::DOUBLE)
        } else {
            Ok(PQLType::LONG)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_arith<O, I, T>(vals: I, op: O, expected: VmStackValue)
    where
        VmBinOp: From<O>,
        I: IntoIterator<Item = T>,
        VmStackValue: From<T>,
    {
        let mut ctx = VmExecContext::default();
        for v in vals {
            ctx.stack.push(v.into());
        }

        let ins = VmInstruction::BinOp(op.into());
        ins.execute(&mut ctx).unwrap();

        assert_eq!(ctx.stack.pop().unwrap(), expected);
    }

    #[test]
    fn test_add() {
        assert_arith(
            [PQLLong::from(1), PQLLong::from(2)],
            VmBinOpArith::Add,
            sval!(@long 3),
        );
    }

    #[test]
    fn test_sub() {
        assert_arith(
            [PQLCardCount::from(1), PQLCardCount::from(2)],
            VmBinOpArith::Sub,
            sval!(@long -1),
        );
    }

    #[test]
    fn test_mul() {
        assert_arith(
            [PQLCardCount::from(1), PQLCardCount::from(2)],
            VmBinOpArith::Mul,
            sval!(@long 2),
        );
    }

    #[test]
    fn test_div() {
        assert_arith(
            [PQLFraction::new(1, 3), PQLFraction::new(1, 2)],
            VmBinOpArith::Div,
            sval!(@float 2.0/3.0),
        );
    }

    #[test]
    fn test_err() {
        let data = CompilerData::default();
        let mut ctx = VmExecContext::from(&data);

        let res = [
            VmInstruction::Push(sval!(@long PQLLong::MAX)),
            VmInstruction::Push(sval!(@long PQLLong::MAX)),
            VmInstruction::BinOp(VmBinOpArith::Add.into()),
        ]
        .iter()
        .try_for_each(|ins| ins.execute(&mut ctx));

        assert_eq!(res, Err(RuntimeError::AddOverflow.into()));
    }

    #[test]
    #[should_panic(expected = "")]
    fn test_internal() {
        to_sval(PQLNumeric::Count(1));
    }
}
