use super::*;

#[derive(PartialEq, Eq, Debug)]
pub struct Stmt<'i> {
    pub selectors: Vec<Selector<'i>>,
    pub from: FromClause<'i>,
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
    pub fn new(
        selectors: Vec<Selector<'i>>,
        from: FromClause<'i>,
    ) -> ResultE<'i, Self> {
        ensure_uniq_names(&selectors)?;

        Ok(Self { selectors, from })
    }
}

#[cfg(test)]
mod tests {
    use super::{super::super::parser::*, *};

    fn s(s: &str) -> Stmt {
        StmtParser::new().parse(s).unwrap()
    }

    fn e(s: &str) -> Error {
        StmtParser::new().parse(s).unwrap_err().into()
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
}
