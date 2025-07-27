use super::{
    ast::{
        self, List, ListElem,
        RangeCard::{AA, AC, AV, CA, CC, CV, VA, VC, VV},
        RankVar as RankV, Span, SuitVar as SuitV, Term, TermElem as Elem,
    },
    Card, Card64, ConstrainRank, ConstrainSuit, Idx, Rank16, MASK64_2,
    MASK64_S, OFFSET_SUIT,
};

#[derive(PartialEq, Eq, Debug, Default, Clone)]
pub(super) struct Constrain {
    pub c64: Option<Card64>,
    pub rank: ConstrainRank,
    pub suit: ConstrainSuit,
}

impl Constrain {
    #[inline]
    pub fn reject(constrains: &[Self], cs: &[Card], perm: &[Idx]) -> bool {
        for (i, &p) in perm.iter().enumerate() {
            let constrain = &constrains[p as usize];

            if let Some(c64) = constrain.c64 {
                if !c64.contains_card(cs[i]) {
                    return true;
                }
            }

            if constrain.rank.reject(cs, perm, i) {
                return true;
            }

            if constrain.suit.reject(cs, perm, i) {
                return true;
            }
        }

        false
    }

    pub fn from_card(t: &Term, card: ast::RangeCard, i: Idx) -> Self {
        match card {
            AA => Self::default(),
            CC(r, s) => (ConstrainRank::from(r), ConstrainSuit::from(s)).into(),
            CA(r) => ConstrainRank::from(r).into(),
            AC(s) => ConstrainSuit::from(s).into(),
            CV(r, sv) => (
                ConstrainRank::from(r),
                ConstrainSuit::from(var_info_suit(t, sv, i)),
            )
                .into(),

            VC(rv, s) => (
                ConstrainRank::from(var_info_rank(t, rv, i)),
                ConstrainSuit::from(s),
            )
                .into(),

            VV(rv, sv) => (
                ConstrainRank::from(var_info_rank(t, rv, i)),
                ConstrainSuit::from(var_info_suit(t, sv, i)),
            )
                .into(),
            VA(rv) => ConstrainRank::from(var_info_rank(t, rv, i)).into(),
            AV(sv) => ConstrainSuit::from(var_info_suit(t, sv, i)).into(),
        }
    }

    pub fn from_list(cs: &List) -> Self {
        let mut c64: Card64 = Card64::empty();

        for c in &cs.0 {
            match c {
                ListElem::CC(r, s) => {
                    c64 |= c64_from_ranksuit_i8(*r as i8, *s as i8);
                }
                ListElem::CA(r) => {
                    c64 |= c64_from_rank_i8(*r as i8);
                }
                ListElem::AC(s) => {
                    c64 |= c64_from_suit_i8(*s as i8);
                }
            }
        }

        c64.into()
    }

    fn from_span_head(span: &Span) -> Self {
        let head = head(span);

        Self::from((
            ConstrainRank::from(r16_from_depth(
                head.rank() as u8,
                span_depth(span),
            )),
            ConstrainSuit::from(head.suit()),
        ))
    }

    pub fn from_span(span: &Span, start_idx: Idx) -> Vec<Self> {
        let elems = span_elems(span);
        let len = elems.len();
        let mut res = Vec::with_capacity(len);

        res.push(Self::from_span_head(span));

        let mut prev_idx = start_idx;

        for i in 1..len {
            res.push(
                (
                    ConstrainRank::Diff(
                        prev_idx,
                        elems[i - 1].rank() as i8 - elems[i].rank() as i8,
                    ),
                    ConstrainSuit::from(elems[i].suit()),
                )
                    .into(),
            );

            prev_idx += 1;
        }

        res
    }
}

#[inline]
fn head(span: &ast::Span) -> ast::SpanElem {
    match span {
        ast::Span::Down(v) | ast::Span::Up(v) | ast::Span::To(v, _) => v[0],
    }
}

#[inline]
fn span_elems(span: &ast::Span) -> &[ast::SpanElem] {
    match span {
        ast::Span::Down(v) | ast::Span::Up(v) | ast::Span::To(v, _) => v,
    }
}

