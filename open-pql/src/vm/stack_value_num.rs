use super::*;

#[derive(Debug, Clone, Copy, From, TryInto)]
pub enum VmStackValueNum {
    Double(PQLDouble),
    Long(PQLLong),
    CardCount(PQLCardCount),
}

impl VmStackValueNum {
    #[allow(clippy::cast_precision_loss)]
    pub fn cast_dbl(self) -> PQLDouble {
        match self {
            Self::Double(v) => v,
            Self::Long(v) => v as PQLDouble,
            Self::CardCount(v) => PQLDouble::from(v),
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn cast_int(self) -> PQLLong {
        match self {
            Self::Double(v) => v.floor() as PQLLong,
            Self::Long(v) => v,
            Self::CardCount(v) => PQLLong::from(v),
        }
    }
}

const EPSILON: PQLDouble = 1e-6;

fn float_eq(l: PQLDouble, r: PQLDouble) -> bool {
    (l - r).abs() <= EPSILON
}

impl PartialEq for VmStackValueNum {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (_, Self::Double(_)) | (Self::Double(_), _) => {
                float_eq(self.cast_dbl(), other.cast_dbl())
            }
            _ => self.cast_int() == other.cast_int(),
        }
    }
}

impl PartialOrd for VmStackValueNum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }

        match (self, other) {
            (_, Self::Double(_)) | (Self::Double(_), _) => {
                self.cast_dbl().partial_cmp(&other.cast_dbl())
            }
            _ => self.cast_int().partial_cmp(&other.cast_int()),
        }
    }
}

// using generics causes stackoverflow
macro_rules! impl_from {
    ($ty:ty) => {
        impl From<$ty> for VmStackValue {
            fn from(v: $ty) -> Self {
                VmStackValueNum::from(v).into()
            }
        }
    };
}

impl_from!(PQLCardCount);
impl_from!(PQLDouble);
impl_from!(PQLLong);

macro_rules! impl_try_into {
    ($ty:ty) => {
        impl TryFrom<VmStackValue> for $ty {
            type Error = PQLError;

            fn try_from(v: VmStackValue) -> Result<Self, Self::Error> {
                if let VmStackValue::Num(v) = v {
                    if let Ok(v) = v.try_into() {
                        return Ok(v);
                    }
                }

                Err(InternalError::BrokenStack.into())
            }
        }
    };
}

impl_try_into!(PQLCardCount);
impl_try_into!(PQLDouble);
impl_try_into!(PQLLong);

macro_rules! impl_op {
    ($ty:ty, $name:ident, $op:tt) => {
        impl $ty for VmStackValueNum {
            type Output = Self;

            fn $name(self, rhs: Self) -> Self::Output {
                match (self, rhs) {
                    (_, Self::Double(_)) | (Self::Double(_), _) => {
                        Self::from(self.cast_dbl() $op rhs.cast_dbl())
                    }
                    _ => Self::from(self.cast_int() $op rhs.cast_int()),
                }
            }
        }
    };
}

impl Div for VmStackValueNum {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::from(self.cast_dbl() / rhs.cast_dbl())
    }
}

impl_op!(Add, add, +);
impl_op!(Sub, sub, -);
impl_op!(Mul, mul, *);

#[cfg(test)]
mod tests {
    use VmStackValueNum::*;

    use super::*;
    use crate::*;

    impl Arbitrary for VmStackValueNum {
        #[allow(clippy::cast_precision_loss)]
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            match g.choose(&[0, 1, 2]).unwrap() {
                0 => Double(PQLLong::arbitrary(g) as PQLDouble / 10.0), // avoid NaN
                1 => Long(i32::arbitrary(g).into()), // avoid overflow
                _ => CardCount(PQLCardCount::arbitrary(g)),
            }
        }
    }

    #[test]
    fn test_eq() {
        let v = Double(100.0 + EPSILON / 10.0);

        assert_eq!(Double(1.0).cast_int(), 1);

        assert_eq!(v, Double(100.0));
        assert_eq!(v, Long(100));
        assert_eq!(v, CardCount(100));

        assert!(v <= Double(100.0));
        assert!(v <= Long(100));
        assert!(v <= CardCount(100));
        assert!(v >= Double(100.0));
        assert!(v >= Long(100));
        assert!(v >= CardCount(100));
    }

    #[test]
    fn test_cmp() {
        let v = Double(100.0);

        assert!(v >= Long(99));
        assert!(v > CardCount(99));
        assert!(v <= Long(101));
        assert!(v < CardCount(101));
        assert!(Long(99) < CardCount(101));
    }

    #[test]
    fn test_from_and_into() {
        assert_eq!(VmStackValue::Num(Long(0)), PQLLong::from(0).into());
        assert_eq!(Ok(0), PQLLong::try_from(VmStackValue::Num(Long(0))));
        assert!(PQLLong::try_from(VmStackValue::Rank(Some(Rank::R2))).is_err());
        assert!(PQLLong::try_from(VmStackValue::Num(Double(0.0))).is_err());
    }
}
