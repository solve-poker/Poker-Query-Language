// TODO: remove!;
#![cfg_attr(coverage_nightly, coverage(off))]

use super::*;

/// Simple Virtual Machine
/// intended to be created and destroyed for each PQL statements
/// and runs in a single thread;
/// clones share `cache`, so trials can run on multiple threads
/// with one clone each
#[derive(Clone, Debug, Default)]
pub struct Vm {
    pub static_data: VmStaticData,
    pub stack: VmStack,
    pub heap: VmHeap,
    pub sampled_data: VmSampledData,
    pub cache: VmCache,
}

impl Vm {
    pub(crate) fn from_stmt(stmt: &ast::Stmt<'_>) -> PQLResult<Self> {
        let static_data = VmStaticData::try_from(&stmt.from)?;

        let sampled_data = VmSampledData::new(
            static_data.game,
            static_data.n_players,
            static_data.dead_card,
        );

        Ok(Self {
            static_data,
            sampled_data,
            ..Default::default()
        })
    }

    pub(crate) fn as_context(&mut self) -> VmExecContext<'_> {
        VmExecContext {
            stack: &mut self.stack,
            heap: &mut self.heap,
            fn_ctx: PQLFnContext {
                game: self.static_data.game,
                sampled_cards: &self.sampled_data.cards,
                n_players: self.static_data.n_players,
                cache: &self.cache,
            },
        }
    }

    pub(crate) fn sample(&mut self, rng: &mut impl rand::Rng) -> Option<()> {
        self.sampled_data.sample(
            rng,
            &self.static_data.player_ranges,
            &self.static_data.board_range,
        )
    }
}
