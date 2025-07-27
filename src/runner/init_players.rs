use super::*;

lazy_static! {
    pub static ref NON_PLAYER_KEYS: FxHashSet<&'static str> =
        ["game", "board"].into_iter().collect();
}

pub fn init_players<'i>(
    fc: &ast::FromClause<'i>,
    game: PQLGame,
) -> (Vec<&'i str>, Vec<RangeProc>) {
    let mut names = vec![];
    let mut procs = vec![];

    for (_, item) in fc
        .inner
        .iter()
        .filter(|(k, _)| !NON_PLAYER_KEYS.contains(&k.as_str()))
    {
        names.push(item.key.inner);

        let err_offset = item.value.loc.0;
        let range_src = item.value.inner.to_owned();

        let proc = move || match PQLRange::from_src(&range_src, game) {
            Ok(c) => Ok(c),
            Err(err) => Err((err_offset, err).into()),
        };

        procs.push(Box::new(proc) as _);
    }

    (names, procs)
}

#[cfg(test)]
mod tests {
    use pql_parser::parser::FromClauseParser;

    use super::*;
    use crate::*;

    fn f(s: &str) -> ast::FromClause {
        FromClauseParser::new().parse(s).unwrap()
    }

    #[test]
    fn test_init_players() {
        let (names, procs) =
            init_players(&f("from hero='Aa', Villain='K'"), PQLGame::Holdem);
        assert_eq!(names, &["hero", "Villain"]);

        let mut i = procs.into_iter();
        let mut p1 = i.next().unwrap()().unwrap();
        let mut p2 = i.next().unwrap()().unwrap();

        let aa = [Card::C_AS, Card::C_AS];

        assert!(p1.is_satisfied(&aa));
        assert!(!p2.is_satisfied(&aa));

        let (_, procs) = init_players(&f("from hero='AAA'"), PQLGame::Holdem);
        let res = procs.into_iter().next().unwrap()();
        assert!(res.is_err());

        let (_, procs) = init_players(&f("from hero='[]-'"), PQLGame::Omaha);
        let res = procs.into_iter().next().unwrap()();
        assert!(res.is_err());
    }
}
