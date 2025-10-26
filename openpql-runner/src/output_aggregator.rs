use super::*;

type Count = usize;

#[derive(Clone, Debug, derive_more::From, Display)]
#[display("{_0}")]
pub enum OutputAggregator {
    Avg(OutputAggregatorAvg),
    Count(OutputAggregatorCount),
    Max(OutputAggregatorCmp<false, true>),
    Min(OutputAggregatorCmp<false, false>),
    MaxSd(OutputAggregatorCmp<true, true>),
    MinSd(OutputAggregatorCmp<true, false>),
}

impl OutputAggregator {
    pub fn new(game: PQLGame, kind: ast::SelectorKind) -> Self {
        match (kind, game) {
            (ast::SelectorKind::Avg, _) => {
                OutputAggregatorAvg::default().into()
            }
            (ast::SelectorKind::Count, _) => {
                OutputAggregatorCount::default().into()
            }
            (ast::SelectorKind::Max, PQLGame::ShortDeck) => {
                OutputAggregatorCmp::<true, true>::default().into()
            }
            (ast::SelectorKind::Min, PQLGame::ShortDeck) => {
                OutputAggregatorCmp::<true, false>::default().into()
            }
            (ast::SelectorKind::Max, _) => {
                OutputAggregatorCmp::<false, true>::default().into()
            }
            (ast::SelectorKind::Min, _) => {
                OutputAggregatorCmp::<false, false>::default().into()
            }
        }
    }

    pub fn push_value(&mut self, val: VmStackValue) {
        match self {
            Self::Avg(inner) => inner.push_val(val),
            Self::Count(inner) => inner.push_val(val),
            Self::Max(inner) => inner.push_val(val),
            Self::Min(inner) => inner.push_val(val),
            Self::MaxSd(inner) => inner.push_val(val),
            Self::MinSd(inner) => inner.push_val(val),
        }
    }
}

#[derive(Clone, Debug, Default, Display)]
#[display("{}", self.to_f())]
pub struct OutputAggregatorAvg {
    num: PQLDouble,
    den: Count,
}

impl OutputAggregatorAvg {
    pub fn push_val(&mut self, val: VmStackValue) {
        self.num += PQLNumeric::try_from(val).unwrap().to_dbl();
        self.den += 1;
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn to_f(&self) -> PQLDouble {
        self.num / self.den as PQLDouble
    }
}

#[derive(Clone, Debug, Default, Display)]
#[display("{_0}")]
pub struct OutputAggregatorCount(Count);

impl OutputAggregatorCount {
    pub fn push_val(&mut self, val: VmStackValue) {
        match val {
            VmStackValue::Bool(true) => {
                self.0 += 1;
            }
            VmStackValue::Bool(false) => {}
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct OutputAggregatorCmp<const SD: bool, const MAX: bool>(
    Option<VmStackValue>,
);

impl<const SD: bool, const MAX: bool> fmt::Display
    for OutputAggregatorCmp<SD, MAX>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(v) => write!(f, "{v}"),
            None => write!(f, "None"),
        }
    }
}

pub fn compare<const SD: bool>(
    lhs: VmStackValue,
    rhs: VmStackValue,
) -> Option<cmp::Ordering> {
    let game = const {
        if SD {
            PQLGame::ShortDeck
        } else {
            PQLGame::Holdem // partial_compare of Omaha is same
        }
    };

    VmBinOpCmp::compare(game, lhs, rhs).unwrap()
}

impl<const SD: bool, const MAX: bool> OutputAggregatorCmp<SD, MAX> {
    pub fn push_val(&mut self, rhs: VmStackValue) {
        let order = const {
            if MAX {
                cmp::Ordering::Less
            } else {
                cmp::Ordering::Greater
            }
        };

        match self.0 {
            Some(lhs) => {
                if compare::<SD>(lhs, rhs) == Some(order) {
                    self.0 = Some(rhs);
                }
            }
            None => self.0 = Some(rhs),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aggregator_avg() {
        let mut agg =
            OutputAggregator::new(PQLGame::default(), ast::SelectorKind::Avg);
        agg.push_value(10.0.into());
        agg.push_value(20.0.into());
        agg.push_value(30.0.into());

        assert_eq!(agg.to_string(), "20");
    }

    #[quickcheck]
    fn test_aggregator_count(vals: Vec<bool>) {
        let mut agg =
            OutputAggregator::new(PQLGame::default(), ast::SelectorKind::Count);
        for val in &vals {
            agg.push_value((*val).into());
        }

        assert_eq!(
            agg.to_string(),
            vals.into_iter().filter(|b| *b).count().to_string()
        );
    }

    fn assert_cmp<T>(
        game: PQLGame,
        sel: ast::SelectorKind,
        values: &[T],
        expected: &str,
    ) where
        T: Copy,
        VmStackValue: From<T>,
    {
        let mut agg = OutputAggregator::new(game, sel);
        for &v in values {
            agg.push_value(v.into());
        }

        assert_eq!(agg.to_string(), expected);
    }

    #[test]
    fn test_aggregator_max() {
        assert_cmp(
            PQLGame::default(),
            ast::SelectorKind::Max,
            &[
                PQLHandType::Flush,
                PQLHandType::FullHouse,
                PQLHandType::Pair,
            ],
            "FULL_HOUSE",
        );
    }

    #[test]
    fn test_aggregator_max_sd() {
        assert_cmp(
            PQLGame::ShortDeck,
            ast::SelectorKind::Max,
            &[
                PQLHandType::Flush,
                PQLHandType::FullHouse,
                PQLHandType::Pair,
            ],
            "FLUSH",
        );
    }

    #[test]
    fn test_aggregator_min() {
        assert_cmp(
            PQLGame::default(),
            ast::SelectorKind::Min,
            &[
                PQLHandType::Flush,
                PQLHandType::FullHouse,
                PQLHandType::Pair,
            ],
            "PAIR",
        );
    }

    #[test]
    fn test_aggregator_min_sd() {
        assert_cmp(
            PQLGame::ShortDeck,
            ast::SelectorKind::Min,
            &[
                PQLHandType::Flush,
                PQLHandType::FullHouse,
                PQLHandType::Pair,
            ],
            "PAIR",
        );
    }
}
