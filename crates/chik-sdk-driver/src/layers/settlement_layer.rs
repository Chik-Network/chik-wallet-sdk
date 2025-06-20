use chik_puzzle_types::offer::SettlementPaymentsSolution;
use chik_puzzles::SETTLEMENT_PAYMENT_HASH;
use chik_sdk_types::puzzles::SettlementPayment;
use klvm_traits::FromKlvm;
use klvmr::{Allocator, NodePtr};

use crate::{DriverError, Layer, Puzzle, SpendContext};

/// The settlement [`Layer`] is used to spend coins that are part of an offer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SettlementLayer;

impl Layer for SettlementLayer {
    type Solution = SettlementPaymentsSolution;

    fn construct_puzzle(&self, ctx: &mut SpendContext) -> Result<NodePtr, DriverError> {
        ctx.alloc_mod::<SettlementPayment>()
    }

    fn construct_solution(
        &self,
        ctx: &mut SpendContext,
        solution: Self::Solution,
    ) -> Result<NodePtr, DriverError> {
        ctx.alloc(&solution)
    }

    fn parse_puzzle(_allocator: &Allocator, puzzle: Puzzle) -> Result<Option<Self>, DriverError> {
        if puzzle.curried_puzzle_hash() != SETTLEMENT_PAYMENT_HASH.into() {
            return Ok(None);
        }
        Ok(Some(Self))
    }

    fn parse_solution(
        allocator: &Allocator,
        solution: NodePtr,
    ) -> Result<Self::Solution, DriverError> {
        Ok(FromKlvm::from_klvm(allocator, solution)?)
    }
}
