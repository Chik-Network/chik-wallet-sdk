use chik_bls::PublicKey;
use chik_puzzles::P2_DELEGATED_CONDITIONS_HASH;
use chik_sdk_types::puzzles::{P2DelegatedConditionsArgs, P2DelegatedConditionsSolution};
use klvm_traits::FromKlvm;
use klvmr::{Allocator, NodePtr};

use crate::{DriverError, Layer, Puzzle, SpendContext};

/// The p2 delegated conditions [`Layer`] allows a certain key to spend the coin.
/// To do so, a list of additional conditions is signed and passed in the solution.
/// Typically, the [`StandardLayer`](crate::StandardLayer) is used instead, since it adds more flexibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct P2DelegatedConditionsLayer {
    /// The public key that has the ability to spend the coin.
    pub public_key: PublicKey,
}

impl Layer for P2DelegatedConditionsLayer {
    type Solution = P2DelegatedConditionsSolution;

    fn construct_puzzle(&self, ctx: &mut SpendContext) -> Result<NodePtr, DriverError> {
        ctx.curry(P2DelegatedConditionsArgs::new(self.public_key))
    }

    fn construct_solution(
        &self,
        ctx: &mut SpendContext,
        solution: Self::Solution,
    ) -> Result<NodePtr, DriverError> {
        ctx.alloc(&solution)
    }

    fn parse_puzzle(allocator: &Allocator, puzzle: Puzzle) -> Result<Option<Self>, DriverError> {
        let Some(puzzle) = puzzle.as_curried() else {
            return Ok(None);
        };

        if puzzle.mod_hash != P2_DELEGATED_CONDITIONS_HASH.into() {
            return Ok(None);
        }

        let args = P2DelegatedConditionsArgs::from_klvm(allocator, puzzle.args)?;

        Ok(Some(Self {
            public_key: args.public_key,
        }))
    }

    fn parse_solution(
        allocator: &Allocator,
        solution: NodePtr,
    ) -> Result<Self::Solution, DriverError> {
        Ok(P2DelegatedConditionsSolution::from_klvm(
            allocator, solution,
        )?)
    }
}
