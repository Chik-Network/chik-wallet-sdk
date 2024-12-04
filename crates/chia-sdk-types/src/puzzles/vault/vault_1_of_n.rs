use chia_protocol::Bytes32;
use clvm_traits::{FromClvm, ToClvm};
use clvm_utils::TreeHash;
use clvmr::NodePtr;
use hex_literal::hex;

use crate::{MerkleProof, Mod};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToClvm, FromClvm)]
#[clvm(curry)]
pub struct Vault1ofNArgs {
    pub merkle_root: Bytes32,
}

impl Vault1ofNArgs {
    pub fn new(merkle_root: Bytes32) -> Self {
        Self { merkle_root }
    }
}

impl Mod for Vault1ofNArgs {
    const MOD_REVEAL: &[u8] = &VAULT_1_OF_N_PUZZLE;
    const MOD_HASH: TreeHash = VAULT_1_OF_N_PUZZLE_HASH;
    type Solution = Vault1ofNSolution<NodePtr, NodePtr>;
}

#[derive(Debug, Clone, PartialEq, Eq, ToClvm, FromClvm)]
#[clvm(solution)]
pub struct Vault1ofNSolution<P, S> {
    pub delegated_puzzle_hash: Bytes32,
    pub merkle_proof: MerkleProof,
    pub member_puzzle: P,
    pub member_solution: S,
}

impl<P, S> Vault1ofNSolution<P, S> {
    pub fn new(
        delegated_puzzle_hash: Bytes32,
        merkle_proof: MerkleProof,
        member_puzzle: P,
        member_solution: S,
    ) -> Self {
        Self {
            delegated_puzzle_hash,
            merkle_proof,
            member_puzzle,
            member_solution,
        }
    }
}

pub const VAULT_1_OF_N_PUZZLE: [u8; 286] = hex!(
    "
    ff02ffff01ff02ffff03ffff09ff05ffff02ff06ffff04ff02ffff04ffff0bff
    ff0101ffff02ff04ffff04ff02ffff04ff2fff8080808080ffff04ff17ff8080
    80808080ffff01ff02ff2fffff04ff0bff5f8080ffff01ff088080ff0180ffff
    04ffff01ffff02ffff03ffff07ff0580ffff01ff0bffff0102ffff02ff04ffff
    04ff02ffff04ff09ff80808080ffff02ff04ffff04ff02ffff04ff0dff808080
    8080ffff01ff0bffff0101ff058080ff0180ff02ffff03ff1bffff01ff02ff06
    ffff04ff02ffff04ffff02ffff03ffff18ffff0101ff1380ffff01ff0bffff01
    02ff2bff0580ffff01ff0bffff0102ff05ff2b8080ff0180ffff04ffff04ffff
    17ff13ffff0181ff80ff3b80ff8080808080ffff010580ff0180ff018080
    "
);

pub const VAULT_1_OF_N_PUZZLE_HASH: TreeHash = TreeHash::new(hex!(
    "bcb9aa74893bebcfa2da87271b0330bf2773b6391144ae72262b6824d9c55939"
));
