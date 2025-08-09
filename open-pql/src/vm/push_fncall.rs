use super::*;

pub(super) fn push_fncall(
    fncall: &ast::FnCall,
    instructions: &mut VmInstructions,
    store: &mut VmStore,
    deps: &InitDeps,
) -> Result<PQLType, PQLError> {
    let function: &dyn PQLFn = fncall
        .name
        .inner
        .parse()
        .map_err(|_| PQLError::UnrecognizedFunction(fncall.loc))?;

    let arg_types = function.arg_types();

    let rtn_type = function.rtn_type();

    if fncall.args.len() != arg_types.len() {
        return Err(PQLError::WrongNumberOfArguments(
            fncall.loc,
            fncall.args.len(),
            arg_types.len(),
        ));
    }

    for (expected, expr) in arg_types.iter().zip(&fncall.args).rev() {
        let _ = push_expr(expr, instructions, store, deps, Some(*expected))?;
    }

    instructions.push(VmInstruction::Call(function));

    Ok(rtn_type)
}

#[cfg(test)]
pub mod tests {
    use pql_parser::parser::FnCallParser;

    use super::*;
    use crate::*;

    fn f(s: &str) -> ast::FnCall {
        FnCallParser::new().parse(s).unwrap()
    }

    #[test]
    fn test_push_fncall_error() {
        fn pushe(s: &str) -> PQLError {
            push_fncall(
                &f(s),
                &mut vec![],
                &mut VmStore::default(),
                &InitDeps::default(),
            )
            .unwrap_err()
        }

        assert_eq!(
            pushe("handranks(hero, flop, 100)"),
            PQLError::WrongNumberOfArguments((0, 26), 3, 2),
        );

        assert_eq!(
            pushe("invalidfunction(1)"),
            PQLError::UnrecognizedFunction((0, 18)),
        );
    }
}
