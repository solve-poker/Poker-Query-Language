use super::*;

#[derive(Debug, Clone, From)]
pub enum VmInstruction {
    Call(&'static dyn PQLFn),
    BinOp(VmBinOp),
    Push(VmStackValue),
    #[from(skip)]
    Read(VmStoreVarIdx),
    #[from(skip)]
    Write(VmStoreVarIdx),
    CastBoolToLong,
}

impl VmInstruction {
    pub(crate) fn execute(
        &self,
        buffer: &mut VmBuffer,
        store: &mut VmStore,
        stack: &mut VmStack,
    ) -> Result<(), PQLError> {
        match self {
            Self::Push(v) => stack.push(*v),

            Self::Call(function) => {
                let val = function.evaluate(buffer, store, stack)?;

                stack.push(val);
            }

            Self::BinOp(op) => op.execute(buffer, store, stack)?,

            Self::CastBoolToLong => {
                let b: PQLBoolean = stack.downcast_pop()?;

                stack.push(PQLLong::from(b).into());
            }

            Self::Read(i) => {
                let v: &VmStackValue = store.downcast_get(*i)?;

                stack.push(*v);
            }

            Self::Write(i) => {
                let ptr: &mut VmStackValue = store.downcast_get_mut(*i)?;
                let v = stack.pop()?;

                *ptr = v;
            }
        }

        Ok(())
    }
}

pub fn init(
    selectors: &[Selector],
    game: PQLGame,
    player_names: &[&str],
) -> Result<(VmInstructions, VmStore), PQLError> {
    let mut instructions = vec![];
    let mut store = VmStore::default();
    let deps = InitDeps { game, player_names };

    for selector in selectors {
        let rtn_type = push_expr(
            &selector.expr,
            &mut instructions,
            &mut store,
            &deps,
            None,
        )?;

        push_selector(
            selector,
            &mut instructions,
            &mut store,
            &deps,
            rtn_type,
        )?;
    }

    Ok((instructions, store))
}

#[cfg_attr(coverage_nightly, coverage(off))]
#[cfg(test)]
pub mod tests {
    use pql_parser::parser::SelectorParser;

    use super::*;
    use crate::*;

    impl PartialEq for VmInstruction {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::BinOp(l), Self::BinOp(r)) => l == r,
                (Self::Push(l), Self::Push(r)) => l == r,
                (Self::Read(l), Self::Read(r))
                | (Self::Write(l), Self::Write(r)) => l == r,
                (Self::CastBoolToLong, Self::CastBoolToLong) => true,

                (Self::Call(l), Self::Call(r)) => {
                    format!("{l:?}") == format!("{r:?}")
                }

