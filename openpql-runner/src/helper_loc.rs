use super::*;

#[inline]
pub fn with_loc<T, I, F, E>(expr: &I, f: F) -> PQLResult<T>
where
    I: Spanned,
    F: FnOnce() -> Result<T, E>,
    PQLErrorKind: From<E>,
{
    f().map_err(|err| PQLError {
        loc: expr.loc(),
        kind: err.into(),
    })
}
