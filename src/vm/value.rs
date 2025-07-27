use super::*;

#[derive(Debug, Clone, From, TryInto)]
pub enum VmValue {
    Thin(VmStackValue),
    Range(PQLRange),
    BoardRange(PQLBoardRange),
    Str(PQLString),
}

mod try_from {
    use VmValue::{BoardRange, Range, Str, Thin};

    use super::*;

    macro_rules! impl_try_from {
        ($to:ty, $pattern:pat => $value:expr) => {
            impl<'a> TryFrom<&'a VmValue> for &'a $to {
                type Error = PQLError;

                fn try_from(value: &'a VmValue) -> Result<Self, Self::Error> {
                    match value {
                        $pattern => Ok($value),
                        _ => Err(InternalError::InvalidVmValue.into()),
                    }
                }
            }

            impl<'a> TryFrom<&'a mut VmValue> for &'a mut $to {
                type Error = PQLError;

                fn try_from(
                    value: &'a mut VmValue,
                ) -> Result<Self, Self::Error> {
                    match value {
                        $pattern => Ok($value),
                        _ => Err(InternalError::InvalidVmValue.into()),
                    }
                }
            }
        };
    }

    impl_try_from!(VmStackValue, Thin(v) => v);
    impl_try_from!(VmStackValueNum, Thin(VmStackValue::Num(v)) => v);

    impl_try_from!(PQLRange, Range(r) => r);
    impl_try_from!(PQLBoardRange, BoardRange(r) => r);
    impl_try_from!(PQLString, Str(r) => r);

    impl_try_from!(PQLLong, Thin(VmStackValue::Num(VmStackValueNum::Long(v))) => v);
    impl_try_from!(PQLDouble, Thin(VmStackValue::Num(VmStackValueNum::Double(v))) => v);
    impl_try_from!(PQLCardCount, Thin(VmStackValue::Num(VmStackValueNum::CardCount(v))) => v);

    impl_try_from!(PQLHandType, Thin(VmStackValue::HandType(v)) => v);
    impl_try_from!(PQLFlopHandCategory, Thin(VmStackValue::FlopCategory(v)) => v);
    impl_try_from!(PQLHiRating, Thin(VmStackValue::Rating(v)) => v);
}

#[cfg_attr(coverage_nightly, coverage(off))]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_try_from() {
        fn assert_ok<T>(v: &mut VmValue)
        where
            for<'a> &'a T: TryFrom<&'a VmValue, Error = PQLError>,
            for<'a> &'a mut T: TryFrom<&'a mut VmValue, Error = PQLError>,
        {
            assert!(<&T>::try_from(&*v).is_ok());
            assert!(<&mut T>::try_from(v).is_ok());
        }
        fn assert_err<T>(v: &mut VmValue)
        where
            for<'a> &'a T: TryFrom<&'a VmValue, Error = PQLError>,
            for<'a> &'a mut T: TryFrom<&'a mut VmValue, Error = PQLError>,
        {
            assert!(<&T>::try_from(&*v).is_err());
            assert!(<&mut T>::try_from(v).is_err());
        }

        let mut v1 = VmValue::Thin(VmStackValue::from(1 as PQLLong));
        let mut v2 = VmValue::Thin(VmStackValue::from(0.0));
        let mut v3 = VmValue::Str(PQLString::from(""));
        let mut v4 = VmValue::Range(PQLRange::default());
        let mut v5 = VmValue::BoardRange(PQLBoardRange::default());

        assert_ok::<VmStackValue>(&mut v1);
        assert_ok::<VmStackValueNum>(&mut v1);
        assert_ok::<PQLLong>(&mut v1);
        assert_ok::<PQLDouble>(&mut v2);
        assert_ok::<PQLString>(&mut v3);
        assert_ok::<PQLRange>(&mut v4);
        assert_ok::<PQLBoardRange>(&mut v5);

        assert_err::<VmStackValue>(&mut v5);
        assert_err::<VmStackValueNum>(&mut v5);
        assert_err::<PQLLong>(&mut v5);
        assert_err::<PQLDouble>(&mut v5);
        assert_err::<PQLString>(&mut v5);
        assert_err::<PQLRange>(&mut v5);
        assert_err::<PQLBoardRange>(&mut v4);
    }
}
