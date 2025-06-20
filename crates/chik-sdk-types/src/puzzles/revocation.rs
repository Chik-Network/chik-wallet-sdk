use std::borrow::Cow;

use chik_protocol::Bytes32;
use chik_puzzles::{REVOCATION_LAYER, REVOCATION_LAYER_HASH};
use klvm_traits::{FromKlvm, ToKlvm};
use klvm_utils::TreeHash;

use crate::Mod;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToKlvm, FromKlvm)]
#[klvm(curry)]
pub struct RevocationArgs {
    pub mod_hash: Bytes32,
    pub hidden_puzzle_hash: Bytes32,
    pub inner_puzzle_hash: Bytes32,
}

impl RevocationArgs {
    pub fn new(hidden_puzzle_hash: Bytes32, inner_puzzle_hash: Bytes32) -> Self {
        Self {
            mod_hash: REVOCATION_LAYER_HASH.into(),
            hidden_puzzle_hash,
            inner_puzzle_hash,
        }
    }
}

impl Mod for RevocationArgs {
    fn mod_reveal() -> Cow<'static, [u8]> {
        Cow::Borrowed(&REVOCATION_LAYER)
    }

    fn mod_hash() -> TreeHash {
        TreeHash::new(REVOCATION_LAYER_HASH)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToKlvm, FromKlvm)]
#[klvm(list)]
pub struct RevocationSolution<P, S> {
    pub hidden: bool,
    pub puzzle: P,
    pub solution: S,
}

impl<P, S> RevocationSolution<P, S> {
    pub fn new(hidden: bool, puzzle: P, solution: S) -> Self {
        Self {
            hidden,
            puzzle,
            solution,
        }
    }
}
