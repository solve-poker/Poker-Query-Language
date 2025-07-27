use super::*;

pub(super) fn push_selector(
    selector: &ast::Selector,
    instructions: &mut VmInstructions,
    store: &mut VmStore,
    deps: &InitDeps,
    expr_type: PQLType,
) -> Result<(), PQLError> {
    match selector.kind {
        ast::SelectorKind::Avg => {
            if let Some(v) = zero_of(expr_type) {
                let idx = store.try_push(v.into())?;
                instructions.push(VmInstruction::Read(idx));
                instructions.push(VmBinOpArith::Add.into());
                instructions.push(VmInstruction::Write(idx));

                let idx = store.try_push(VmStackValue::INT_ZERO.into())?;
                instructions.push(VmStackValue::INT_ONE.into());
                instructions.push(VmInstruction::Read(idx));
                instructions.push(VmBinOpArith::Add.into());
                instructions.push(VmInstruction::Write(idx));
            } else {
                return Err(make_err(selector, expr_type));
            }
        }

        ast::SelectorKind::Count => {
            if expr_type == PQLType::Boolean {
                let idx = store.try_push(VmStackValue::INT_ZERO.into())?;
                instructions.push(VmInstruction::CastBoolToLong);
                instructions.push(VmInstruction::Read(idx));
                instructions.push(VmBinOpArith::Add.into());
                instructions.push(VmInstruction::Write(idx));
            } else {
                return Err(make_err(selector, expr_type));
            }
        }

        ast::SelectorKind::Max => {
            if let Ok(v) = VmStackValue::min_of(expr_type, deps.game) {
                let idx = store.try_push(v.into())?;
                instructions.push(VmInstruction::Read(idx));
                instructions.push(VmBinOp::MaxOf.into());
                instructions.push(VmInstruction::Write(idx));
            } else {
                return Err(make_err(selector, expr_type));
            }
        }

        ast::SelectorKind::Min => {
            if let Ok(v) = VmStackValue::max_of(expr_type, deps.game) {
                let idx = store.try_push(v.into())?;
                instructions.push(VmInstruction::Read(idx));
                instructions.push(VmBinOp::MinOf.into());
                instructions.push(VmInstruction::Write(idx));
            } else {
                return Err(make_err(selector, expr_type));
            }
        }
    }

    Ok(())
}

#[inline]
const fn zero_of(t: PQLType) -> Option<VmStackValue> {
    match t {
        // TODO: fix this
        PQLType::Fraction
        | PQLType::Numeric
        | PQLType::Double
        | PQLType::Equity => Some(VmStackValue::DBL_ZERO),

        PQLType::Integer
        | PQLType::Long
        | PQLType::CardCount
        | PQLType::PlayerCount => Some(VmStackValue::INT_ZERO),
        _ => None,
    }
}

#[inline]
fn make_err(selector: &ast::Selector, expr_type: PQLType) -> PQLError {
    (
        selector.expr.loc(),
        TypeError::SelectorDoesNotSupport(selector.kind.to_string(), expr_type),
    )
        .into()
}

#[allow(clippy::float_cmp)]
#[cfg_attr(coverage_nightly, coverage(off))]
#[cfg(test)]
pub mod tests {
    use pql_parser::parser::SelectorParser;

    use super::*;
    use crate::*;

    fn s(s: &str) -> Selector {
        SelectorParser::new().parse(s).unwrap()
    }

    fn push(selector: &ast::Selector, t: PQLType) -> (VmInstructions, VmStore) {
        let mut instructions = vec![];
        let mut store = VmStore::default();
        let deps = InitDeps::default();

        push_selector(selector, &mut instructions, &mut store, &deps, t)
            .unwrap();

        (instructions, store)
    }

    fn fst_val<T>(selector: &ast::Selector, t: PQLType) -> T
    where
        T: Clone,
        for<'a> &'a T: TryFrom<&'a VmValue>,
    {
        push(selector, t)
            .1
            .downcast_get::<&T>(0.into())
            .unwrap()
            .clone()
    }

    #[test]
    fn test_push_selector_avg() {
        let sel = s("avg(_)");
        let i = 0.into();

        let (ins, store) = push(&sel, PQLType::Double);

        assert_eq!(
            ins,
            [
                VmInstruction::Read(i),
                VmBinOpArith::Add.into(),
                VmInstruction::Write(i),
                VmStackValue::ONE.into(),
                VmInstruction::Read(i + 1),
                VmBinOpArith::Add.into(),
                VmInstruction::Write(i + 1),
            ]
        );

        assert_eq!(0, *store.downcast_get::<&PQLLong>(i + 1).unwrap());

        assert_eq!(PQLDouble::from(0.0), fst_val(&sel, PQLType::Double));
        assert_eq!(PQLDouble::from(0.0), fst_val(&sel, PQLType::Equity));
        assert_eq!(PQLLong::from(0), fst_val(&sel, PQLType::Long));
        assert_eq!(PQLLong::from(0), fst_val(&sel, PQLType::Integer));
        assert_eq!(PQLLong::from(0), fst_val(&sel, PQLType::CardCount));
        assert_eq!(PQLLong::from(0), fst_val(&sel, PQLType::PlayerCount));
    }

