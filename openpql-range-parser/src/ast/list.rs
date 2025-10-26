use super::{
    Display, Error, LalrError, Loc, RangeCard, RankConst, SuitConst, ToString,
};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Display)]
pub enum ListElem {
    #[display("{_0}{_1}")]
    CC(RankConst, SuitConst),
    #[display("{_0}")]
    CA(RankConst),
    #[display("{_0}")]
    AC(SuitConst),
}

fn to_str(elems: &[ListElem]) -> String {
    format!(
        "[{}]",
        elems
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(",")
    )
}

#[derive(Clone, PartialEq, Eq, Debug, derive_more::From, Display)]
#[display("{}", to_str(_0))]
pub struct List(pub Vec<ListElem>);

impl TryFrom<(Loc, Vec<RangeCard>, Loc)> for List {
    type Error = LalrError<'static>;

    fn try_from(
        (l, v, r): (Loc, Vec<RangeCard>, Loc),
    ) -> Result<Self, Self::Error> {
        let mut inner = vec![];

        for c in v {
            match c {
                RangeCard::CC(r, s) => {
                    inner.push(ListElem::CC(r, s));
                }
                RangeCard::CA(r) => {
                    inner.push(ListElem::CA(r));
                }
                RangeCard::AC(s) => {
                    inner.push(ListElem::AC(s));
                }
                _ => {
                    return Err(Error::InvalidList((l, r)).into());
                }
            }
        }

        Ok(Self(inner))
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    fn assert_list(src: &str, expected: &str) {
        let list = parse_list(src).unwrap();

        assert_eq!(list.to_string(), expected, "{src} != {expected}");
    }

    #[quickcheck]
    fn test_list(c: RangeCard) {
        let src = format!("[{c}]");
        let res = parse_list(&src);

        let is_err = match c {
            RangeCard::CC(_, _) | RangeCard::CA(_) | RangeCard::AC(_) => false,
            RangeCard::CV(_, _)
            | RangeCard::VC(_, _)
            | RangeCard::VV(_, _)
            | RangeCard::VA(_)
            | RangeCard::AV(_)
            | RangeCard::AA => true,
        };

        if is_err {
            assert_eq!(res, Err(Error::InvalidList((0, src.len())).into()));
        } else {
            assert_eq!(res, Ok((0, vec![c], 0).try_into().unwrap()));
        }
    }

    #[test]
    fn test_list_ok() {
        assert_list("[A, s]", "[A,s]");
    }

    #[test]
    fn test_list_error() {
        assert_eq!(parse_list("[B] "), Err(Error::InvalidList((0, 3)).into()));
        assert_eq!(parse_list("[Bs]"), Err(Error::InvalidList((0, 4)).into()));
        assert_eq!(parse_list("[*w]"), Err(Error::InvalidList((0, 4)).into()));
        assert_eq!(parse_list("[Aw]"), Err(Error::InvalidList((0, 4)).into()));
        assert_eq!(parse_list("[Bw]"), Err(Error::InvalidList((0, 4)).into()));
        assert_eq!(parse_list("[*] "), Err(Error::InvalidList((0, 3)).into()));
    }
}
