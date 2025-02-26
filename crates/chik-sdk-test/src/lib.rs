mod announcements;
mod error;
mod events;
mod keys;
mod peer_simulator;
mod simulator;
mod transaction;

pub use announcements::*;
pub use error::*;
pub use events::*;
pub use keys::*;
pub use peer_simulator::*;
pub use simulator::*;
pub use transaction::*;

use chik_protocol::{Bytes32, Program};
use klvm_traits::{FromKlvm, ToKlvm};
use klvm_utils::tree_hash;
use klvmr::Allocator;

pub fn to_program(value: impl ToKlvm<Allocator>) -> anyhow::Result<Program> {
    let mut allocator = Allocator::new();
    let ptr = value.to_klvm(&mut allocator)?;
    Ok(Program::from_klvm(&allocator, ptr)?)
}

pub fn to_puzzle(value: impl ToKlvm<Allocator>) -> anyhow::Result<(Bytes32, Program)> {
    let mut allocator = Allocator::new();
    let ptr = value.to_klvm(&mut allocator)?;
    let puzzle_reveal = Program::from_klvm(&allocator, ptr)?;
    let puzzle_hash = tree_hash(&allocator, ptr);
    Ok((puzzle_hash.into(), puzzle_reveal))
}
