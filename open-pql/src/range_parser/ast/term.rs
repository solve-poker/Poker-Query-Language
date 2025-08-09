use super::{From, RangeCard, Span, list::List};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Term {
    pub inner: Vec<Elem>,
}

impl From<Vec<Elem>> for Term {
    fn from(inner: Vec<Elem>) -> Self {
        Self { inner }
    }
}

impl From<Span> for Term {
    fn from(s: Span) -> Self {
        Self {
            inner: vec![Elem::Span(s)],
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Elem {
    Card(RangeCard),
    List(List),
    Span(Span),
}

#[cfg(test)]
mod tests {
    use super::super::{
        super::tests::{parse_card as c, parse_term as p},
        ListElem as E, RankConst as R, Span as S, SpanElem as SE,
        TermElem as TE,
    };

    #[test]
    fn testcard() {
        assert_eq!(
            p("AsA").unwrap().inner,
            vec![TE::Card(c("As").unwrap()), TE::Card(c("A").unwrap())]
        );
    }

    #[test]
    fn test_list() {
        assert_eq!(
            p("[A,K]").unwrap().inner,
            vec![TE::List(vec![E::CA(R::RA), E::CA(R::RK)].into())]
        );

        assert_eq!(
            p("[A]").unwrap().inner,
            vec![TE::List(vec![E::CA(R::RA)].into())]
        );

        assert_eq!(
            p("[A,]").unwrap().inner,
            vec![TE::List(vec![E::CA(R::RA)].into())]
        );
    }

    #[test]
    fn test_span() {
        let span = S::Down(vec![SE::CA(R::RA)]);

        assert_eq!(p("[A-]").unwrap().inner, vec![TE::Span(span.clone())]);

        assert_eq!(p("A-").unwrap().inner, vec![TE::Span(span)]);
    }

    #[test]
    fn test_term() {
        assert_eq!(
            p("R[A,K]").unwrap().inner,
            vec![
                TE::Card(c("R").unwrap()),
                TE::List(vec![E::CA(R::RA), E::CA(R::RK)].into()),
            ]
        );
    }
}
