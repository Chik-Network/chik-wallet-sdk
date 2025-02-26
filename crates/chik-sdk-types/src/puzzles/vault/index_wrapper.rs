use hex_literal::hex;
use klvm_traits::{FromKlvm, ToKlvm};
use klvm_utils::TreeHash;

use crate::Mod;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToKlvm, FromKlvm)]
#[klvm(curry)]
pub struct IndexWrapperArgs<I> {
    pub index: usize,
    pub inner_puzzle: I,
}

impl<I> IndexWrapperArgs<I> {
    pub fn new(index: usize, inner_puzzle: I) -> Self {
        Self {
            index,
            inner_puzzle,
        }
    }
}

impl<I> Mod for IndexWrapperArgs<I> {
    const MOD_REVEAL: &[u8] = &INDEX_WRAPPER;
    const MOD_HASH: TreeHash = INDEX_WRAPPER_HASH;
}

pub const INDEX_WRAPPER: [u8; 7] = hex!("ff02ff05ff0780");

pub const INDEX_WRAPPER_HASH: TreeHash = TreeHash::new(hex!(
    "847d971ef523417d555ea9854b1612837155d34d453298defcd310774305f657"
));
