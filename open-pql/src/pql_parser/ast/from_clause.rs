use super::*;

#[derive(PartialEq, Eq, Debug, Default)]
pub struct FromClause<'i> {
    pub inner: FxHashMap<String, FromItem<'i>>,
}

impl<'i> FromClause<'i> {
    pub(crate) fn new<T: IntoIterator<Item = FromItem<'i>>>(
        items: T,
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

        Ok(res)
    }

    pub(crate) fn get_pql_str(&self, key: &str) -> Option<&Str<'i>> {
        self.inner.get(key).map(|item| &item.value)
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
mod tests {
    use super::{super::super::parser::*, *};

    fn p(s: &str) -> FromClause {
        FromClauseParser::new().parse(s).unwrap()
    }

    fn e(s: &str) -> Error {
        FromClauseParser::new().parse(s).unwrap_err().into()
    }

    #[test]
    fn test_from_clause() {
        let obj = p("from game='holdem', hero='AA'");

        assert_eq!(obj.inner.keys().len(), 2);

        assert_eq!(
            obj.inner.get("game").unwrap(),
            &(("game", (5, 9)), ("holdem", (10, 18))).into(),
        );

        assert_eq!(
            obj.inner.get("hero").unwrap(),
            &(("hero", (20, 24)), ("AA", (25, 29))).into(),
        );
    }

    #[test]
    fn test_from_clause_norm_key() {
        assert!(
            p("from GAME=''").inner.contains_key("game"),
            "should normalize keys"
        );
        assert!(!p("from GAME=''").inner.contains_key("GAME"));
    }

    #[test]
    fn test_from_clause_dup_key() {
        assert_eq!(
            e("from GAME='', game=''"),
            Error::DuplicatedKeyInFrom((14, 18))
        );
    }
}
