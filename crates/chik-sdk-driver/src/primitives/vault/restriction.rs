use chik_sdk_types::{Force1of2RestrictedVariable, Timelock};
use klvm_utils::TreeHash;
use klvmr::NodePtr;

use crate::{DriverError, SpendContext};

use super::{KnownPuzzles, VaultLayer};

#[derive(Debug, Clone, Copy)]
pub struct Restriction {
    puzzle_hash: TreeHash,
    is_member_condition_validator: bool,
    kind: RestrictionKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestrictionKind {
    Timelock(Timelock),
    ForceCoinMessage,
    ForceAssertCoinAnnouncement,
    Force1of2RestrictedVariable(Force1of2RestrictedVariable),
    Unknown,
}

impl Restriction {
    pub fn new(
        puzzle_hash: TreeHash,
        is_member_condition_validator: bool,
        kind: RestrictionKind,
    ) -> Self {
        Self {
            puzzle_hash,
            is_member_condition_validator,
            kind,
        }
    }

    pub fn is_member_condition_validator(&self) -> bool {
        self.is_member_condition_validator
    }

    pub fn kind(&self) -> RestrictionKind {
        self.kind
    }
}

impl VaultLayer for Restriction {
    fn puzzle_hash(&self) -> TreeHash {
        self.puzzle_hash
    }

    fn puzzle(&self, ctx: &mut SpendContext) -> Result<NodePtr, DriverError> {
        match &self.kind {
            RestrictionKind::Timelock(timelock) => ctx.curry(timelock),
            RestrictionKind::ForceCoinMessage => ctx.force_coin_message_puzzle(),
            RestrictionKind::ForceAssertCoinAnnouncement => {
                ctx.force_assert_coin_announcement_puzzle()
            }
            RestrictionKind::Force1of2RestrictedVariable(restriction) => ctx.curry(restriction),
            RestrictionKind::Unknown => Err(DriverError::UnknownPuzzle),
        }
    }

    fn replace(self, known_puzzles: &KnownPuzzles) -> Self {
        let kind = known_puzzles
            .restrictions
            .get(&self.puzzle_hash)
            .copied()
            .unwrap_or(self.kind);

        Self {
            puzzle_hash: self.puzzle_hash,
            is_member_condition_validator: self.is_member_condition_validator,
            kind,
        }
    }
}
