use super::*;

pub fn init_game(fc: &ast::FromClause<'_>) -> Result<PQLGame, PQLError> {
    fc.get_pql_str("game").map_or_else(
        || Ok(PQLGame::default()),
        |s| {
            s.inner
                .parse()
                .map_or(Err(PQLError::UnrecognizedGame(s.loc)), Ok)
        },
    )
}

#[cfg(test)]
mod tests {
    use pql_parser::parser::FromClauseParser;

    use super::*;
    use crate::*;

    fn f(s: &str) -> ast::FromClause<'_> {
        FromClauseParser::new().parse(s).unwrap()
    }

    #[test]
    fn test_init_game() {
        assert_eq!(
            init_game(&f("from game='  HOLDEM '")),
            Ok(PQLGame::Holdem),
            "should trim and ignore case"
        );

        assert_eq!(
            init_game(&f("from key='val'")),
            Ok(PQLGame::Holdem),
            "should default to holdem"
        );
    }
}
