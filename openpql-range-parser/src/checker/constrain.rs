use super::{
    Array, Card, Card64, ConstrainRank, ConstrainSuit, From, Idx, List,
    ListElem, RangeCard, Rank16, RankDiff, Span, SpanElem, Term,
};

#[derive(PartialEq, Eq, Debug, Default, Clone)]
pub(super) struct Constrain<const N: usize>
where
    [Idx; N]: Array<Item = Idx>,
{
    pub c64: Option<Card64>,
    pub rank: ConstrainRank<N>,
    pub suit: ConstrainSuit<N>,
}

impl<const N: usize> Constrain<N>
where
    [Idx; N]: Array<Item = Idx>,
{
    #[inline]
    pub fn reject(constrains: &[Self], cs: &[Card], perm: &[Idx]) -> bool {
        for (i, &p) in perm.iter().enumerate() {
            let constrain = &constrains[p as usize];

            if let Some(c64) = constrain.c64
                && !c64.contains_card(cs[i])
            {
                return true;
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

    #[allow(clippy::enum_glob_use)]
    pub fn from_card(t: &Term, card: RangeCard, i: Idx) -> Self {
        use RangeCard::*;

        match card {
            AA => Self::default(),
            CC(r, s) => (ConstrainRank::from(r), ConstrainSuit::from(s)).into(),
            CA(r) => ConstrainRank::from(r).into(),
            AC(s) => ConstrainSuit::from(s).into(),
            CV(r, sv) => {
                (ConstrainRank::from(r), ConstrainSuit::from((t, sv, i))).into()
            }

            VC(rv, s) => {
                (ConstrainRank::from((t, rv, i)), ConstrainSuit::from(s)).into()
            }

            VV(rv, sv) => (
                ConstrainRank::from((t, rv, i)),
                ConstrainSuit::from((t, sv, i)),
            )
                .into(),
            VA(rv) => ConstrainRank::from((t, rv, i)).into(),
            AV(sv) => ConstrainSuit::from((t, sv, i)).into(),
        }
    }

    pub fn from_list(cs: &List) -> Self {
        let mut c64: Card64 = Card64::default();

        for c in &cs.0 {
            match c {
                ListElem::CC(r, s) => {
                    c64 |= Card64::from(Card::new(*r, *s));
                }
                ListElem::CA(r) => {
                    c64 |= Card64::from_ranks(Rank16::from(*r));
                }
                ListElem::AC(s) => {
                    c64 |= Card64::from_suit(*s);
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
                        elems[i - 1].rank() as RankDiff
                            - elems[i].rank() as RankDiff,
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
fn head(span: &Span) -> SpanElem {
    match span {
        Span::Down(v) | Span::Up(v) | Span::To(v, _) => v[0],
    }
}

#[inline]
fn span_elems(span: &Span) -> &[SpanElem] {
    match span {
        Span::Down(v) | Span::Up(v) | Span::To(v, _) => v,
    }
}

#[inline]
fn span_depth(s: &Span) -> RankDiff {
    const RANK_I8_A: RankDiff = 12;

    match s {
        Span::Down(t) => -t.iter().map(|e| e.rank() as RankDiff).min().unwrap(),
        Span::Up(b) => {
            RANK_I8_A - b.iter().map(|e| e.rank() as RankDiff).max().unwrap()
        }
        Span::To(t, b) => b[0].rank() as RankDiff - t[0].rank() as RankDiff,
    }
}

#[inline]
fn r16_from_depth(rank_u8: u8, d: RankDiff) -> Rank16 {
    let ones = 2u16.pow(u32::from(d.unsigned_abs() + 1)) - 1;

    if d > 0 {
        Rank16::from(ones << rank_u8)
    } else {
        Rank16::from(ones << rank_u8.saturating_add_signed(d))
    }
}

impl<const N: usize> From<ConstrainRank<N>> for Constrain<N>
where
    [Idx; N]: Array<Item = Idx>,
{
    fn from(r: ConstrainRank<N>) -> Self {
        Self::from((r, ConstrainSuit::default()))
    }
}

impl<const N: usize> From<ConstrainSuit<N>> for Constrain<N>
where
    [Idx; N]: Array<Item = Idx>,
{
    fn from(s: ConstrainSuit<N>) -> Self {
        Self::from((ConstrainRank::default(), s))
    }
}

impl<const N: usize> From<(ConstrainRank<N>, ConstrainSuit<N>)> for Constrain<N>
where
    [Idx; N]: Array<Item = Idx>,
{
    fn from((rank, suit): (ConstrainRank<N>, ConstrainSuit<N>)) -> Self {
        Self {
            c64: None,
            rank,
            suit,
        }
    }
}

impl<const N: usize> From<Card64> for Constrain<N>
where
    [Idx; N]: Array<Item = Idx>,
{
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
    use super::*;
    use crate::*;

    fn assert_span_depth(s: &str, expected: RankDiff) {
        assert_eq!(span_depth(&parse_span(s).unwrap()), expected);
    }

    #[test]
    fn test_span_depth() {
        assert_span_depth("2-", 0);
        assert_span_depth("A+", 0);

        assert_span_depth("TQ8s-", -6);
        assert_span_depth("JTs-T9s", -1);
        assert_span_depth("K2s+", 1);
    }

    #[test]
    fn test_rank16_from_depth() {
        assert_eq!(r16_from_depth(0, 0), r16!("2"));
        assert_eq!(r16_from_depth(12, 0), r16!("A"));
        assert_eq!(r16_from_depth(4, 2), r16!("678"));
        assert_eq!(r16_from_depth(12, -5), r16!("9TJQKA"));
    }
}
