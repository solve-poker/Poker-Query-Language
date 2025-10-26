use super::*;

pub type SourceLocation = (usize, usize);

pub trait HasSourceLocation {
    fn loc(&self) -> SourceLocation;
}

impl HasSourceLocation for &ast::Ident<'_> {
    fn loc(&self) -> SourceLocation {
        self.loc
    }
}

impl HasSourceLocation for &ast::Str<'_> {
    fn loc(&self) -> SourceLocation {
        self.loc
    }
}

impl HasSourceLocation for &ast::Num {
    fn loc(&self) -> SourceLocation {
        self.loc
    }
}

impl HasSourceLocation for &ast::FnCall<'_> {
    fn loc(&self) -> SourceLocation {
        self.loc
    }
}

impl HasSourceLocation for &ast::Expr<'_> {
    fn loc(&self) -> SourceLocation {
        ast::Expr::loc(self)
    }
}

impl HasSourceLocation for &SourceLocation {
    fn loc(&self) -> SourceLocation {
        (self.0, self.1)
    }
}

#[inline]
pub fn with_loc<T, I, F, E>(expr: &I, proc: F) -> PQLResult<T>
where
    for<'a> &'a I: HasSourceLocation,
    F: Fn() -> Result<T, E>,
    PQLErrorKind: From<E>,
{
    match proc() {
        Ok(v) => Ok(v),
        Err(err) => Err(PQLError {
            loc: expr.loc(),
            kind: err.into(),
        }),
    }
}
