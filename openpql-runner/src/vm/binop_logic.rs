use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VmBinOpLogic {
    And,
    Or,
}

impl VmBinOpLogic {
    pub fn execute(self, ctx: &mut VmExecContext) -> Result<(), PQLErrorKind> {
        let rhs = ctx.stack.downcast_pop::<PQLBoolean>();
        let lhs = ctx.stack.downcast_pop::<PQLBoolean>();

        ctx.stack.push(
            match self {
                Self::And => lhs && rhs,
                Self::Or => lhs || rhs,
            }
            .into(),
        );

        Ok(())
    }

    pub fn resolve_type(
        self,
        lhs_type: PQLType,
        rhs_type: PQLType,
    ) -> Result<PQLType, PQLErrorKind> {
        if lhs_type == PQLType::BOOLEAN && rhs_type == PQLType::BOOLEAN {
            Ok(PQLType::BOOLEAN)
        } else {
            Err(PQLErrorKind::LogicalOperationUnsupported(
                lhs_type, rhs_type,
            ))
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    fn assert_logic<I, T>(vals: I, op: VmBinOpLogic, expected: bool)
    where
        I: IntoIterator<Item = T>,
        VmStackValue: From<T>,
    {
        let mut ctx = VmExecContext::default();
        for v in vals {
            ctx.stack.push(v.into());
        }

        let ins = VmInstruction::BinOp(op.into());
        ins.execute(&mut ctx).unwrap();

        assert_eq!(ctx.stack.pop().unwrap(), expected.into());
    }

    #[test]
    fn test_and() {
        assert_logic([true, true], VmBinOpLogic::And, true);
        assert_logic([true, false], VmBinOpLogic::And, false);
        assert_logic([false, true], VmBinOpLogic::And, false);
        assert_logic([false, false], VmBinOpLogic::And, false);
    }

    #[test]
    fn test_or() {
        assert_logic([true, true], VmBinOpLogic::Or, true);
        assert_logic([true, false], VmBinOpLogic::Or, true);
        assert_logic([false, true], VmBinOpLogic::Or, true);
        assert_logic([false, false], VmBinOpLogic::Or, false);
    }

    #[test]
    fn test_resolve_type_err() {
        assert_eq!(
            VmBinOpLogic::And.resolve_type(PQLType::LONG, PQLType::BOOLEAN),
            Err(PQLErrorKind::LogicalOperationUnsupported(
                PQLType::LONG,
                PQLType::BOOLEAN,
            )),
        );
    }
}
