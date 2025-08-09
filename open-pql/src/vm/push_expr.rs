use push_fncall::push_fncall;
use push_ident::push_ident;
use push_num::push_num;
use push_str::push_str;

use super::*;

pub(super) fn push_expr(
    expr: &ast::Expr,
    instructions: &mut VmInstructions,
    store: &mut VmStore,
    deps: &InitDeps,
    expected_type: Option<PQLType>,
) -> Result<PQLType, PQLError> {
    match expr {
        ast::Expr::Ident(id) => {
            push_ident(id, instructions, store, deps, expected_type)
        }
        ast::Expr::Str(s) => {
            push_str(s, instructions, store, deps, expected_type)
        }
        ast::Expr::Int(i) => {
            push_num(i, instructions, store, deps, expected_type)
        }

        ast::Expr::FnCall(fncall) => {
            push_fncall(fncall, instructions, store, deps)
        }

        ast::Expr::BinOp(op, l, r) => {
            let op: VmBinOp = (*op).into();
            let kind_r = push_expr(r, instructions, store, deps, None)?;
            let kind_l = push_expr(l, instructions, store, deps, None)?;

            let kind = op.check_type(expr.loc(), kind_l, kind_r)?;

            instructions.push(op.into());

            Ok(kind)
        }
    }
}

#[cfg(test)]
pub mod tests {
    use pql_parser::parser::ExprParser;

    use super::*;
    use crate::*;

    fn e(s: &str) -> ast::Expr {
        ExprParser::new().parse(s).unwrap()
    }

    #[test]
    fn test_push_expr() {
        fn push(expr: ast::Expr) -> (VmInstructions, VmStore) {
            let mut v = vec![];
            let mut s = VmStore::default();
            push_expr(&expr, &mut v, &mut s, &InitDeps::default(), None)
                .unwrap();

            (v, s)
        }

        assert_eq!(
            push(e("flop")).0,
            [VmInstruction::Push(PQLStreet::Flop.into())]
        );

        assert_eq!(
            push(e("1.0")).0,
            [VmInstruction::Push(PQLDouble::from(1.0).into())]
        );

        assert_eq!(
            push(e("1+0")).0,
            [
                VmInstruction::Push(PQLLong::from(0).into()),
                VmInstruction::Push(PQLLong::from(1).into()),
                VmBinOpArith::Add.into(),
            ],
            "should push second argument, first argument and op"
        );

        assert_eq!(
            push(e("boardranks(flop)")).0,
            [
                VmInstruction::Push(PQLStreet::Flop.into()),
                VmInstruction::Call(&(functions::board_ranks as fn(_, _) -> _)),
            ]
        );

        assert_eq!(
            push(e("'string'"))
                .1
                .downcast_get::<&PQLString>(0.into())
                .unwrap(),
            "string",
        );
    }

    #[test]
    fn test_push_expr_error() {
        fn pushe(expr: ast::Expr) -> PQLError {
            let mut v = vec![];
            let mut s = VmStore::default();

            push_expr(&expr, &mut v, &mut s, &InitDeps::default(), None)
                .unwrap_err()
        }

        assert_eq!(pushe(e("_ = _")), PQLError::UnknownIdent((4, 5)));
        assert_eq!(pushe(e("_ = 1")), PQLError::UnknownIdent((0, 1)));
        assert_eq!(
            pushe(e("1 = flop")),
            PQLError::TypeError(
                (0, 8),
                TypeError::BinOpError(
                    "=".into(),
                    PQLType::Integer,
                    PQLType::Street
                )
            )
        );

        let err = "256".parse::<u8>().unwrap_err();
        assert_eq!(
            pushe(e("minoutstohandtype(_, flop, pair, 256)")),
            PQLError::ParseIntError((33, 36), err)
        );
    }
}
