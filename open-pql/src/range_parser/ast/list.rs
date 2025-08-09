use super::{
    Error, LalrError, Loc, RangeCard, RankConst as RankC, SuitConst as SuitC,
};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Elem {
    CC(RankC, SuitC),
    CA(RankC),
    AC(SuitC),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct List(pub Vec<Elem>);

impl TryFrom<(Loc, Vec<RangeCard>, Loc)> for List {
    type Error = LalrError<'static>;

    fn try_from(
        (l, v, r): (Loc, Vec<RangeCard>, Loc),
    ) -> Result<Self, Self::Error> {
        let mut inner = vec![];

        for c in v {
            match c {
                RangeCard::CC(r, s) => {
                    inner.push(Elem::CC(r, s));
                }
                RangeCard::CA(r) => {
                    inner.push(Elem::CA(r));
                }
                RangeCard::AC(s) => {
                    inner.push(Elem::AC(s));
                }
                _ => {
                    return Err(Error::InvalidList((l, r)).into());
                }
            }
        }

        Ok(Self(inner))
    }
}

impl From<Vec<Elem>> for List {
    fn from(inner: Vec<Elem>) -> Self {
        Self(inner)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{
        super::tests::parse_list as p, ListElem as E, RangeCard,
        RankConst as R, SuitConst as S, *,
    };

    #[quickcheck]
    fn test_list(c: RangeCard) {
        let src = format!("[{}]", c.to_src());
        let res = p(&src);

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
        assert_eq!(p("[A,s]"), Ok(vec![E::CA(R::RA), E::AC(S::S)].into()));
    }

    #[test]
    fn test_list_error() {
        assert_eq!(p("[B] "), Err(Error::InvalidList((0, 3)).into()));
        assert_eq!(p("[Bs]"), Err(Error::InvalidList((0, 4)).into()));
        assert_eq!(p("[*w]"), Err(Error::InvalidList((0, 4)).into()));
        assert_eq!(p("[Aw]"), Err(Error::InvalidList((0, 4)).into()));
        assert_eq!(p("[Bw]"), Err(Error::InvalidList((0, 4)).into()));
        assert_eq!(p("[*] "), Err(Error::InvalidList((0, 3)).into()));
    }
}
