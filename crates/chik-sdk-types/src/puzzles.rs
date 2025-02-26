mod augmented_condition;
mod p2_curried;
mod p2_delegated_conditions;
mod p2_delegated_singleton;
mod p2_one_of_many;
mod p2_singleton;

pub use augmented_condition::*;
pub use p2_curried::*;
pub use p2_delegated_conditions::*;
pub use p2_delegated_singleton::*;
pub use p2_one_of_many::*;
pub use p2_singleton::*;

#[cfg(feature = "chip-0035")]
mod datalayer;

#[cfg(feature = "chip-0035")]
pub use datalayer::*;

#[cfg(feature = "experimental-vaults")]
mod vault;

#[cfg(feature = "experimental-vaults")]
pub use vault::*;

#[cfg(test)]
mod tests {
    #[macro_export]
    macro_rules! assert_puzzle_hash {
        ($puzzle:ident => $puzzle_hash:ident) => {
            let mut a = klvmr::Allocator::new();
            let ptr = klvmr::serde::node_from_bytes(&mut a, &$puzzle)?;
            let hash = klvm_utils::tree_hash(&mut a, ptr);
            assert_eq!($puzzle_hash, hash);
        };
    }
}
