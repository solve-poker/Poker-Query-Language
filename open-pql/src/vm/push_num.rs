use super::*;

pub(super) fn push_num(
    i: &ast::Num,
    instructions: &mut VmInstructions,
    _store: &mut VmStore,
    deps: &InitDeps,
    expected_type: Option<PQLType>,
) -> Result<PQLType, PQLError> {
    let (val, kind) = get_val(i, expected_type, deps)?;

    instructions.push(val.into());

    Ok(kind)
}

fn parse_i(i: &ast::Num) -> Result<VmStackValue, PQLError> {
    match i.inner.parse::<PQLInteger>() {
        Ok(v) => Ok(v.into()),
        Err(e) => Err(PQLError::ParseIntError(i.loc, e)),
    }
}

fn parse_c(i: &ast::Num) -> Result<VmStackValue, PQLError> {
    match i.inner.parse::<PQLCardCount>() {
        Ok(v) => Ok(v.into()),
        Err(e) => Err(PQLError::ParseIntError(i.loc, e)),
    }
}

/// # Panics
/// "-?(\d+)?\.\d+" garanteed by grammar
fn parse_f(i: &ast::Num) -> VmStackValue {
    i.inner.parse::<PQLDouble>().unwrap().into()
}

fn get_val(
    i: &ast::Num,
    expected_type: Option<PQLType>,
    _deps: &InitDeps,
) -> Result<(VmStackValue, PQLType), PQLError> {
    expected_type.map_or_else(
        || {
            if i.is_float {
                Ok((parse_f(i), PQLType::Double))
            } else {
                Ok((parse_i(i)?, PQLType::Integer))
            }
        },
        |kind| match kind {
            PQLType::Double => Ok((parse_f(i), PQLType::Double)),
            PQLType::Integer => Ok((parse_i(i)?, PQLType::Integer)),
            PQLType::CardCount => Ok((parse_c(i)?, PQLType::CardCount)),

            _ => Err((i.loc, TypeError::TypeMismatch(kind)).into()),
        },
    )
}

#[cfg(test)]
mod tests {
    use std::{cmp, fmt};

    use pql_parser::parser::*;

    use super::*;
    use crate::*;

    fn n(s: &str) -> ast::Num {
        NumParser::new().parse(s).unwrap()
    }

    #[test]
    fn test_error() {
        let deps = InitDeps::default();
        let i = n("1");
        let t = PQLType::Rank;

        assert_eq!(
            PQLError::from((i.loc, TypeError::TypeMismatch(t))),
            get_val(&i, Some(t), &deps).unwrap_err(),
        );

        let mut i = n("1");
        i.inner = "invalid";
        let t = PQLType::Integer;

        assert!(matches!(
            get_val(&i, Some(t), &deps).unwrap_err(),
            PQLError::ParseIntError(_, _)
        ));
        assert!(matches!(
            get_val(&i, None, &deps).unwrap_err(),
            PQLError::ParseIntError(_, _)
        ));
    }

    #[test]
    fn test_cardcount_overflow() {
        let deps = InitDeps::default();
        let i = n("256");

        assert!(matches!(
            get_val(&i, Some(PQLType::CardCount), &deps).unwrap_err(),
            PQLError::ParseIntError(_, _)
        ));
    }

    fn assert_num<T>(
        int: &str,
        t: Option<PQLType>,
        expected: T,
        expected_type: PQLType,
    ) where
        T: TryFrom<VmStackValue> + fmt::Debug + cmp::PartialEq,
        T::Error: fmt::Debug,
    {
        let (stack_val, kind) =
            get_val(&n(int), t, &InitDeps::default()).unwrap();

        assert_eq!(expected, stack_val.try_into().unwrap());
        assert_eq!(expected_type, kind);
    }

    fn assert_with_ty<T>(int: &str, expected: T, expected_type: PQLType)
    where
        T: TryFrom<VmStackValue> + fmt::Debug + cmp::PartialEq,
        T::Error: fmt::Debug,
    {
        assert_num(int, Some(expected_type), expected, expected_type);
    }

    fn assert_without_ty<T>(int: &str, expected: T, expected_type: PQLType)
    where
        T: TryFrom<VmStackValue> + fmt::Debug + cmp::PartialEq,
        T::Error: fmt::Debug,
    {
        assert_num(int, None, expected, expected_type);
    }

    #[test]
    fn test_push_num_long() {
        assert_with_ty::<PQLInteger>("10", 10, PQLType::Integer);
        assert_without_ty::<PQLInteger>("10", 10, PQLType::Integer);
    }

    #[test]
    fn test_push_num_cardcount() {
        assert_with_ty::<PQLCardCount>("10", 10, PQLType::CardCount);
    }

    #[test]
    fn test_push_num_double() {
        assert_with_ty::<PQLDouble>("1.0", 1.0, PQLType::Double);
        assert_without_ty::<PQLDouble>("1.0", 1.0, PQLType::Double);
    }
}
