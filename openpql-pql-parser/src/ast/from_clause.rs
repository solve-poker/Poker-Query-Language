use super::{
    Entry, Error, FxHashMap, Ident, Loc, ResultE, Str, String, fmt, user_err,
};

#[derive(PartialEq, Eq, Default)]
pub struct FromClause<'i> {
    pub inner: FxHashMap<String, FromItem<'i>>,
    pub loc: (Loc, Loc),
}

impl<'i> FromClause<'i> {
    const BOARD_KEY: &'static str = "board";
    const GAME_KEY: &'static str = "game";
    const DEADCARD_KEY: &'static str = "dead";
    const NON_PLAYER_KEYS: [&'static str; 3] =
        [Self::BOARD_KEY, Self::GAME_KEY, Self::DEADCARD_KEY];

    pub(crate) fn new<T: IntoIterator<Item = FromItem<'i>>>(
        items: T,
        loc: (Loc, Loc),
    ) -> ResultE<'i, Self> {
        let mut res = Self::default();

        for item in items {
            let key = item.key.inner.to_ascii_lowercase();

            if let Entry::Vacant(e) = res.inner.entry(key) {
                e.insert(item);
            } else {
                return Err(user_err(Error::DuplicatedKeyInFrom(item.key.loc)));
            }
        }

        res.loc = loc;

        Ok(res)
    }

    fn get_val(&self, key: &str) -> Option<&Str<'_>> {
        self.inner.get(key).as_ref().map(|item| &item.value)
    }

    pub fn get_board_range(&self) -> Option<&Str<'_>> {
        self.get_val(Self::BOARD_KEY)
    }

    pub fn get_game(&self) -> Option<&Str<'_>> {
        self.get_val(Self::GAME_KEY)
    }

    pub fn get_dead(&self) -> Option<&Str<'_>> {
        self.get_val(Self::DEADCARD_KEY)
    }

    pub fn get_players(&self) -> Vec<(&Ident<'_>, &Str<'_>)> {
        self.inner
            .keys()
            .filter(|k| !Self::NON_PLAYER_KEYS.contains(&k.as_str()))
            .map(|k| &self.inner[k])
            .map(|item| (&item.key, &item.value))
            .collect()
    }
}

impl fmt::Debug for FromClause<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map()
            .entries(
                self.inner
                    .values()
                    .map(|item| (item.key.inner, item.value.inner)),
            )
            .finish()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct FromItem<'i> {
    pub key: Ident<'i>,
    pub value: Str<'i>,
}

impl<'i, U, V> From<(U, V)> for FromItem<'i>
where
    U: Into<Ident<'i>>,
    V: Into<Str<'i>>,
{
    fn from(t: (U, V)) -> Self {
        Self {
            key: t.0.into(),
            value: t.1.into(),
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    fn mk_inner<'s>(
        src: &'s str,
        kvs: &[(&'static str, &'static str)],
    ) -> FxHashMap<String, FromItem<'s>> {
        let mut res = FxHashMap::default();
        for (key, val) in kvs {
            let id = Ident::from((*key, loc(src, key)));
            let s = Str::from((strip_str(val), loc(src, val)));
            res.insert((*key).to_string(), FromItem::from((id, s)));
        }

        res
    }

    fn assert_from_clause(src: &str, kvs: &[(&'static str, &'static str)]) {
        assert_eq!(parse_from_clause(src).unwrap().inner, mk_inner(src, kvs));
    }

    #[test]
    fn test_from_clause() {
        let src = "from game='holdem', hero='AA'";

        assert_from_clause(src, &[("game", "'holdem'"), ("hero", "'AA'")]);
    }

    #[test]
    fn test_from_clause_norm_key() {
        let obj = parse_from_clause("from GAME=''").unwrap().inner;
        assert!(obj.contains_key("game"), "should use lowercase for keys");
        assert!(!obj.contains_key("GAME"));
    }

    #[test]
    fn test_values() {
        let obj = parse_from_clause("from hero='AA'").unwrap();
        assert_eq!(obj.get_game(), None);
        assert_eq!(obj.get_board_range(), None);
        assert_eq!(obj.get_dead(), None);
        //assert_eq!(obj.get_players(), &[("hero", "AA")]);
    }

    fn assert_err(src: &str, expected: Error) {
        assert_eq!(parse_from_clause(src).unwrap_err(), expected);
    }

    #[test]
    fn test_from_clause_dup_key() {
        let src = "from GAME='', game=''";
        assert_err(src, Error::DuplicatedKeyInFrom(loc(src, "game")));
    }

    #[test]
    fn test_debug() {
        let obj = parse_from_clause("from game='holdem', hero='AA'").unwrap();

        assert!(format!("{obj:?}").find(r#"hero": "AA"#).is_some());
    }
}
