use super::*;

pub fn push_binop(
    data: &mut CompilerData,
    op: ast::BinOp,
    lhs: &ast::Expr,
    rhs: &ast::Expr,
) -> PQLResult<PQLType> {
    let lhs_type = push_expr(data, lhs, PQLType::all())?;
    let rhs_type = push_expr(data, rhs, PQLType::all())?;

    let loc = (lhs.loc().0, rhs.loc().1);
    let op = VmBinOp::from(op);

    match op.resolve_type(lhs_type, rhs_type) {
        Ok(rtn_type) => {
            data.prog.push((op.into(), loc));

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

    fn assert_binop(
        type_hint: PQLType,
        src: &str,
        lhs: VmStackValue,
        op: VmBinOp,
        rhs: VmStackValue,
    ) {
        let expr = parse_expr(src).unwrap();

        let mut data = CompilerData::default();

        let tp = push_expr(&mut data, &expr, type_hint).unwrap();

        assert_eq!(data.prog[0].0, lhs.into());
        assert_eq!(data.prog[1].0, rhs.into());
        assert_eq!(data.prog[2].0, op.into());
        assert_eq!(tp, type_hint);
    }

    #[test]
    fn test_arith() {
        assert_binop(
            PQLType::LONG,
            "2 + 1",
            sval!(@long 2),
            VmBinOpArith::Add.into(),
            sval!(@long 1),
        );

        assert_binop(
            PQLType::LONG,
            "2 - 1",
            sval!(@long 2),
            VmBinOpArith::Sub.into(),
            sval!(@long 1),
        );

        assert_binop(
            PQLType::LONG,
            "2 * 1",
            sval!(@long 2),
            VmBinOpArith::Mul.into(),
            sval!(@long 1),
        );

        assert_binop(
            PQLType::DOUBLE,
            "2 / 1",
            sval!(@long 2),
            VmBinOpArith::Div.into(),
            sval!(@long 1),
        );
    }

    #[test]
    fn test_cmp() {
        assert_binop(
            PQLType::BOOLEAN,
            "flush <= fullhouse",
            sval!(@handtype PQLHandType::Flush),
            VmBinOpCmp::Le.into(),
            sval!(@handtype PQLHandType::FullHouse),
        );

        assert_binop(
            PQLType::BOOLEAN,
            "flush = fullhouse",
            sval!(@handtype PQLHandType::Flush),
            VmBinOpCmp::Eq.into(),
            sval!(@handtype PQLHandType::FullHouse),
        );
    }

    #[test]
    fn test_binop_err() {
        assert_expr_err(
            PQLType::NUMERIC,
            "flop + 1",
            PQLErrorKind::ArithmeticOperationUnsupported(
                PQLType::STREET,
                PQLType::LONG,
            ),
            "flop + 1",
        );

        assert_expr_err(
            PQLType::BOOLEAN,
            "1 > flop",
            PQLErrorKind::ComparisonOperationUnsupported(
                PQLType::LONG,
                PQLType::STREET,
            ),
            "1 > flop",
        );

        assert_expr_err(
            PQLType::NUMERIC,
            "flopunderpair >= flopnothing",
            PQLErrorKind::TypeError(PQLType::BOOLEAN, PQLType::NUMERIC),
            "flopunderpair >= flopnothing",
        );

        assert_expr_err(
            PQLType::NUMERIC,
            "a + 1",
            PQLErrorKind::UnrecognizedIdentifier,
            "a",
        );

        assert_expr_err(
            PQLType::NUMERIC,
            "1 + b",
            PQLErrorKind::UnrecognizedIdentifier,
            "b",
        );
    }

    #[test]
    fn test_binop_cmp_err() {
        assert_expr_err(
            PQLType::BOOLEAN,
            "flop < turn",
            PQLErrorKind::ComparisonOperationUnsupported(
                PQLType::STREET,
                PQLType::STREET,
            ),
            "flop < turn",
        );
    }
}
