use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PQLNumeric {
    Count(PQLCardCount),
    Long(PQLLong),
    Double(PQLDouble),
    Frac(PQLFraction),
}

const fn int_add(
    lhs: PQLLong,
    rhs: PQLLong,
) -> Result<PQLNumeric, RuntimeError> {
    match lhs.checked_add(rhs) {
        Some(v) => Ok(PQLNumeric::Long(v)),
        None => Err(RuntimeError::AddOverflow),
    }
}

const fn int_sub(
    lhs: PQLLong,
    rhs: PQLLong,
) -> Result<PQLNumeric, RuntimeError> {
    match lhs.checked_sub(rhs) {
        Some(v) => Ok(PQLNumeric::Long(v)),
        None => Err(RuntimeError::SubOverflow),
    }
}

const fn int_mul(
    lhs: PQLLong,
    rhs: PQLLong,
) -> Result<PQLNumeric, RuntimeError> {
    match lhs.checked_mul(rhs) {
        Some(v) => Ok(PQLNumeric::Long(v)),
        None => Err(RuntimeError::MulOverflow),
    }
}

#[allow(clippy::unnecessary_wraps)]
const fn dbl_add(
    lhs: PQLDouble,
    rhs: PQLDouble,
) -> Result<PQLNumeric, RuntimeError> {
    Ok(PQLNumeric::Double(lhs + rhs))
}

#[allow(clippy::unnecessary_wraps)]
const fn dbl_sub(
    lhs: PQLDouble,
    rhs: PQLDouble,
) -> Result<PQLNumeric, RuntimeError> {
    Ok(PQLNumeric::Double(lhs - rhs))
}

#[allow(clippy::unnecessary_wraps)]
const fn dbl_mul(
    lhs: PQLDouble,
    rhs: PQLDouble,
) -> Result<PQLNumeric, RuntimeError> {
    Ok(PQLNumeric::Double(lhs * rhs))
}

#[allow(clippy::unnecessary_wraps)]
const fn dbl_div(
    lhs: PQLDouble,
    rhs: PQLDouble,
) -> Result<PQLNumeric, RuntimeError> {
    Ok(PQLNumeric::Double(lhs / rhs))
}

impl PQLNumeric {
    pub fn try_add(self, other: Self) -> Result<Self, RuntimeError> {
        if self.is_int() && other.is_int() {
            int_add(self.to_int(), other.to_int())
        } else {
            dbl_add(self.to_dbl(), other.to_dbl())
        }
    }

    pub fn try_sub(self, other: Self) -> Result<Self, RuntimeError> {
        if self.is_int() && other.is_int() {
            int_sub(self.to_int(), other.to_int())
        } else {
            dbl_sub(self.to_dbl(), other.to_dbl())
        }
    }

    pub fn try_mul(self, other: Self) -> Result<Self, RuntimeError> {
        if self.is_int() && other.is_int() {
            int_mul(self.to_int(), other.to_int())
        } else {
            dbl_mul(self.to_dbl(), other.to_dbl())
        }
    }

    pub const fn try_div(self, other: Self) -> Result<Self, RuntimeError> {
        dbl_div(self.to_dbl(), other.to_dbl())
    }

    pub fn partial_compare(self, other: Self) -> Option<cmp::Ordering> {
        if self.is_int() && other.is_int() {
            Some(self.to_int().cmp(&other.to_int()))
        } else {
            self.to_dbl().partial_cmp(&other.to_dbl())
        }
    }

    const fn is_int(self) -> bool {
        matches!(self, Self::Count(_) | Self::Long(_))
    }

    fn to_int(self) -> PQLLong {
        match self {
            Self::Count(v) => PQLLong::from(v),
            Self::Long(v) => v,
            _ => unreachable!(),
        }
    }

