#![allow(clippy::wildcard_imports)]

use std::{cmp::Ordering, collections::VecDeque, ops::*};

use derive_more::derive::{Display, From, TryInto};

use crate::{
    Board, Card, Flop, FlopHandCategory, Hand, HandType, InternalError,
    LocInfo, PQLBoardRange, PQLBoolean, PQLCard, PQLCardCount, PQLDouble,
    PQLGame, PQLHiRating, PQLInteger, PQLLong, PQLPlayer, PQLRange, PQLRank,
    PQLRankSet, PQLStreet, PQLString, PQLType, TypeError,
    error::PQLError,
    functions::PQLFn,
    pql_parser::ast::{self, Selector},
    *,
};

mod bin_op;
mod buffer;
pub mod instruction;
mod push_expr;
mod push_fncall;
mod push_ident;
mod push_num;
mod push_selector;
mod push_str;
mod rng;
mod stack;
mod stack_value;
mod stack_value_num;
mod store;
mod store_var_idx;
mod value;

use bin_op::*;
pub(crate) use buffer::*;
pub use instruction::VmInstruction;
use push_expr::push_expr;
use push_selector::push_selector;
pub use rng::*;
pub use stack::VmStack;
pub use stack_value::*;
pub use stack_value_num::VmStackValueNum;
pub use store::VmStore;
pub use store_var_idx::*;
pub use value::*;

pub type VmInstructions = Vec<VmInstruction>;

#[derive(Debug, Clone, Default)]
pub struct Vm {
    pub(crate) board_range: PQLRange,
    pub(crate) player_ranges: Vec<PQLRange>,
    pub(crate) instructions: VmInstructions,
    pub(crate) store: VmStore,
    pub(crate) buffer: VmBuffer,
    pub(crate) stack: VmStack,
    pub(crate) rng: Rng,
    pub n_trials: usize,
    pub n_failed: usize,
}

impl Vm {
    fn sample_next_frame(&mut self) -> Option<()> {
        self.rng.reset();

        for (i, range) in self.player_ranges.iter_mut().enumerate() {
            self.rng.deal(range)?;

            self.buffer.player_hands[i].clone_from(&self.rng.mem);
        }

        self.rng.deal(&mut self.board_range)?;

        let cards = &self.rng.mem;
        self.buffer.board = Board::from_slice(&cards[..5]);

        Some(())
    }

    fn compute(&mut self) -> Result<(), PQLError> {
        let stack = &mut self.stack;

        for ins in &self.instructions {
            ins.execute(&mut self.buffer, &mut self.store, stack)?;
        }

        Ok(())
    }

    pub fn try_run(mut self) -> Result<Self, PQLError> {
        let mut n = self.n_trials;

        while n > 0 && self.n_failed < self.n_trials {
            if self.sample_next_frame() == Some(()) {
                self.compute()?;

                n -= 1;
            } else {
                self.n_failed += 1;
            }
        }

        Ok(self)
    }
}

#[derive(Debug, Clone, Default)]
struct InitDeps<'init, 'input> {
    pub game: PQLGame,
    pub player_names: &'init [&'input str],
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    impl Vm {
        pub fn new_test_vm() -> Self {
            let mut buffer = VmBuffer::default();
            buffer.player_hands.push(Vec::new());

            Self {
                board_range: PQLBoardRange::default().into(),
                player_ranges: [PQLRange::default()].into(),
                instructions: [].into(),
                store: VmStore::default(),
                buffer,
                stack: VmStack::default(),
                rng: Rng::default(),
                n_trials: 1,
                n_failed: 0,
            }
        }
    }

    fn create_vm_shortdeck_cards() -> Vm {
        let mut vm = Vm::new_test_vm();

        vm.rng = Rng::new(Card::ARR_ALL_SHORT.into());

        vm
    }

    #[test]
    fn test_sample_next_frame() {
        fastrand::seed(0);

        let mut vm = create_vm_shortdeck_cards();

        let g = (&vm.buffer).into();

        vm.sample_next_frame().unwrap();
        assert_eq!(Some(flop!("6d6c8d")), vm.buffer.board.flop);

        vm.board_range = PQLBoardRange::from_src("222").unwrap().into();
        assert!(vm.sample_next_frame().is_none());

        vm.player_ranges[0] = PQLRange::from_src("22", g).unwrap();
        assert!(vm.sample_next_frame().is_none());
    }

    #[test]
    fn test_sample_try_run() {
        fastrand::seed(0);

        let mut vm = create_vm_shortdeck_cards();

        vm.instructions =
            vec![VmInstruction::Call(&(functions::turn_card as fn(_) -> _))];

        let mut vm = vm.try_run().unwrap();

        assert_eq!(card!("8h"), vm.stack.downcast_pop::<PQLCard>().unwrap());
    }

    #[test]
    fn test_sample_try_run_error() {
        let mut vm = create_vm_shortdeck_cards();
        let g = (&vm.buffer).into();

        vm.player_ranges[0] = PQLRange::from_src("22", g).unwrap();

        let vm = vm.try_run().unwrap();
        assert_eq!(vm.n_failed, vm.n_trials);

        let mut vm = create_vm_shortdeck_cards();
        vm.store.try_push(PQLString::from("").into()).unwrap();
        vm.instructions = vec![
            VmStackValue::from(VmStoreVarIdx::from(0)).into(),
            VmInstruction::Call(&(functions::rate_hi_hand as fn(&_, _) -> _)),
        ];
        assert!(vm.try_run().is_err());
    }
}
