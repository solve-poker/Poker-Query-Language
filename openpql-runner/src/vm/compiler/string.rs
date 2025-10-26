use super::*;

fn resolve_range<'vm, T>(
    game: PQLGame,
    string: &'vm ast::Str,
) -> PQLResult<VmHeapValue>
where
    T: TryFrom<(PQLGame, &'vm str)>,
    VmHeapValue: From<T>,
    PQLErrorKind: From<<T as TryFrom<(PQLGame, &'vm str)>>::Error>,
{
    match T::try_from((game, string.inner)) {
        Ok(range) => Ok(range.into()),
        Err(err) => Err(mk_err(string, err)),
    }
}

pub fn push_str(
    data: &mut CompilerData,
    string: &ast::Str,
    expected_type: PQLType,
) -> PQLResult<PQLType> {
    let (value, rtn_type) = match expected_type {
        PQLType::RANGE => (
            resolve_range::<PQLRange>(data.static_data.game, string)?,
            PQLType::RANGE,
        ),
        PQLType::BOARDRANGE => (
            resolve_range::<PQLBoardRange>(data.static_data.game, string)?,
            PQLType::BOARDRANGE,
        ),
        _ => (string.inner.to_string().into(), PQLType::STRING),
    };

    let idx = data.heap.len();
    data.heap.push(value);

    data.prog
        .push((VmInstruction::Push(idx.into()), string.loc));

    Ok(rtn_type)
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    fn assert_str(
        type_hint: PQLType,
        src: &str,
        expected: VmHeapValue,
        expected_type: PQLType,
    ) {
        let expr = parse_str(src).unwrap();

        let mut data = CompilerData::default();

        let tp = push_str(&mut data, &expr, type_hint).unwrap();

        assert_eq!(data.prog[0].0, VmInstruction::Push(sval!(@ref 0)));
        assert!(data.heap[0].is_eq(&expected));
        assert_eq!(tp, expected_type);
    }

    #[test]
    fn test_str() {
        assert_str(
            PQLType::all(),
            "'str'",
            VmHeapValue::Str("str".to_string()),
            PQLType::STRING,
        );

        assert_str(
            PQLType::STRING,
            "'str'",
            VmHeapValue::Str("str".to_string()),
            PQLType::STRING,
        );
    }

    #[test]
    fn test_range() {
        assert_str(
            PQLType::RANGE,
            "'AA'",
            VmHeapValue::Range((PQLGame::default(), "AA").try_into().unwrap()),
            PQLType::RANGE,
        );
    }

    #[test]
    fn test_board_range() {
        assert_str(
            PQLType::BOARDRANGE,
            "'AA'",
            VmHeapValue::BoardRange(
                (PQLGame::default(), "AA").try_into().unwrap(),
            ),
            PQLType::BOARDRANGE,
        );
    }

    fn assert_err<E>(expected_type: PQLType, src: &str, err: E)
    where
        PQLErrorKind: From<E>,
    {
        let expr = parse_str(src).unwrap();

        let mut data = CompilerData::default();

        assert_eq!(
            push_str(&mut data, &expr, expected_type),
            Err(mk_err(&expr, err))
        );
    }

    #[test]
    fn test_str_err() {
        assert_err(PQLType::RANGE, "'[N]'", RangeError::InvalidList((0, 3)));
        assert_err(
            PQLType::BOARDRANGE,
            "'[N]'",
            RangeError::InvalidList((0, 3)),
        );
    }
}
