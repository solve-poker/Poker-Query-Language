use super::*;

pub fn push_fncall(
    data: &mut CompilerData,
    fncall: &ast::FnCall,
) -> PQLResult<PQLType> {
    let function: &dyn PQLFn =
        with_loc(&fncall.name, || fncall.name.inner.parse())?;

    with_loc(fncall, || {
        validate_argument_count(&fncall.args, &function.arg_types())
    })?;

    let arg_types = function.arg_types();
    for (i, arg) in fncall.args.iter().enumerate() {
        push_expr(data, arg, arg_types[i])?;
    }

    data.prog
        .push((VmInstruction::FnCall(function), fncall.loc));

    Ok(function.rtn_type())
}

const fn validate_argument_count(
    given: &[ast::Expr],
    expected: &[PQLType],
) -> Result<(), PQLErrorKind> {
    if given.len() == expected.len() {
        Ok(())
    } else {
        Err(PQLErrorKind::WrongNumberOfArguments(
            given.len(),
            expected.len(),
        ))
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_fncall() {
        let mut data = CompilerData::default();

        let expr = parse_fn_call("turncard()").unwrap();
        push_fncall(&mut data, &expr).unwrap();

        assert!(matches!(data.prog[0].0, VmInstruction::FnCall(_)));
    }

    fn assert_err<E>(src: &str, _expected_type: PQLType, err: E, err_src: &str)
    where
        PQLErrorKind: From<E>,
    {
        let expr = parse_fn_call(src).unwrap();

        let mut data = CompilerData::default();

        let pos_s = src.find(err_src).unwrap();
        let pos_e = pos_s + err_src.len();

        assert_eq!(
            push_fncall(&mut data, &expr).unwrap_err(),
            ((pos_s, pos_e), err).into()
        );
    }

    #[test]
    fn test_fncall_err_fnname() {
        assert_err(
            "invalid(1, 2)",
            PQLType::NUMERIC,
            PQLErrorKind::UnrecognizedFunction,
            "invalid",
        );
    }

    #[test]
    fn test_fncall_err_nargs() {
        let given = 3;
        let expected = 2;

        assert_err(
            "equity(hero, river, extra)",
            PQLType::NUMERIC,
            PQLErrorKind::WrongNumberOfArguments(given, expected),
            "equity(hero, river, extra)",
        );
    }

    #[test]
    fn test_fncall_err_arg() {
        let given = PQLType::DOUBLE;
        let expected = PQLType::PLAYER;

        assert_err(
            "equity(1.0, flop)",
            PQLType::NUMERIC,
            PQLErrorKind::TypeError(given, expected),
            "1.0",
        );
    }
}