    #[test]
    fn test_push_selector_count() {
        let sel = s("count(_)");
        let i = 0.into();

        let (ins, store) = push(&sel, PQLType::Boolean);

        assert_eq!(
            ins,
            [
                VmInstruction::CastBoolToLong,
                VmInstruction::Read(i),
                VmBinOpArith::Add.into(),
                VmInstruction::Write(i),
            ]
        );

        assert_eq!(0, *store.downcast_get::<&PQLLong>(i).unwrap());
    }

    #[test]
    fn test_push_selector_max() {
        let sel = s("max(_)");
        let i = 0.into();

        assert_eq!(
            push(&sel, PQLType::Long).0,
            [
                VmInstruction::Read(i),
                VmBinOp::MaxOf.into(),
                VmInstruction::Write(i),
            ]
        );

        assert_eq!(PQLDouble::MIN, fst_val(&sel, PQLType::Double));
        assert_eq!(PQLDouble::MIN, fst_val(&sel, PQLType::Equity));
        assert_eq!(PQLLong::MIN, fst_val(&sel, PQLType::Long));
        assert_eq!(PQLLong::MIN, fst_val(&sel, PQLType::Integer));
        assert_eq!(PQLCardCount::MIN, fst_val(&sel, PQLType::CardCount));
        assert_eq!(PQLCardCount::MIN, fst_val(&sel, PQLType::PlayerCount));

        assert_eq!(
            PQLFlopHandCategory::min(PQLGame::default()),
            fst_val(&sel, PQLType::FlopHandCategory)
        );
        assert_eq!(
            PQLHandType::min(PQLGame::default()),
            fst_val(&sel, PQLType::HandType)
        );
        assert_eq!(PQLHiRating::MIN, fst_val(&sel, PQLType::HiRating));
    }

    #[test]
    fn test_push_selector_min() {
        let sel = s("min(_)");
        let i = 0.into();

        assert_eq!(
            push(&sel, PQLType::Long).0,
            [
                VmInstruction::Read(i),
                VmBinOp::MinOf.into(),
                VmInstruction::Write(i),
            ]
        );

        assert_eq!(PQLDouble::MAX, fst_val(&sel, PQLType::Double));
        assert_eq!(PQLDouble::MAX, fst_val(&sel, PQLType::Equity));
        assert_eq!(PQLLong::MAX, fst_val(&sel, PQLType::Long));
        assert_eq!(PQLLong::MAX, fst_val(&sel, PQLType::Integer));
        assert_eq!(PQLCardCount::MAX, fst_val(&sel, PQLType::CardCount));
        assert_eq!(PQLCardCount::MAX, fst_val(&sel, PQLType::PlayerCount));

        assert_eq!(
            PQLFlopHandCategory::max(PQLGame::default()),
            fst_val(&sel, PQLType::FlopHandCategory)
        );
        assert_eq!(
            PQLHandType::max(PQLGame::default()),
            fst_val(&sel, PQLType::HandType)
        );
        assert_eq!(PQLHiRating::MAX, fst_val(&sel, PQLType::HiRating));
    }

    fn assert_err(selector: &ast::Selector, t: PQLType) {
        let mut instructions = vec![];
        let mut store = VmStore::default();
        let deps = InitDeps::default();

        assert!(push_selector(
            selector,
            &mut instructions,
            &mut store,
            &deps,
            t
        )
        .is_err());
    }

    #[quickcheck]
    fn test_push_selector_error_avg(t: PQLType) {
        if !t.is_num() {
            assert_err(&s("avg(_)"), t);
        }
    }

    #[quickcheck]
    fn test_push_selector_error_count(t: PQLType) {
        if t != PQLType::Boolean {
            assert_err(&s("count(_)"), t);
        }
    }

    #[test]
    fn test_push_selector_error_min_max() {
        use PQLType::*;

        for text in ["min(_)", "max(_)"] {
            assert_err(&s(text), BoardRange);
            assert_err(&s(text), Boolean);
            assert_err(&s(text), Card);
            assert_err(&s(text), Player);
            assert_err(&s(text), Range);
            assert_err(&s(text), RankSet);
            assert_err(&s(text), Street);
            assert_err(&s(text), String);
        }
    }

    #[test]
    fn test_push_selector_error() {
        fn assert_store_err(selector: &ast::Selector, t: PQLType) {
            let mut instructions = vec![];
            let mut store = VmStore::new_packed();
            let deps = InitDeps::default();

            assert!(push_selector(
                selector,
                &mut instructions,
                &mut store,
                &deps,
                t
            )
            .is_err());
        }

        assert_store_err(&s("avg(_)"), PQLType::Long);
        assert_store_err(&s("count(_)"), PQLType::Boolean);
        assert_store_err(&s("min(_)"), PQLType::Long);
        assert_store_err(&s("max(_)"), PQLType::Long);

        let mut store = VmStore::new_packed();
        store.inner.pop();

        assert!(push_selector(
            &s("avg(_)"),
            &mut vec![],
            &mut store,
            &InitDeps::default(),
            PQLType::Long
        )
        .is_err());
    }
}
