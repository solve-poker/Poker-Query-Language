use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VmUnaryOpLogic {
    Not,
}

impl VmUnaryOpLogic {
    #[allow(clippy::unnecessary_wraps)]
    pub fn execute(self, ctx: &mut VmExecContext) -> Result<(), PQLErrorKind> {
        let arg = ctx.stack.downcast_pop::<PQLBoolean>();

        ctx.stack.push(
            match self {
                Self::Not => !arg,
            }
            .into(),
        );

        Ok(())
    }

    #[allow(clippy::unused_self)]
    pub fn resolve_type(
        self,
        arg_type: PQLType,
    ) -> Result<PQLType, PQLErrorKind> {
        if arg_type == PQLType::BOOLEAN {
            Ok(PQLType::BOOLEAN)
        } else {
            Err(PQLErrorKind::LogicalOperationUnsupported(
                arg_type, arg_type,
            ))
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    fn assert_logic<T>(val: T, op: VmUnaryOpLogic, expected: bool)
    where
        VmStackValue: From<T>,
    {
        let mut ctx = VmExecContext::default();
        ctx.stack.push(val.into());

        let ins = VmInstruction::UnaryOp(op.into());
        ins.execute(&mut ctx).unwrap();

        assert_eq!(ctx.stack.pop().unwrap(), expected.into());
    }

    #[test]
    fn test_not() {
        assert_logic(true, VmUnaryOpLogic::Not, false);
        assert_logic(false, VmUnaryOpLogic::Not, true);
    }

    #[test]
    fn test_resolve_type_err() {
        assert_eq!(
            VmUnaryOpLogic::Not.resolve_type(PQLType::LONG),
            Err(PQLErrorKind::LogicalOperationUnsupported(
                PQLType::LONG,
                PQLType::LONG,
            )),
        );
    }
}