#[inline]
fn var_info_rank(
    term: &ast::Term,
    r: RankV,
    self_i: u8,
) -> (Vec<u8>, Vec<u8>, u16) {
    #[inline]
    const fn u16_from_rank_i8(r: i8) -> u16 {
        1 << r
    }

    let mut eq = vec![];
    let mut neq = vec![];
    let mut banned = 0;

    for (i, e) in term.inner.iter().enumerate() {
        if i == self_i as usize {
            continue;
        }

        match e {
            Elem::Card(CC(r, _) | CV(r, _) | CA(r)) => {
                banned |= u16_from_rank_i8(*r as i8);
            }
            Elem::Card(VC(other, _) | VV(other, _) | VA(other)) => {
                if *other == r {
                    eq.push(i.to_le_bytes()[0]);
                } else {
                    neq.push(i.to_le_bytes()[0]);
                }
            }
            _ => (),
        }
    }

    (eq, neq, banned)
}

#[inline]
fn var_info_suit(
    term: &ast::Term,
    s: SuitV,
    self_i: u8,
) -> (Vec<u8>, Vec<u8>, u8) {
    #[inline]
    const fn u8_from_suit_i8(s: i8) -> u8 {
        1 << s
    }

    let mut eq = vec![];
    let mut neq = vec![];
    let mut banned = 0;

    for (i, e) in term.inner.iter().enumerate() {
        if i == self_i as usize {
            continue;
        }

        match e {
            Elem::Card(CC(_, s) | VC(_, s) | AC(s)) => {
                banned |= u8_from_suit_i8(*s as i8);
            }
            Elem::Card(CV(_, other) | VV(_, other) | AV(other)) => {
                if *other == s {
                    eq.push(i.to_le_bytes()[0]);
                } else {
                    neq.push(i.to_le_bytes()[0]);
                }
            }
            _ => (),
        }
    }

    (eq, neq, banned)
}

#[inline]
const fn c64_from_ranksuit_i8(r: i8, s: i8) -> Card64 {
    Card64::from_u64(Card64::u64_from_ranksuit_i8(r, s))
}

#[inline]
const fn c64_from_rank_i8(r: i8) -> Card64 {
    Card64::from_u64(MASK64_2 << r)
}

#[inline]
const fn c64_from_suit_i8(s: i8) -> Card64 {
    Card64::from_u64(MASK64_S << (s * OFFSET_SUIT))
}

#[inline]
fn span_depth(s: &Span) -> i8 {
    const RANK_I8_A: i8 = 12;

    match s {
        ast::Span::Down(t) => -t.iter().map(|e| e.rank() as i8).min().unwrap(),
        ast::Span::Up(b) => {
            RANK_I8_A - b.iter().map(|e| e.rank() as i8).max().unwrap()
        }
        Span::To(t, b) => b[0].rank() as i8 - t[0].rank() as i8,
    }
}

#[inline]
fn r16_from_depth(rank_u8: u8, d: i8) -> Rank16 {
    let ones = 2u16.pow(u32::from(d.unsigned_abs() + 1)) - 1;

    if d > 0 {
        Rank16::from_u16(ones << rank_u8)
    } else {
        Rank16::from_u16(ones << rank_u8.saturating_add_signed(d))
    }
}

impl From<ConstrainRank> for Constrain {
    fn from(r: ConstrainRank) -> Self {
        Self::from((r, ConstrainSuit::default()))
    }
}

impl From<ConstrainSuit> for Constrain {
    fn from(s: ConstrainSuit) -> Self {
        Self::from((ConstrainRank::default(), s))
    }
}

impl From<(ConstrainRank, ConstrainSuit)> for Constrain {
    fn from((rank, suit): (ConstrainRank, ConstrainSuit)) -> Self {
        Self {
            c64: None,
            rank,
            suit,
        }
    }
}

