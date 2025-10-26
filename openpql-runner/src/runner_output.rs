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
