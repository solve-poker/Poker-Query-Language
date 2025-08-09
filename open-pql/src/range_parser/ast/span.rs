use super::{
    Error, LalrError, Loc, RangeCard, RankConst as RankC, ResultE,
    SuitConst as SuitC, TermElem,
};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Elem {
    CC(RankC, SuitC),
    CA(RankC),
}

impl Elem {
    pub const fn rank(self) -> RankC {
        match self {
            Self::CC(r, _) | Self::CA(r) => r,
        }
    }

    pub const fn suit(self) -> Option<SuitC> {
        match self {
            Self::CC(_, s) => Some(s),
            Self::CA(_) => None,
        }
    }
}

impl TryFrom<(Loc, Loc, RangeCard)> for Elem {
    type Error = LalrError<'static>;

    fn try_from((l, r, c): (Loc, Loc, RangeCard)) -> Result<Self, Self::Error> {
        match c {
            RangeCard::CC(r, s) => Ok(Self::CC(r, s)),
            RangeCard::CA(r) => Ok(Self::CA(r)),
            _ => Err(Error::InvalidSpan((l, r)).into()),
        }
    }
}

impl TryFrom<(Loc, Loc, TermElem)> for Elem {
    type Error = LalrError<'static>;

    fn try_from((l, r, e): (Loc, Loc, TermElem)) -> Result<Self, Self::Error> {
        match e {
            TermElem::Card(c) => Self::try_from((l, r, c)),
            _ => Err(Error::InvalidSpan((l, r)).into()),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Span {
    Down(Vec<Elem>),
    Up(Vec<Elem>),
    To(Vec<Elem>, Vec<Elem>),
}

#[inline]
fn to_span_elems<'i, T>(l: Loc, r: Loc, cs: Vec<T>) -> ResultE<'i, Vec<Elem>>
where
    Elem: TryFrom<(Loc, Loc, T), Error = LalrError<'i>>,
{
    cs.into_iter().map(|c| Elem::try_from((l, r, c))).collect()
}

#[inline]
fn ensure_same_format<'i>(
    l: Loc,
    r: Loc,
    v1: &[Elem],
    v2: &[Elem],
) -> ResultE<'i, ()> {
    #[inline]
    const fn is_same_distance(
        v1: &[Elem],
        v2: &[Elem],
        i: usize,
        j: usize,
    ) -> bool {
        v1[j].rank() as i8 - v1[i].rank() as i8
            == v2[j].rank() as i8 - v2[i].rank() as i8
    }

    let len = v1.len();

    if v2.len() != len {
        return Err(Error::NumberOfRanksMismatchInSpan((l, r)).into());
    }

    for i in 0..len {
        if i < len - 1 && !is_same_distance(v1, v2, i, i + 1) {
            return Err(Error::RankDistanceMismatchInSpan((l, r)).into());
        }
        if v1[i].suit() != v2[i].suit() {
            return Err(Error::SuitMismatchInSpan((l, r)).into());
        }
    }

    Ok(())
}

impl<'i> Span {
    #[allow(clippy::needless_pass_by_value)]
    pub(crate) fn spandown<T>(l: Loc, cs: Vec<T>, r: Loc) -> ResultE<'i, Self>
    where
        Elem: TryFrom<(Loc, Loc, T), Error = LalrError<'i>>,
    {
        Ok(Self::Down(to_span_elems(l, r, cs)?))
    }

    #[allow(clippy::needless_pass_by_value)]
    pub(crate) fn spanup<T>(l: Loc, cs: Vec<T>, r: Loc) -> ResultE<'i, Self>
    where
        Elem: TryFrom<(Loc, Loc, T), Error = LalrError<'i>>,
    {
        Ok(Self::Up(to_span_elems(l, r, cs)?))
    }