impl From<Card64> for Constrain {
    fn from(c: Card64) -> Self {
        Self {
            c64: Some(c),
            rank: ConstrainRank::Nil,
            suit: ConstrainSuit::Nil,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ast::{RankVar::*, SuitVar::*},
        *,
    };
    use crate::{
        constants::MASK16_RANKS,
        range_parser::tests::{parse_span, parse_term},
        *,
    };

    #[test]
    fn test_card64_from_ranksuit_vals() {
        assert_eq!(
            c64_from_ranksuit_i8(Rank::RT as i8, Suit::H as i8),
            c64!("Th")
        );
    }

    #[test]
    fn test_card64_from_rankval() {
        assert_eq!(c64_from_rank_i8(Rank::RA as i8), c64!("As Ah Ad Ac"));
    }

    #[test]
    fn test_card64_from_suitval() {
        assert_eq!(
            c64_from_suit_i8(Suit::C as i8),
            Card64::from_u64(u64::from(MASK16_RANKS) << (3 * 16))
        );
    }

    fn s(s: &str) -> ast::Span {
        parse_span(s).unwrap()
    }

    #[test]
    fn test_span_depth() {
        assert_eq!(span_depth(&s("2-")), 0);
        assert_eq!(span_depth(&s("A+")), 0);

        assert_eq!(span_depth(&s("TQ8s-")), -6);
        assert_eq!(span_depth(&s("JTs-T9s")), -1);
        assert_eq!(span_depth(&s("K2s+")), 1);
    }

    #[test]
    fn test_rank16_from_depth() {
        assert_eq!(r16_from_depth(0, 0), r16!("2"));
        assert_eq!(r16_from_depth(12, 0), r16!("A"));
        assert_eq!(r16_from_depth(4, 2), r16!("678"));
        assert_eq!(r16_from_depth(12, -5), r16!("9TJQKA"));
    }

    fn p(s: &str) -> ast::Term {
        parse_term(s).unwrap()
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    #[test]
    fn test_var_info_rank() {
        let t = p("R[A,K][AK-]BAR");
        assert_eq!(
            var_info_rank(&t, RB, 3),
            (vec![], vec![0, 5], r16!("A").to_u16()),
        );
        assert_eq!(
            var_info_rank(&t, RR, 0),
            (vec![5], vec![3], r16!("A").to_u16()),
        );

        assert_eq!(
            var_info_rank(&p("RAs"), RR, 0),
            (vec![], vec![], r16!("A").to_u16()),
            "should be different than A"
        );
        assert_eq!(
            var_info_rank(&p("RAw"), RR, 0),
            (vec![], vec![], r16!("A").to_u16()),
            "should be different than A"
        );
        assert_eq!(
            var_info_rank(&p("RA"), RR, 0),
            (vec![], vec![], r16!("A").to_u16()),
            "should be different than A"
        );
        assert_eq!(
            var_info_rank(&p("ROs"), RR, 0),
            (vec![], vec![1], 0),
            "should be different than the rank at 1"
        );
        assert_eq!(
            var_info_rank(&p("ROw"), RR, 0),
            (vec![], vec![1], 0),
            "should be different than the rank at 1"
        );
        assert_eq!(
            var_info_rank(&p("RO"), RR, 0),
            (vec![], vec![1], 0),
            "should be different than the rank at 1"
        );
        assert_eq!(
            var_info_rank(&p("RRs"), RR, 0),
            (vec![1], vec![], 0),
            "should be same as the rank at 1"
        );
        assert_eq!(
            var_info_rank(&p("RRx"), RR, 0),
            (vec![1], vec![], 0),
            "should be same as the rank at 1"
        );
        assert_eq!(
            var_info_rank(&p("RR"), RR, 0),
            (vec![1], vec![], 0),
            "should be same as the rank at 1"
        );
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    #[test]
    fn test_var_info_suit() {
        let t = p("x[c][AdKh-]ysx");
        assert_eq!(
            var_info_suit(&t, Y, 3),
            (vec![], vec![0, 5], s4!("s").to_u8()),
        );
        assert_eq!(
            var_info_suit(&t, X, 0),
            (vec![5], vec![3,], s4!("s").to_u8())
        );

        assert_eq!(
            var_info_suit(&p("xAs"), X, 0),
            (vec![], vec![], s4!("s").to_u8()),
            "should be different than the s"
        );
        assert_eq!(
            var_info_suit(&p("xOs"), X, 0),
            (vec![], vec![], s4!("s").to_u8()),
            "should be different than the s"
        );
        assert_eq!(
            var_info_suit(&p("xs"), X, 0),
            (vec![], vec![], s4!("s").to_u8()),
            "should be different than the s"
        );
        assert_eq!(
            var_info_suit(&p("xAy"), X, 0),
            (vec![], vec![1], 0),
            "should be different than the suit at 1"
        );
        assert_eq!(
            var_info_suit(&p("xRy"), X, 0),
            (vec![], vec![1], 0),
            "should be different than the suit at 1"
        );
        assert_eq!(
            var_info_suit(&p("xy"), X, 0),
            (vec![], vec![1], 0),
            "should be different than the suit at 1"
        );
        assert_eq!(
            var_info_suit(&p("xAx"), X, 0),
            (vec![1], vec![], 0),
            "should be same as the suit at 1"
        );
        assert_eq!(
            var_info_suit(&p("xRx"), X, 0),
            (vec![1], vec![], 0),
            "should be same as the suit at 1"
        );
        assert_eq!(
            var_info_suit(&p("xx"), X, 0),
            (vec![1], vec![], 0),
            "should be same as the suit at 1"
        );
    }
}
