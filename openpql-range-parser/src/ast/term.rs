use super::{Display, From, List, RangeCard, Span, ToString};

#[derive(Clone, PartialEq, Eq, Debug, derive_more::From, Display)]
#[display("{}", to_str(_0))]
pub struct Term(pub Vec<TermElem>);

impl From<Span> for Term {
    fn from(s: Span) -> Self {
        Self(vec![TermElem::Span(s)])
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Display)]
pub enum TermElem {
    #[display("{_0}")]
    Card(RangeCard),
    #[display("{_0}")]
    List(List),
    #[display("{_0}")]
    Span(Span),
}

fn to_str(elems: &[TermElem]) -> String {
    elems.iter().map(ToString::to_string).collect::<String>()
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use crate::*;

    fn assert_term(src: &str, expected: &str) {
        let list = parse_term(src).unwrap();

        assert_eq!(list.to_string(), expected, "{src} != {expected}");
    }

    #[test]
    fn test_card() {
        assert_term("AsA", "AsA");
    }

    #[test]
    fn test_list() {
        assert_term("[A,K]", "[A,K]");
        assert_term("[A]", "[A]");
        assert_term("[A,]", "[A]");
    }

    #[test]
    fn test_span() {
        assert_term("[A-]", "A-");
        assert_term("A-", "A-");
    }

    #[test]
    fn test_term() {
        assert_term("R[A,K]", "R[A,K]");
    }

    #[test]
    fn test_span_error_invalid() {
        assert_err(parse_term("A[A]+ "), Error::InvalidSpan((0, 5)));
        assert_err(parse_term("A[A-]-"), Error::InvalidSpan((0, 6)));
        assert_err(parse_term("[A]-A "), Error::InvalidSpan((0, 5)));
        assert_err(parse_term("A-[A] "), Error::InvalidSpan((0, 5)));
    }
}