                _ => false,
            }
        }
    }

    fn exec<S, T>(ins: VmInstruction, xs: S, ys: T) -> (VmStore, VmStack)
    where
        S: IntoIterator<Item = VmValue>,
        T: IntoIterator<Item = VmStackValue>,
    {
        let mut buffer = VmBuffer::default();
        let mut store = VmStore::default();
        let mut stack = VmStack::default();

        for x in xs {
            store.try_push(x).unwrap();
        }

        for y in ys {
            stack.push(y);
        }

        ins.execute(&mut buffer, &mut store, &mut stack).unwrap();

        (store, stack)
    }

    #[test]
    fn test_execute_push() {
        let v = PQLLong::from(0).into();
        let (_, mut stack) = exec(VmInstruction::Push(v), [], []);

        assert_eq!(v, stack.pop().unwrap());
    }

    #[test]
    fn test_execute_read() {
        let v: VmStackValue = PQLLong::from(0).into();
        let (_, mut stack) =
            exec(VmInstruction::Read(0.into()), [v.into()], []);

        assert_eq!(v, stack.pop().unwrap());
    }

    #[test]
    fn test_execute_write() {
        let v0: VmStackValue = PQLLong::from(0).into();
        let v1: VmStackValue = PQLLong::from(1).into();
        let (store, mut stack) =
            exec(VmInstruction::Write(0.into()), [v0.into()], [v1]);

        assert_eq!(v1, *store.downcast_get::<&VmStackValue>(0.into()).unwrap());
        assert!(stack.pop().is_err());
    }

    #[test]
    fn test_execute_cast_bool() {
        let (_, mut stack) =
            exec(VmInstruction::CastBoolToLong, [], [true.into()]);

        assert_eq!(1, stack.downcast_pop::<PQLLong>().unwrap());
    }

    fn exec_binop<A1, A2>(op: VmBinOp, l: A1, r: A2) -> VmStackValue
    where
        VmStackValue: From<A1> + From<A2>,
    {
        let mut buffer = VmBuffer::default();
        let mut store = VmStore::default();
        let mut stack = VmStack::default();

        stack.push(r.into());
        stack.push(l.into());

        let ins = VmInstruction::BinOp(op);

        ins.execute(&mut buffer, &mut store, &mut stack).unwrap();

        stack.pop().unwrap()
    }

    #[quickcheck]
    fn test_execute_binop_arith(l: VmStackValueNum, r: VmStackValueNum) {
        use VmBinOpArith::*;
        let v0 = PQLDouble::from(0.0).into();

        let sum = exec_binop(VmBinOp::Arith(Add), l, r);
        let diff = exec_binop(VmBinOp::Arith(Sub), l, r);
        let prod = exec_binop(VmBinOp::Arith(Mul), l, r);

        assert_eq!(sum, (l + r).into());
        assert_eq!(diff, (l - r).into());
        assert_eq!(prod, (l * r).into());

        if r != v0 {
            let quo = exec_binop(VmBinOp::Arith(Div), l, r);
            assert_eq!(quo, (l / r).into());
        }
    }

    #[test]
    fn test_execute_binop_cmp() {
        use VmBinOpCmp::*;
        let t: VmStackValue = true.into();

        assert_eq!(t, exec_binop(VmBinOp::Cmp(Ge), Rank::RA, Rank::RK));
        assert_eq!(t, exec_binop(VmBinOp::Cmp(Gt), Rank::RA, Rank::RK));
        assert_eq!(t, exec_binop(VmBinOp::Cmp(Le), Rank::RQ, Rank::RK));
        assert_eq!(t, exec_binop(VmBinOp::Cmp(Lt), Rank::RQ, Rank::RK));
    }

    #[test]
    fn test_execute_call() {
        let mut buffer = VmBuffer::default();
        let mut store = VmStore::default();
        let mut stack = VmStack::default();

        let ins = VmInstruction::Call(&(functions::turn_card as fn(_) -> _));

        ins.execute(&mut buffer, &mut store, &mut stack).unwrap();

        assert_eq!(
            PQLCard::default(),
            stack.downcast_pop::<PQLCard>().unwrap()
        );
    }

    fn assert_err<S, T>(ins: VmInstruction, xs: S, ys: T)
    where
        S: IntoIterator<Item = VmValue>,
        T: IntoIterator<Item = VmStackValue>,
    {
        let mut buffer = VmBuffer::default();
        let mut store = VmStore::default();
        let mut stack = VmStack::default();

        for x in xs {
            store.try_push(x).unwrap();
        }

        for y in ys {
            stack.push(y);
        }

        assert!(ins.execute(&mut buffer, &mut store, &mut stack).is_err());
    }

    #[test]
    fn test_execute_error() {
        let v: VmStackValue = PQLLong::from(0).into();
        let s: VmValue = VmValue::Str(String::new());

        assert_err(VmInstruction::Read(0.into()), [s.clone()], []);
        assert_err(VmInstruction::Write(0.into()), [s], []);
        assert_err(VmInstruction::Write(0.into()), [v.into()], []);

        assert_err(VmInstruction::CastBoolToLong, [], [v]);
        assert_err(
            VmInstruction::BinOp(VmBinOp::Arith(VmBinOpArith::Add)),
            [],
            [],
        );
    }

    #[test]
    fn test_init() {
        fn s(s: &str) -> Selector {
            SelectorParser::new().parse(s).unwrap()
        }

        let g = PQLGame::default();
        let i = 0.into();
        let (ins, _) = init(&[s("count(1 > 0)")], g, &[]).unwrap();

        assert_eq!(
            ins,
            [
                VmStackValue::INT_ZERO.into(),
                VmStackValue::INT_ONE.into(),
                VmBinOpCmp::Gt.into(),
                VmInstruction::CastBoolToLong,
                VmInstruction::Read(i),
                VmBinOpArith::Add.into(),
                VmInstruction::Write(i),
            ]
        );

        assert!(init(&[s("count(_ > 0)")], g, &[]).is_err());
        assert!(init(&[s("count(1)")], g, &[]).is_err());
    }
}
