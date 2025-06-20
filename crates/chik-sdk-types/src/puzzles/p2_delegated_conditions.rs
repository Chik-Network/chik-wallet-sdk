use std::borrow::Cow;

use chik_bls::PublicKey;
use chik_puzzles::{P2_DELEGATED_CONDITIONS, P2_DELEGATED_CONDITIONS_HASH};
use klvm_traits::{FromKlvm, ToKlvm};
use klvm_utils::TreeHash;
use klvmr::NodePtr;

use crate::{Condition, Mod};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToKlvm, FromKlvm)]
#[klvm(curry)]
pub struct P2DelegatedConditionsArgs {
    pub public_key: PublicKey,
}

impl P2DelegatedConditionsArgs {
    pub fn new(public_key: PublicKey) -> Self {
        Self { public_key }
    }
}

impl Mod for P2DelegatedConditionsArgs {
    fn mod_reveal() -> Cow<'static, [u8]> {
        Cow::Borrowed(&P2_DELEGATED_CONDITIONS)
    }

    fn mod_hash() -> TreeHash {
        P2_DELEGATED_CONDITIONS_HASH.into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, ToKlvm, FromKlvm)]
#[klvm(list)]
pub struct P2DelegatedConditionsSolution<T = NodePtr> {
    pub conditions: Vec<Condition<T>>,
}

impl P2DelegatedConditionsSolution {
    pub fn new(conditions: Vec<Condition>) -> Self {
        Self { conditions }
    }
}
