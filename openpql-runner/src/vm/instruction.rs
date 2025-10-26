use super::*;

#[derive(Clone, derive_more::Debug, derive_more::From)]
pub enum VmInstruction {
    #[debug("Push({_0})")]
    Push(VmStackValue),
    FnCall(&'static dyn PQLFn),
    BinOp(VmBinOp),
    CastNum(PQLType),
}

impl VmInstruction {
    pub fn execute(&self, ctx: &mut VmExecContext) -> Result<(), PQLErrorKind> {
        match self {
            Self::Push(stack_value) => ctx.stack.push(*stack_value),
            Self::FnCall(proc) => {
                proc.execute(ctx).map(|val| ctx.stack.push(val))?;
            }
            Self::BinOp(op) => op.execute(ctx)?,
            Self::CastNum(kind) => {
                let value = ctx.stack.pop()?;

                ctx.stack.push(cast_num(value, *kind)?);
            }
        }

        Ok(())
    }
}

fn cast_num(
    value: VmStackValue,
    kind: PQLType,
) -> Result<VmStackValue, PQLErrorKind> {
    match (kind, value) {
        (PQLType::CARDCOUNT, VmStackValue::Long(v)) => {
            PQLCardCount::try_from(v).map_or_else(
                |_| {
                    Err(RuntimeError::ValueRetrievalFailed(PQLType::CARDCOUNT)
                        .into())
                },
                |count| Ok(count.into()),
            )
        }
        (
            PQLType::CARDCOUNT,
            VmStackValue::Frac(_) | VmStackValue::Double(_),
        ) => Err(RuntimeError::ValueRetrievalFailed(PQLType::CARDCOUNT).into()),
        _ => Err(InternalError::UnexpectedTypeCast.into()),
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
pub mod tests {
    use VmInstruction::*;

    use super::*;
    use crate::*;

    impl PartialEq for VmInstruction {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::Push(l), Self::Push(r)) => l == r,
                (Self::FnCall(_l), Self::FnCall(_r)) => panic!(),
                (Self::BinOp(l), Self::BinOp(r)) => l == r,
                (Self::CastNum(l), Self::CastNum(r)) => l == r,
                _ => false,
            }
        }
    }

    #[test]
    fn test_push() {
        let mut ctx = VmExecContext::default();

        for ins in [Push(sval!(@long 100)), Push(sval!(@count 10))] {
            ins.execute(&mut ctx).unwrap();
        }

        assert_eq!(ctx.stack.pop().unwrap(), sval!(@count 10));
        assert_eq!(ctx.stack.pop().unwrap(), sval!(@long 100));
    }

    fn test_function(street: PQLStreet, _: ()) -> PQLBoolean {
        street == PQLStreet::Flop
    }

    fn test_function_ptr() -> &'static dyn PQLFn {
        &(test_function as fn(street: PQLStreet, _: ()) -> PQLBoolean)
    }

    impl PQLFn for fn(PQLStreet, ()) -> PQLBoolean {
        fn arg_types(&self) -> Vec<PQLType> {
            vec![PQLType::STREET]
        }

        fn rtn_type(&self) -> PQLType {
            PQLType::BOOLEAN
        }

        fn execute(
            &self,
            ctx: &mut VmExecContext,
        ) -> Result<VmStackValue, PQLErrorKind> {
            let arg = ctx.stack.downcast_pop::<PQLStreet>();

            Ok(self(arg, ()).into())
        }
    }

    #[test]
    fn test_fncall() {
        let mut ctx = VmExecContext::default();

        for ins in [Push(sval!(@street turn)), FnCall(test_function_ptr())] {
            ins.execute(&mut ctx).unwrap();
        }

        assert_eq!(ctx.stack.pop().unwrap(), sval!(@bool false));
    }

    #[test]
    fn test_cast() {
        let mut ctx = VmExecContext::default();

        for ins in [Push(sval!(@long 1)), CastNum(PQLType::CARDCOUNT)] {
            ins.execute(&mut ctx).unwrap();
        }

        assert_eq!(ctx.stack.pop().unwrap(), sval!(@count 1));

        let mut ctx = VmExecContext::default();
        ctx.stack.push(sval!(@long 256));

        assert_eq!(
            CastNum(PQLType::CARDCOUNT).execute(&mut ctx),
            Err(RuntimeError::ValueRetrievalFailed(PQLType::CARDCOUNT).into())
        );
    }

    fn assert_fncall_err<E>(src: &str, e: E)
    where
        PQLErrorKind: From<E>,
    {
        let expr = parse_expr(src).unwrap();
        let mut data = CompilerData::default();
        push_expr(&mut data, &expr, PQLType::RANK).unwrap();

        let mut ctx = VmExecContext::from(&data);

        let err = data
            .prog
            .iter()
            .try_for_each(|(ins, _)| ins.execute(&mut ctx))
            .unwrap_err();

        assert_eq!(err, e.into());
    }

    #[test]
    fn test_fncall_err() {
        assert_fncall_err(
            "nthRank(6, boardRanks(river))",
            RuntimeError::ValueRetrievalFailed(PQLType::RANK),
        );
        assert_fncall_err(
            "nthRank(255 + 1.0, boardRanks(river))",
            RuntimeError::ValueRetrievalFailed(PQLType::CARDCOUNT),
        );
    }

    #[test]
    fn test_internal() {
        assert_eq!(
            CastNum(PQLType::CARDCOUNT).execute(&mut VmExecContext::default()),
            Err(InternalError::StackUnderflow.into()),
        );

        assert_fncall_err(
            "nthRank(255 + 1.0, boardRanks(river))",
            RuntimeError::ValueRetrievalFailed(PQLType::CARDCOUNT),
        );

        let mut ctx = VmExecContext::default();
        ctx.stack.push(sval!(@long 256));

        assert_eq!(
            CastNum(PQLType::STREET).execute(&mut ctx),
            Err(InternalError::UnexpectedTypeCast.into())
        );
    }
}
