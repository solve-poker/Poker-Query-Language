use super::*;

pub fn push_expr(
    data: &mut CompilerData,
    expr: &ast::Expr,
    expected_type: PQLType,
) -> PQLResult<PQLType> {
    use ast::Expr::{BinOp, FnCall, Ident, Num, Str, UnaryOp};

    let rtn_type = match expr {
        Ident(ident) => push_ident(data, ident, expected_type),
        Str(s) => push_str(data, s, expected_type),
        FnCall(fncall) => push_fncall(data, fncall),
        Num(num) => push_num(data, num, expected_type),
        BinOp(op, l, r) => push_binop(data, *op, l, r),
        UnaryOp(op, start, e) => push_unary_op(data, *op, *start, e),
    }?;

    if rtn_type != expected_type
        && rtn_type.is_num()
        && expected_type.is_num()
        && expected_type.is_concrete()
    {
        data.prog
            .push((VmInstruction::CastNum(expected_type), expr.loc()));
        return Ok(expected_type);
    }
    check_type(expr, rtn_type, expected_type)?;

    Ok(rtn_type)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_expr_err() {
        assert_expr_err(
            PQLType::STREET,
            "0.1",
            PQLErrorKind::TypeError {
                given: PQLType::DOUBLE,
                expected: PQLType::STREET,
            },
            "0.1",
        );

        assert_expr_err(
            PQLType::NUMERIC,
            "river",
            PQLErrorKind::TypeError {
                given: PQLType::STREET,
                expected: PQLType::NUMERIC,
            },
            "river",
        );

        assert_expr_err(
            PQLType::BOOLEAN,
            "1",
            PQLErrorKind::TypeError {
                given: PQLType::LONG,
                expected: PQLType::BOOLEAN,
            },
            "1",
        );

        assert_expr_err(
            PQLType::NUMERIC,
            "'AA'",
            PQLErrorKind::TypeError {
                given: PQLType::STRING,
                expected: PQLType::NUMERIC,
            },
            "'AA'",
        );
    }

    #[test]
    fn test_expr_cast() {
        let expr = parse_expr("nthRank(1 + 2, boardRanks(river))").unwrap();
        let mut data = CompilerData::default();
        push_expr(&mut data, &expr, PQLType::RANK).unwrap();

        assert!(
            data.prog
                .iter()
                .any(|(ins, _)| *ins == VmInstruction::CastNum(PQLType::CARDCOUNT))
        );
    }
}