    #[allow(clippy::cast_lossless)]
    #[allow(clippy::cast_precision_loss)]
    pub const fn to_dbl(self) -> PQLDouble {
        match self {
            Self::Count(v) => v as PQLDouble,
            Self::Long(v) => v as PQLDouble,
            Self::Double(v) => v,
            Self::Frac(v) => v.to_double(),
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
pub mod tests {
    use super::*;
    use crate::*;

    fn cnt_(v: PQLCardCount) -> PQLNumeric {
        PQLNumeric::Count(v)
    }

    fn long(v: PQLLong) -> PQLNumeric {
        PQLNumeric::Long(v)
    }

    fn dbl_(v: i8) -> PQLNumeric {
        PQLNumeric::Double(PQLDouble::from(v))
    }

    fn frac(v: FractionInner) -> PQLNumeric {
        PQLNumeric::Frac(PQLFraction::new(v, 1))
    }

    #[test]
    fn test_add() {
        let op = PQLNumeric::try_add;
        assert_eq!(op(cnt_(1), cnt_(2)), Ok(long(3)));
        assert_eq!(op(cnt_(1), long(2)), Ok(long(3)));
        assert_eq!(op(cnt_(1), frac(2)), Ok(dbl_(3)));
        assert_eq!(op(cnt_(1), dbl_(2)), Ok(dbl_(3)));

        assert_eq!(op(long(1), long(2)), Ok(long(3)));
        assert_eq!(op(long(1), frac(2)), Ok(dbl_(3)));
        assert_eq!(op(long(1), dbl_(2)), Ok(dbl_(3)));

        assert_eq!(op(frac(1), frac(2)), Ok(dbl_(3)));
        assert_eq!(op(frac(1), dbl_(2)), Ok(dbl_(3)));

        assert_eq!(op(dbl_(1), dbl_(2)), Ok(dbl_(3)));
    }

    #[test]
    fn test_sub() {
        let op = PQLNumeric::try_sub;
        assert_eq!(op(cnt_(1), cnt_(2)), Ok(long(-1)));
        assert_eq!(op(cnt_(1), long(2)), Ok(long(-1)));
        assert_eq!(op(cnt_(1), frac(2)), Ok(dbl_(-1)));
        assert_eq!(op(cnt_(1), dbl_(2)), Ok(dbl_(-1)));

        assert_eq!(op(long(1), long(2)), Ok(long(-1)));
        assert_eq!(op(long(1), frac(2)), Ok(dbl_(-1)));
        assert_eq!(op(long(1), dbl_(2)), Ok(dbl_(-1)));

        assert_eq!(op(frac(1), frac(2)), Ok(dbl_(-1)));
        assert_eq!(op(frac(1), dbl_(2)), Ok(dbl_(-1)));

        assert_eq!(op(dbl_(1), dbl_(2)), Ok(dbl_(-1)));
    }

    #[test]
    fn test_mul() {
        let op = PQLNumeric::try_mul;
        assert_eq!(op(cnt_(1), cnt_(2)), Ok(long(2)));
        assert_eq!(op(cnt_(1), long(2)), Ok(long(2)));
        assert_eq!(op(cnt_(1), frac(2)), Ok(dbl_(2)));
        assert_eq!(op(cnt_(1), dbl_(2)), Ok(dbl_(2)));

        assert_eq!(op(long(1), long(2)), Ok(long(2)));
        assert_eq!(op(long(1), frac(2)), Ok(dbl_(2)));
        assert_eq!(op(long(1), dbl_(2)), Ok(dbl_(2)));

        assert_eq!(op(frac(1), frac(2)), Ok(dbl_(2)));
        assert_eq!(op(frac(1), dbl_(2)), Ok(dbl_(2)));

        assert_eq!(op(dbl_(1), dbl_(2)), Ok(dbl_(2)));
    }

    #[test]
    fn test_div() {
        let op = PQLNumeric::try_div;
        let half = PQLNumeric::Double(0.5);
        assert_eq!(op(cnt_(1), cnt_(2)), Ok(half));
        assert_eq!(op(cnt_(1), long(2)), Ok(half));
        assert_eq!(op(cnt_(1), frac(2)), Ok(half));
        assert_eq!(op(cnt_(1), dbl_(2)), Ok(half));

        assert_eq!(op(long(1), long(2)), Ok(half));
        assert_eq!(op(long(1), frac(2)), Ok(half));
        assert_eq!(op(long(1), dbl_(2)), Ok(half));

        assert_eq!(op(frac(1), frac(2)), Ok(half));
        assert_eq!(op(frac(1), dbl_(2)), Ok(half));

        assert_eq!(op(dbl_(1), dbl_(2)), Ok(half));
    }

    #[test]
    fn test_err() {
        assert_eq!(
            long(PQLLong::MAX).try_add(cnt_(1)),
            Err(RuntimeError::AddOverflow)
        );

        assert_eq!(
            long(PQLLong::MIN).try_sub(cnt_(1)),
            Err(RuntimeError::SubOverflow)
        );

        assert_eq!(
            long(PQLLong::MIN).try_mul(cnt_(2)),
            Err(RuntimeError::MulOverflow)
        );
    }

    #[test]
    #[should_panic(expected = "")]
    fn test_internal() {
        dbl_(1).to_int();
    }
}
