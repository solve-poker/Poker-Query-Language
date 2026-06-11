use super::{Error, LalrError, Loc, LocInfo, NumValueFloat, NumValueInt, Spanned, str};

impl Spanned for Num {
    fn loc(&self) -> LocInfo {
        self.loc
    }
}

/// Numeric literal with its parsed value and source span.
#[derive(Clone, PartialEq, derive_more::From, derive_more::Debug)]
#[debug("{}", self.inner)]
pub struct Num {
    /// Parsed numeric value.
    pub inner: NumValue,
    /// Source span of the literal.
    pub loc: (Loc, Loc),
}

impl From<(NumValueFloat, (Loc, Loc))> for Num {
    fn from((val, loc): (NumValueFloat, (Loc, Loc))) -> Self {
        Self {
            inner: val.into(),
            loc,
        }
    }
}

impl From<(NumValueInt, (Loc, Loc))> for Num {
    fn from((val, loc): (NumValueInt, (Loc, Loc))) -> Self {
        Self {
            inner: val.into(),
            loc,
        }
    }
}

/// # Panics
/// float parse won't fail /-?(\d+)?\.\d+/
/// <https://doc.rust-lang.org/std/primitive.f64.html#method.from_str>
impl<'input> TryFrom<(&'input str, (Loc, Loc), bool)> for Num {
    type Error = LalrError<'input>;

    fn try_from(
        (src, loc, is_float): (&'input str, (Loc, Loc), bool),
    ) -> Result<Self, Self::Error> {
        if is_float {
            Ok((src.parse::<NumValueFloat>().unwrap(), loc).into())
        } else {
            src.parse::<NumValueInt>().map_or_else(
                |_| Err(Error::InvalidNumericValue(loc).into()),
                |v| Ok((v, loc).into()),
            )
        }
    }
}

/// Parsed numeric value, either integer or floating-point.
#[derive(Clone, Copy, Debug, PartialEq, derive_more::From, derive_more::Display)]
pub enum NumValue {
    /// Integer value.
    #[display("{_0}")]
    Int(NumValueInt),
    /// Floating-point value.
    #[display("{_0}")]
    Float(NumValueFloat),
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::*;

    fn assert_num<T>(src: &str, expected: T)
    where
        NumValue: From<T>,
    {
        let loc_start = 0;
        let loc_end = src.len();
        assert_eq!(
            parse_num(src),
            Ok((NumValue::from(expected), (loc_start, loc_end)).into())
        );
    }

    #[test]
    fn test_num() {
        assert_num("0", 0);
        assert_num("-1", -1);
        assert_num("-1.5", -1.5);
        assert_num("-.5", -0.5);
        assert_num(".5", 0.5);
    }

    #[test]
    fn test_err() {
        let toobig = format!("{}0", NumValueInt::MAX);
        assert_eq!(
            parse_num(&toobig),
            Err(Error::InvalidNumericValue((0, toobig.len())))
        );
    }

    #[test]
    fn test_dbg() {
        assert_eq!(format!("{:?}", Num::from((-123, (0, 1)))), "-123");
    }

    #[test]
    fn test_loc() {
        let n = Num::from((-1, (4, 6)));
        assert_eq!(n.loc(), (4, 6));
    }
}
