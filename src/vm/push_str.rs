use super::*;

pub(super) fn push_str(
    s: &ast::Str,
    instructions: &mut VmInstructions,
    store: &mut VmStore,
    deps: &InitDeps,
    expected_type: Option<PQLType>,
) -> Result<PQLType, PQLError> {
    let (val, kind) = push_val(s, expected_type, store, deps)?;

    instructions.push(val.into());

    Ok(kind)
}

pub(super) fn push_val(
    s: &ast::Str,
    kind: Option<PQLType>,
    store: &mut VmStore,
    deps: &InitDeps,
) -> Result<(VmStackValue, PQLType), PQLError> {
    match kind {
        Some(PQLType::BoardRange) => match PQLBoardRange::from_src(s.inner) {
            Ok(c) => Ok((store.try_push(c.into())?.into(), PQLType::Range)),
            Err(err) => Err((s.loc.0, err).into()),
        },
        Some(PQLType::Range) => match PQLRange::from_src(s.inner, deps.game) {
            Ok(c) => Ok((store.try_push(c.into())?.into(), PQLType::Range)),
            Err(err) => Err((s.loc.0, err).into()),
        },
        Some(PQLType::String) | None => Ok((
            store.try_push(s.inner.to_owned().into())?.into(),
            PQLType::String,
        )),

        Some(k) => Err((s.loc, TypeError::TypeMismatch(k)).into()),
    }
}

#[cfg_attr(coverage_nightly, coverage(off))]
#[cfg(test)]
mod tests {
    use pql_parser::parser::StrParser;

    use super::*;
    use crate::*;

    fn s(s: &str) -> ast::Str {
        StrParser::new().parse(s).unwrap()
    }

    #[test]
    fn test_error() {
        fn pushe(s: &ast::Str, t: Option<PQLType>) -> PQLError {
            push_str(
                s,
                &mut Vec::default(),
                &mut VmStore::default(),
                &InitDeps::default(),
                t,
            )
            .unwrap_err()
        }

        fn to_loc(e: PQLError) -> Option<LocInfo> {
            (&e).into()
        }

        let mut v = s("'AAAAAA'");
        v.loc = (100, 200);

        assert_eq!(Some((101, 107)), to_loc(pushe(&v, Some(PQLType::Range))));
        assert_eq!(
            Some((101, 107)),
            to_loc(pushe(&v, Some(PQLType::BoardRange)))
        );

        assert!(matches!(
            pushe(&v, Some(PQLType::Rank)),
            PQLError::TypeError(_, TypeError::TypeMismatch(PQLType::Rank))
        ));

        let v = s("'*'");
        for t in [
            Some(PQLType::Range),
            Some(PQLType::BoardRange),
            Some(PQLType::String),
            None,
        ] {
            assert!(push_str(
                &v,
                &mut Vec::default(),
                &mut VmStore::new_packed(),
                &InitDeps::default(),
                t,
            )
            .is_err());
        }
    }

    #[test]
    fn test_range() {
        let mut store = VmStore::default();
        let deps = InitDeps::default();

        let v = s("'AA'");

        let _ = push_val(&v, Some(PQLType::Range), &mut store, &deps).unwrap();
        let range = store.downcast_get_mut::<&mut PQLRange>(0.into()).unwrap();

        assert!(range.is_satisfied(cards!("AsAh").as_ref()));
        assert!(!range.is_satisfied(cards!("2s2h").as_ref()));

        let _ =
            push_val(&v, Some(PQLType::BoardRange), &mut store, &deps).unwrap();

        let _ = store
            .downcast_get_mut::<&mut PQLBoardRange>(1.into())
            .unwrap();
    }

    #[test]
    fn test_str() {
        let mut store = VmStore::default();
        let deps = InitDeps::default();

        let v = s("'string'");

        let _ = push_val(&v, None, &mut store, &deps).unwrap();

        let ptr = store.downcast_get::<&PQLString>(0.into()).unwrap();

        assert_eq!("string", ptr);
    }
}
