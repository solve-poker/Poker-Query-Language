use super::*;

pub(super) fn push_ident(
    id: &ast::Ident,
    instructions: &mut VmInstructions,
    _store: &mut VmStore,
    deps: &InitDeps,
    expected_type: Option<PQLType>,
) -> Result<PQLType, PQLError> {
    let (val, kind) = get_val(id, expected_type, deps)?;

    instructions.push(val.into());

    Ok(kind)
}

fn get_val_without_type(
    id: &ast::Ident,
    deps: &InitDeps,
) -> Result<(VmStackValue, PQLType), PQLError> {
    if let Some(v) = parse_id::<PQLFlopHandCategory, _>((id.inner, deps.game)) {
        return Ok((v, PQLType::FlopHandCategory));
    }

    if let Some(v) = parse_id::<PQLHandType, _>((id.inner, deps.game)) {
        return Ok((v, PQLType::HandType));
    }

    if let Some(v) = parse_id::<PQLStreet, _>(id.inner) {
        return Ok((v, PQLType::Street));
    }

    Err(PQLError::UnknownIdent(id.loc))
}

fn parse_id<S, T>(v: T) -> Option<VmStackValue>
where
    S: TryFrom<T, Error = ParseError>,
    VmStackValue: From<S>,
{
    S::try_from(v).map_or(None, |v| Some(v.into()))
}

fn get_val(
    id: &ast::Ident,
    kind: Option<PQLType>,
    deps: &InitDeps,
) -> Result<(VmStackValue, PQLType), PQLError> {
    kind.map_or_else(
        || get_val_without_type(id, deps),
        |kind| {
            match kind {
                PQLType::FlopHandCategory => {
                    parse_id::<PQLFlopHandCategory, _>((id.inner, deps.game))
                }
                PQLType::HandType => {
                    parse_id::<PQLHandType, _>((id.inner, deps.game))
                }
                PQLType::Street => parse_id::<PQLStreet, _>(id.inner),
                PQLType::Player => {
                    parse_id::<PQLPlayer, _>((id.inner, deps.player_names))
                }
                _ => return Err((id.loc, TypeError::TypeMismatch(kind)).into()),
            }
            .map_or_else(
                || Err((id.loc, TypeError::InvalidIdent(kind)).into()),
                |v| Ok((v, kind)),
            )
        },
    )
}

#[cfg(test)]
mod tests {
    use std::{cmp, fmt};

    use pql_parser::parser::*;

    use super::*;
    use crate::*;

    fn i(s: &str) -> ast::Ident {
        IdentParser::new().parse(s).unwrap()
    }

    #[test]
    fn test_ident_error() {
        let deps = InitDeps::default();

        let invalid_id = i("invalid_id");
        let loc = (0, 10);

        assert_eq!(
            PQLError::UnknownIdent(loc),
            get_val(&invalid_id, None, &deps).unwrap_err(),
        );

        let types = [
            PQLType::Street,
            PQLType::FlopHandCategory,
            PQLType::HandType,
            PQLType::Player,
        ];

        for t in types {
            assert_eq!(
                PQLError::from((loc, TypeError::InvalidIdent(t))),
                get_val(&invalid_id, Some(t), &deps).unwrap_err(),
            );
        }

        assert_eq!(
            PQLError::from((loc, TypeError::TypeMismatch(PQLType::Rank))),
            get_val(&invalid_id, Some(PQLType::Rank), &deps).unwrap_err(),
        );
    }

    #[test]
    fn test_push_ident() {
        const fn make_deps() -> InitDeps<'static, 'static> {
            InitDeps {
                game: PQLGame::Holdem,
                player_names: &["hero", "villan"],
            }
        }

        fn assert_ident<T>(
            id: &str,
            t: Option<PQLType>,
            expected: T,
            expected_type: PQLType,
        ) where
            T: TryFrom<VmStackValue> + fmt::Debug + cmp::PartialEq,
            T::Error: fmt::Debug,
        {
            let deps = make_deps();

            let (stack_val, kind) = get_val(&i(id), t, &deps).unwrap();

            assert_eq!(expected, stack_val.try_into().unwrap());
            assert_eq!(expected_type, kind);
        }

        fn assert_ident_with_and_without_type<T>(
            id: &str,
            expected: T,
            expected_type: PQLType,
        ) where
            T: TryFrom<VmStackValue> + fmt::Debug + cmp::PartialEq + Clone,
            T::Error: fmt::Debug,
        {
            assert_ident(id, None, expected.clone(), expected_type);
            assert_ident(id, Some(expected_type), expected, expected_type);
        }

        assert_ident_with_and_without_type(
            "flopnothing",
            PQLFlopHandCategory::from((
                FlopHandCategory::Nothing,
                PQLGame::default(),
            )),
            PQLType::FlopHandCategory,
        );

        assert_ident_with_and_without_type(
            "highcard",
            PQLHandType::from((HandType::HighCard, PQLGame::default())),
            PQLType::HandType,
        );

        assert_ident_with_and_without_type(
            "flop",
            PQLStreet::Flop,
            PQLType::Street,
        );

        assert_ident(
            "hero",
            Some(PQLType::Player),
            PQLPlayer::from(0),
            PQLType::Player,
        );
    }
}
