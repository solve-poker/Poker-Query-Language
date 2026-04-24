use super::{Error, Expr, FromClause, FxHashSet, ResultE, Selector, user_err};

/// A single `select ... from ... [where ...]` statement.
#[derive(PartialEq, Debug)]
pub struct Stmt<'i> {
    /// Aggregate selectors in the `select` list.
    pub selectors: Vec<Selector<'i>>,
    /// `from` clause.
    pub from: FromClause<'i>,
    /// Optional `where` predicate.
    pub where_clause: Option<Expr<'i>>,
}

fn ensure_uniq_names<'i>(selectors: &[Selector]) -> ResultE<'i, ()> {
    let mut used = FxHashSet::default();

    for selector in selectors {
        if let Some(id) = &selector.alias
            && !used.insert(id.inner.to_ascii_lowercase())
        {
            return Err(user_err(Error::DuplicatedSelectorName(id.loc)));
        }
    }

    Ok(())
}

impl<'i> Stmt<'i> {
    /// Builds a statement, rejecting duplicate selector aliases.
    pub fn new(
        selectors: Vec<Selector<'i>>,
        from: FromClause<'i>,
        where_clause: Option<Expr<'i>>,
    ) -> ResultE<'i, Self> {
        ensure_uniq_names(&selectors)?;

        Ok(Self {
            selectors,
            from,
            where_clause,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    fn s(s: &str) -> Stmt<'_> {
        parser::StmtParser::new().parse(s).unwrap()
    }

    fn e(s: &str) -> Error {
        parser::StmtParser::new().parse(s).unwrap_err().into()
    }

    #[test]
    fn test_stmt() {
        assert_eq!(s("select avg(_)  from _=''").selectors.len(), 1);
        assert_eq!(s("select avg(_), from _=''").selectors.len(), 1);

        assert_eq!(
            s("select avg(_) as s1 from _=''").selectors[0].alias,
            Some(("s1", (17, 19)).into())
        );

        assert_eq!(
            e("select avg(_) as s1, avg(_) as s1 from _=''"),
            Error::DuplicatedSelectorName((31, 33))
        );
    }

    #[test]
    fn test_stmt_where_absent() {
        let stmt = s("select count(_) from _=''");
        assert!(stmt.where_clause.is_none());
    }

    #[test]
    fn test_stmt_where_present() {
        let stmt = s("select count(_) from _='' where 1 = 1");
        assert!(stmt.where_clause.is_some());
    }

    #[test]
    fn test_stmt_where_logical() {
        let _ = s("select count(_) from _='' where 1 = 1 and 2 = 2");
        let _ = s("select count(_) from _='' where 1 = 1 or 2 = 2");
        let _ = s("select count(_) from _='' where not 1 = 2");
        let _ = s("select count(_) from _='' where not 1 = 2 and 1 = 1");
    }
}
