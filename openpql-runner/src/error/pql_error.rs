use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PQLError {
    pub loc: LocInfo,
    pub kind: PQLErrorKind,
}

impl<E> From<(LocInfo, E)> for PQLError
where
    PQLErrorKind: From<E>,
{
    fn from((loc, kind): (LocInfo, E)) -> Self {
        Self {
            loc,
            kind: PQLErrorKind::from(kind),
        }
    }
}
