use chik_protocol::Bytes32;
use klvm_traits::{FromKlvm, ToKlvm};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToKlvm, FromKlvm)]
#[klvm(list)]
pub struct NewMetadataInfo<M> {
    pub new_metadata: M,
    pub new_updater_puzzle_hash: Bytes32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToKlvm, FromKlvm)]
#[klvm(list)]
pub struct NewMetadataOutput<M, C> {
    pub metadata_info: NewMetadataInfo<M>,
    pub conditions: C,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToKlvm, FromKlvm)]
#[klvm(list)]
pub struct TradePrice {
    pub amount: u64,
    pub puzzle_hash: Bytes32,
}
