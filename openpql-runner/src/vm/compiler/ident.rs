use super::*;

fn resolve_player(
    data: &VmStaticData,
    ident: &ast::Ident<'_>,
) -> PQLResult<VmStackValue> {
    with_loc(ident, || {
        data.find_player(ident.inner)
            .map_or(Err(PQLErrorKind::InvalidPlayer), |p| Ok(p.into()))
    })
}

fn resolve_ident<T>(ident: &ast::Ident) -> PQLResult<VmStackValue>
where
    T: FromStr,
    VmStackValue: From<T>,
    PQLErrorKind: From<<T as FromStr>::Err>,
{
    match ident.inner.parse::<T>() {
        Ok(v) => Ok(v.into()),
        Err(err) => Err(mk_err(ident, err)),
    }
}

pub fn push_ident(
    data: &mut CompilerData,
    ident: &ast::Ident,
    expected_type: PQLType,
) -> PQLResult<PQLType> {
    let (val, rtn_type) = match expected_type {
        PQLType::PLAYER => {
            (resolve_player(data.static_data, ident)?, PQLType::PLAYER)
        }
        PQLType::STREET => {
            (resolve_ident::<PQLStreet>(ident)?, PQLType::STREET)
        }
        PQLType::FLOPHANDCATEGORY => (
            resolve_ident::<PQLFlopHandCategory>(ident)?,
            PQLType::FLOPHANDCATEGORY,
        ),
        PQLType::HANDTYPE => {
            (resolve_ident::<PQLHandType>(ident)?, PQLType::HANDTYPE)
        }
        _ => {
            if let Ok(value) = resolve_ident::<PQLStreet>(ident)
                .or_else(|_| resolve_ident::<PQLFlopHandCategory>(ident))
                .or_else(|_| resolve_ident::<PQLHandType>(ident))
            {
                (value, PQLType::from(value))
            } else {
                return Err(mk_err(
                    ident,
                    PQLErrorKind::UnrecognizedIdentifier,
                ));
            }
        }
    };

    data.prog.push((val.into(), ident.loc));

    Ok(rtn_type)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    fn create_static_data() -> VmStaticData {
        let mut static_data = VmStaticData::default();
        static_data.player_names.push("p0".into());
        static_data.player_names.push("p1".into());

        static_data
    }

    fn assert_ident(
        type_hint: PQLType,
        src: &str,
        expected: VmStackValue,
        expected_type: PQLType,
    ) {
        let id = parse_ident(src).unwrap();

        let static_data = create_static_data();
        let mut data = CompilerData::new(&static_data);

        let tp = push_ident(&mut data, &id, type_hint).unwrap();

        assert_eq!(data.prog[0].0, VmInstruction::Push(expected));
        assert_eq!(tp, expected_type);
    }

    #[test]
    fn test_ident_with_type_hint() {
        assert_ident(
            PQLType::STREET,
            "flop",
            PQLStreet::Flop.into(),
            PQLType::STREET,
        );

        assert_ident(
            PQLType::FLOPHANDCATEGORY,
            "FLOPTOPPAIR",
            PQLFlopHandCategory::TopPair.into(),
            PQLType::FLOPHANDCATEGORY,
        );

        assert_ident(
            PQLType::HANDTYPE,
            "highcard",
            PQLHandType::HighCard.into(),
            PQLType::HANDTYPE,
        );

        assert_ident(
            PQLType::PLAYER,
            "p1",
            PQLPlayer::from(1).into(),
            PQLType::PLAYER,
        );
    }

    #[test]
    fn test_ident_without_type_hint() {
        assert_ident(
            PQLType::all(),
            "turn",
            PQLStreet::Turn.into(),
            PQLType::STREET,
        );

        assert_ident(
            PQLType::all(),
            "flopnothing",
            PQLFlopHandCategory::Nothing.into(),
            PQLType::FLOPHANDCATEGORY,
        );

        assert_ident(
            PQLType::all(),
            "pair",
            PQLHandType::Pair.into(),
            PQLType::HANDTYPE,
        );
    }

    fn assert_err<E>(expected_type: PQLType, src: &str, err: E)
    where
        PQLErrorKind: From<E>,
    {
        let expr = parse_ident(src).unwrap();

        let mut data = CompilerData::default();

        assert_eq!(
            push_ident(&mut data, &expr, expected_type),
            Err(mk_err(&expr, err))
        );
    }

    #[test]
    fn test_ident_error() {
        assert_err(PQLType::all(), "id", PQLErrorKind::UnrecognizedIdentifier);

        assert_err(
            PQLType::STREET,
            "invalid",
            ParseError::InvalidStreet("invalid".into()),
        );

        assert_err(
            PQLType::FLOPHANDCATEGORY,
            "invalid",
            ParseError::InvalidFlopHandCategory("invalid".into()),
        );

        assert_err(
            PQLType::HANDTYPE,
            "invalid",
            ParseError::InvalidHandType("invalid".into()),
        );

        assert_err(PQLType::PLAYER, "invalid", PQLErrorKind::InvalidPlayer);
    }
}
