use chik_protocol::Bytes32;
use klvm_traits::{FromKlvm, ToKlvm};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ToKlvm, FromKlvm)]
#[klvm(list)]
pub struct OptionMetadata {
    pub expiration_seconds: u64,
    pub strike_type: OptionType,
}

impl OptionMetadata {
    pub fn new(expiration_seconds: u64, strike_type: OptionType) -> Self {
        Self {
            expiration_seconds,
            strike_type,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ToKlvm, FromKlvm)]
#[klvm(list)]
#[repr(u8)]
pub enum OptionType {
    Xck {
        amount: u64,
    },
    Cat {
        asset_id: Bytes32,
        amount: u64,
    },
    RevocableCat {
        asset_id: Bytes32,
        hidden_puzzle_hash: Bytes32,
        amount: u64,
    },
    Nft {
        launcher_id: Bytes32,
        settlement_puzzle_hash: Bytes32,
        amount: u64,
    },
}

impl OptionType {
    pub fn amount(&self) -> u64 {
        match self {
            OptionType::Xck { amount }
            | OptionType::Cat { amount, .. }
            | OptionType::RevocableCat { amount, .. }
            | OptionType::Nft { amount, .. } => *amount,
        }
    }

    pub fn is_hinted(&self) -> bool {
        matches!(
            self,
            OptionType::Cat { .. } | OptionType::RevocableCat { .. } | OptionType::Nft { .. }
        )
    }
}
