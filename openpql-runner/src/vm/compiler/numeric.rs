use super::*;

pub fn push_num(
    data: &mut CompilerData,
    num: &ast::Num,
    expected_type: PQLType,
) -> PQLResult<PQLType> {
    let (value, rtn_type) = match (expected_type, num.inner) {
        (PQLType::CARDCOUNT, ast::NumValue::Int(long)) => {
            match PQLCardCount::try_from(long) {
                Ok(v) => (v.into(), PQLType::CARDCOUNT),
                Err(_) => {
                    return Err(mk_err(num, PQLErrorKind::InvalidCardCount));
                }
            }
        }
        (_, ast::NumValue::Int(long)) => (long.into(), PQLType::LONG),
        (_, ast::NumValue::Float(float)) => (float.into(), PQLType::DOUBLE),
    };

    data.prog.push((VmInstruction::Push(value), num.loc));

    Ok(rtn_type)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    fn assert_num(
        type_hint: PQLType,
        src: &str,
        expected: VmStackValue,
        expected_type: PQLType,
    ) {
        let expr = parse_num(src).unwrap();

        let mut data = CompilerData::default();

        let tp = push_num(&mut data, &expr, type_hint).unwrap();

        assert_eq!(data.prog[0].0, VmInstruction::Push(expected));
        assert_eq!(tp, expected_type);
    }

    fn assert_err<E>(expected_type: PQLType, src: &str, err: E)
    where
        PQLErrorKind: From<E>,
    {
        let expr = parse_num(src).unwrap();

        let mut data = CompilerData::default();

        assert_eq!(
            push_num(&mut data, &expr, expected_type),
            Err(mk_err(&expr, err))
        );
    }

    #[test]
    fn test_num_with_type_hint() {
        assert_num(
            PQLType::CARDCOUNT,
            "1",
            sval!(@count 1),
            PQLType::CARDCOUNT,
        );
        assert_num(PQLType::LONG, "-1", sval!(@long -1), PQLType::LONG);
        assert_num(PQLType::DOUBLE, "0.1", 0.1.into(), PQLType::DOUBLE);
    }

    #[test]
    fn test_num_without_type_hint() {
        assert_num(PQLType::all(), "1", sval!(@long 1), PQLType::LONG);
        assert_num(PQLType::all(), "-1", sval!(@long -1), PQLType::LONG);
        assert_num(PQLType::all(), "0.1", 0.1.into(), PQLType::DOUBLE);
    }

    #[test]
    fn test_num_err() {
        let toobig = "256";
        assert_err(PQLType::CARDCOUNT, toobig, PQLErrorKind::InvalidCardCount);
    }
}
