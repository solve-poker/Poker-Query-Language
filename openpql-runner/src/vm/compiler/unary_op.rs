use super::*;

pub fn push_unary_op(
    data: &mut CompilerData,
    op: ast::UnaryOp,
    start: usize,
    arg: &ast::Expr,
) -> PQLResult<PQLType> {
    let arg_type = push_expr(data, arg, PQLType::all())?;

    let loc = (start, arg.loc().1);
    let op = VmUnaryOp::from(op);

    match op.resolve_type(arg_type) {
        Ok(rtn_type) => {
            data.prog.push((VmInstruction::UnaryOp(op), loc));

            Ok(rtn_type)
        }
        Err(err) => Err(mk_err(&loc, err)),
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_not() {
        let expr = parse_expr("not 1 = 1").unwrap();
        let mut data = CompilerData::default();
        let tp = push_expr(&mut data, &expr, PQLType::BOOLEAN).unwrap();

        assert_eq!(tp, PQLType::BOOLEAN);
        assert_eq!(
            data.prog.last().map(|(ins, _)| ins.clone()),
            Some(VmInstruction::UnaryOp(VmUnaryOpLogic::Not.into())),
        );
    }

    #[test]
    fn test_not_err() {
        assert_expr_err(
            PQLType::BOOLEAN,
            "not 1",
            PQLErrorKind::LogicalOperationUnsupported {
                lhs: PQLType::LONG,
                rhs: PQLType::LONG,
            },
            "not 1",
        );
    }
}