    #[allow(clippy::needless_pass_by_value)]
    pub(crate) fn spanto<T>(
        l: Loc,
        top: Vec<T>,
        btm: Vec<T>,
        r: Loc,
    ) -> ResultE<'i, Self>
    where
        Elem: TryFrom<(Loc, Loc, T), Error = LalrError<'i>>,
    {
        let t = to_span_elems(l, r, top)?;
        let b = to_span_elems(l, r, btm)?;

        ensure_same_format(l, r, &t, &b).map(|()| {
            if t[0].rank() > b[0].rank() {
                Self::To(t, b)
            } else {
                Self::To(b, t)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{
            super::tests::{parse_span as p, parse_term as t},
            RangeCard::{self, *},
            RankConst as R, Span, SpanElem as E, SuitConst as S,
        },
        Error,
    };
    use crate::*;

    const fn valid_card(c: RangeCard) -> bool {
        matches!(c, CC(_, _) | CA(_))
    }

    fn els(cs: &[RangeCard]) -> Vec<E> {
        cs.iter()
            .map(|c| match c {
                CC(r, s) => E::CC(*r, *s),
                CA(r) => E::CA(*r),
                _ => panic!("invalid span element {c:?}"),
            })
            .collect()
    }

    #[quickcheck]
    fn test_spandown(c1: RangeCard, c2: RangeCard) -> TestResult {
        if !valid_card(c1) || !valid_card(c2) {
            return TestResult::discard();
        }

        let src = format!("{}{}-", c1.to_src(), c2.to_src());

        assert_eq!(p(&src), Ok(Span::Down(els(&[c1, c2]))));

        TestResult::passed()
    }

    #[quickcheck]
    fn test_spanup(c1: RangeCard, c2: RangeCard) -> TestResult {
        if !valid_card(c1) || !valid_card(c2) {
            return TestResult::discard();
        }

        let src = format!("{}{}+", c1.to_src(), c2.to_src());

        assert_eq!(p(&src), Ok(Span::Up(els(&[c1, c2]))));

        TestResult::passed()
    }

    #[quickcheck]
    fn test_spanto(a1: R, a2: R, b1: R, b2: R, s: S) -> TestResult {
        if (a1 as i8 - a2 as i8) != (b1 as i8 - b2 as i8) {
            return TestResult::discard();
        }

        let src = format!(
            "{}{}{}-{}{}{}",
            a1.to_src(),
            b1.to_src(),
            s.to_src(),
            a2.to_src(),
            b2.to_src(),
            s.to_src(),
        );

        let v1 = els(&[RangeCard::CA(a1), RangeCard::CC(b1, s)]);
        let v2 = els(&[RangeCard::CA(a2), RangeCard::CC(b2, s)]);

        let (top, btm) = if a1 > a2 { (v1, v2) } else { (v2, v1) };

        assert_eq!(p(&src), Ok(Span::To(top, btm)));

        TestResult::passed()
    }

    #[test]
    fn test_spanto_fixed() {
        const C_AS: E = E::CC(R::RA, S::S);
        const C_K: E = E::CA(R::RK);
        const C_JS: E = E::CC(R::RJ, S::S);
        const C_T: E = E::CA(R::RT);

        let span = p("AsK-JsT").unwrap();

        assert_eq!(span, Span::To(vec![C_AS, C_K], vec![C_JS, C_T]));
        assert_eq!(p("T-A"), p("A-T"));
        assert_eq!(p("TT-AA"), p("AA-TT"));
    }

    #[test]
    fn test_span_error() {
        assert_eq!(
            p("A-AK"),
            Err(Error::NumberOfRanksMismatchInSpan((0, 4)).into())
        );

        assert_eq!(
            p("AA-QT"),
            Err(Error::RankDistanceMismatchInSpan((0, 5)).into())
        );

        assert_eq!(p("As-K"), Err(Error::SuitMismatchInSpan((0, 4)).into()));
    }

    #[test]
    fn test_span_error_invalid() {
        assert_eq!(p("B-    "), Err(Error::InvalidSpan((0, 2)).into()));
        assert_eq!(p("Bs-   "), Err(Error::InvalidSpan((0, 3)).into()));
        assert_eq!(p("*w-   "), Err(Error::InvalidSpan((0, 3)).into()));
        assert_eq!(p("Aw-   "), Err(Error::InvalidSpan((0, 3)).into()));
        assert_eq!(p("Bw-   "), Err(Error::InvalidSpan((0, 3)).into()));
        assert_eq!(p("*-    "), Err(Error::InvalidSpan((0, 2)).into()));
        assert_eq!(t("A[A]+ "), Err(Error::InvalidSpan((0, 5)).into()));
        assert_eq!(t("A[A-]-"), Err(Error::InvalidSpan((0, 6)).into()));
        assert_eq!(t("[A]-A "), Err(Error::InvalidSpan((0, 5)).into()));
        assert_eq!(t("A-[A] "), Err(Error::InvalidSpan((0, 5)).into()));
    }
}
