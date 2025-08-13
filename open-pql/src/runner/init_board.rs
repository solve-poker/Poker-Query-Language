use super::*;

pub fn init_board(fc: &ast::FromClause<'_>) -> RangeProc {
    let (err_offset, range_src) = fc.get_pql_str("board").map_or_else(
        || (0, "*".to_owned()),
        |src| (src.loc.0, src.inner.to_owned()),
    );

    Box::new(move || match PQLBoardRange::from_src(&range_src) {
        Ok(c) => Ok(c.into()),
        Err(err) => Err((err_offset, err).into()),
    })
}

#[cfg(test)]
mod tests {
    use pql_parser::parser::FromClauseParser;

    use super::*;
    use crate::*;

    fn f(s: &str) -> ast::FromClause<'_> {
        FromClauseParser::new().parse(s).unwrap()
    }

    fn make_board(s: &str) -> PQLBoardRange {
        PQLBoardRange::from_src(s).unwrap()
    }

    #[test]
    fn test_board() {
        assert_eq!(
            init_board(&f("from p1='*', p2='*'"))(),
            Ok(make_board("*").into())
        );

        assert_eq!(
            init_board(&f("from p1='*', p2='*', board='AsAh'"))(),
            Ok(make_board("AsAh").into())
        );
    }

    #[test]
    fn test_board_error() {
        let err_str = "from p1='*', p2='*', board='???'";
        let offset = err_str.chars().position(|c| c == '?').unwrap() - 1;

        assert_eq!(
            init_board(&f(err_str))(),
            Err(PQLError::InvalidRange(
                offset,
                range_parser::Error::InvalidToken((0, 1))
            )),
        );
    }
}
