// TODO: remove!; tmp implementation
#![cfg_attr(coverage_nightly, coverage(off))]

use super::*;

#[derive(Clone, Debug)]
pub struct RunnerOutput {
    aggregators: Vec<OutputAggregator>,
    pub n_fail: usize,
    pub n_succ: usize,
}

impl RunnerOutput {
    pub fn new(game: PQLGame, selectors: &[ast::Selector]) -> Self {
        Self {
            aggregators: selectors
                .iter()
                .map(|s| OutputAggregator::new(game, s.kind))
                .collect(),
            n_fail: 0,
            n_succ: 0,
        }
    }

    pub fn push_value(&mut self, idx: usize, val: VmStackValue) {
        self.aggregators[idx].push_value(val);
    }

    /// # Panics
    /// both outputs must come from the same statement
    pub fn merge(&mut self, other: Self) {
        assert_eq!(self.aggregators.len(), other.aggregators.len());

        for (agg, other_agg) in
            self.aggregators.iter_mut().zip(other.aggregators)
        {
            agg.merge(other_agg);
        }

        self.n_fail += other.n_fail;
        self.n_succ += other.n_succ;
    }

    pub fn report_to_stream<W: io::Write>(
        &self,
        stmt: &ast::Stmt,
        stream: &mut W,
    ) -> io::Result<()> {
        for (i, aggregator) in self.aggregators.iter().enumerate() {
            let sel = &stmt.selectors[i];
            if let Some(id) = &sel.alias {
                write!(stream, "{}", id.inner)?;
            } else {
                write!(stream, "{}", sel.kind)?;
            }
            writeln!(stream, " {i} = {aggregator}")?;
        }

        Ok(())
    }
}
