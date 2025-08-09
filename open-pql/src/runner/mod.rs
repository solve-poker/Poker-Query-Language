use std::{io, slice::Iter, sync::LazyLock};

use derive_more::Debug;

mod init_board;
mod init_game;
mod init_players;

use concurrency::parallel_exec;
use init_board::init_board;
use init_game::init_game;
use init_players::init_players;
use pql_parser::{ast, parse};
use rustc_hash::*;
use vm::{Vm, VmStackValue, VmStackValueNum, VmStoreVarIdx, *};

use super::*;
use crate::{LocInfo, PQLLong, vm::VmInstruction};

#[derive(Debug)]
pub struct StatementsRunner<'src> {
    pub(crate) src: &'src str,
    pub n_trials: usize,
    pub n_threads: usize,
    #[debug(skip)]
    pub stream_out: Box<dyn io::Write>,
    #[debug(skip)]
    pub stream_err: Box<dyn io::Write>,
}

type RangeProc = Box<dyn FnOnce() -> Result<PQLRange, PQLError> + Send>;

/// # Panics
/// ranges at least consists of one player and the board
pub fn init_vm(stmt: &ast::Stmt, n_trials: usize) -> Result<Vm, PQLError> {
    let fc = &stmt.from;
    let sels = &stmt.selectors;

    let game = init_game(fc)?;
    let board_range_proc = init_board(fc);
    let (player_names, mut ranges_procs) = init_players(fc, game);

    ranges_procs.push(board_range_proc);
    let mut ranges = parallel_exec(ranges_procs)?
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;
    let board_range = ranges.pop().unwrap();

    let (instructions, store) = instruction::init(sels, game, &player_names)?;

    let player_ranges = ranges;

    Ok(Vm {
        board_range,
        player_ranges,
        instructions,
        store,
        stack: VmStack::default(),
        buffer: VmBuffer::new(player_names.len(), game),
        rng: Rng::default(),
        n_trials,
        n_failed: 0,
    })
}

impl<'src> StatementsRunner<'src> {
    pub const fn new(
        src: &'src str,
        n_trials: usize,
        n_threads: usize,
        stream_out: Box<dyn io::Write>,
        stream_err: Box<dyn io::Write>,
    ) -> Self {
        Self {
            src,
            n_trials,
            n_threads,
            stream_out,
            stream_err,
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn run(&mut self) {
        match parse(self.src) {
            Ok(stmts) => {
                for stmt in &stmts {
                    match self.run_stmt(stmt) {
                        Ok(()) => (),
                        Err(e) => self.report_error(&e),
                    }
                }

                self.stream_out
                    .flush()
                    .expect("Failed to write to output stream");
            }
            Err(e) => {
                self.report_error(&PQLError::from(e));
            }
        }
    }

    fn run_stmt(&mut self, stmt: &ast::Stmt) -> Result<(), PQLError> {
        let n = self.n_threads;
        let n_trials = self.n_trials / n;

        let vm = init_vm(stmt, n_trials)?;

        let mut vms: Vec<Vm> = Vec::with_capacity(n);
        vms.push(vm);

        while vms.len() != n {
            vms.push(vms[0].clone());
        }

        let procs = vms
            .into_iter()
            .map(|vm| Box::new(move || vm.try_run()) as _)
            .collect::<Vec<_>>();

        match (parallel_exec(procs)?)
            .into_iter()
            .collect::<Result<Vec<Vm>, PQLError>>()
        {
            Ok(vms) => {
                self.aggregate_outputs(&vms, &stmt.selectors, self.n_trials);
            }

            Err(e) => self.report_error(&e),
        }

        Ok(())
    }

    fn report_error(&mut self, e: &PQLError) {
        let loc: Option<LocInfo> = e.into();

        writeln!(self.stream_err, "Error:")
            .and_then(|()| writeln!(self.stream_err, "{e}"))
            .and_then(|()| {
                if let Some((a, b)) = loc {
                    writeln!(self.stream_err, "{}", &self.src[a..b])
                } else {
                    Ok(())
                }
            })
            .expect("Failed to write to error stream");
    }

    #[allow(clippy::cast_precision_loss)]
    fn aggregate_outputs(
        &mut self,
        vms: &[Vm],
        selectors: &[ast::Selector],
        n_trials: usize,
    ) {
        fn next_write(iter: &mut Iter<'_, VmInstruction>) -> VmStoreVarIdx {
            loop {
                match iter.next() {
                    Some(VmInstruction::Write(idx)) => return *idx,
                    Some(_) => (),
                    None => todo!(),
                }
            }
        }

        let vec_ins = vms[0].instructions.clone();
        let _game: PQLGame = (&vms[0].buffer).into();
        let mut iter = vec_ins.iter();

        for (n, selector) in selectors.iter().enumerate() {
            let idx = next_write(&mut iter);

            let name = selector.alias.as_ref().map_or_else(
                || format!("{:?}{}", selector.kind, n + 1),
                |id| id.inner.into(),
            );

            let res = match selector.kind {
                ast::SelectorKind::Avg => {
                    let v: VmStackValue = vms
                        .iter()
                        .map(|vm| {
                            *vm.store
                                .downcast_get::<&VmStackValue>(idx)
                                .unwrap()
                        })
                        .reduce(|m, e| m.try_add(e).unwrap())
                        .unwrap();

                    let c: PQLLong = vms
                        .iter()
                        .map(|vm| {
                            *vm.store.downcast_get::<&PQLLong>(idx + 1).unwrap()
                        })
                        .sum::<PQLLong>();

                    let n: VmStackValueNum = v.try_into().unwrap();
                    writeln!(
                        self.stream_out,
                        "{name} = {}",
                        n.cast_dbl() / c as f64
                    )
                }

                ast::SelectorKind::Count => {
                    let c: PQLLong = vms
                        .iter()
                        .map(|vm| {
                            *vm.store.downcast_get::<&PQLLong>(idx).unwrap()
                        })
                        .sum::<PQLLong>();

                    writeln!(
                        self.stream_out,
                        "{name} = {}%({})",
                        100.0 * c as f64 / n_trials as f64,
                        c
                    )
                }

                ast::SelectorKind::Max => {
                    let v: VmStackValue = vms
                        .iter()
                        .map(|vm| {
                            *vm.store
                                .downcast_get::<&VmStackValue>(idx)
                                .unwrap()
                        })
                        .reduce(|m, e| if m >= e { m } else { e })
                        .unwrap();

                    writeln!(self.stream_out, "{name} = {v}")
                }

                ast::SelectorKind::Min => {
                    let v: VmStackValue = vms
                        .iter()
                        .map(|vm| {
                            *vm.store
                                .downcast_get::<&VmStackValue>(idx)
                                .unwrap()
                        })
                        .reduce(|m, e| if m <= e { m } else { e })
                        .unwrap();

                    writeln!(self.stream_out, "{name} = {v}")
                }
            };

            res.expect("Failed to write to output stream");
        }
    }
}
