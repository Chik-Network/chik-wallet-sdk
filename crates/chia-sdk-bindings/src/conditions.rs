use std::sync::{Arc, RwLock};

use bindy::Result;
use chia_bls as bls;
use chia_protocol::{Bytes, Bytes32};
use chia_sdk_driver::SpendContext;
use chia_sdk_types::{self as types, Memos};
use clvm_traits::{FromClvm, ToClvm};
use clvmr::NodePtr;
use paste::paste;

use crate::{Clvm, Program, PublicKey};

trait Convert<T> {
    fn convert(self, clvm: &Arc<RwLock<SpendContext>>) -> Result<T>;
}

impl Convert<Program> for NodePtr {
    fn convert(self, clvm: &Arc<RwLock<SpendContext>>) -> Result<Program> {
        Ok(Program(clvm.clone(), self))
    }
}

impl Convert<NodePtr> for Program {
    fn convert(self, _clvm: &Arc<RwLock<SpendContext>>) -> Result<NodePtr> {
        Ok(self.1)
    }
}

impl Convert<PublicKey> for bls::PublicKey {
    fn convert(self, _clvm: &Arc<RwLock<SpendContext>>) -> Result<PublicKey> {
        Ok(PublicKey(self))
    }
}

impl Convert<bls::PublicKey> for PublicKey {
    fn convert(self, _clvm: &Arc<RwLock<SpendContext>>) -> Result<bls::PublicKey> {
        Ok(self.0)
    }
}

impl Convert<Bytes> for Bytes {
    fn convert(self, _clvm: &Arc<RwLock<SpendContext>>) -> Result<Bytes> {
        Ok(self)
    }
}

impl Convert<Bytes32> for Bytes32 {
    fn convert(self, _clvm: &Arc<RwLock<SpendContext>>) -> Result<Bytes32> {
        Ok(self)
    }
}

impl Convert<u64> for u64 {
    fn convert(self, _clvm: &Arc<RwLock<SpendContext>>) -> Result<u64> {
        Ok(self)
    }
}

impl Convert<u32> for u32 {
    fn convert(self, _clvm: &Arc<RwLock<SpendContext>>) -> Result<u32> {
        Ok(self)
    }
}

impl Convert<u8> for u8 {
    fn convert(self, _clvm: &Arc<RwLock<SpendContext>>) -> Result<u8> {
        Ok(self)
    }
}

impl Convert<Memos<NodePtr>> for Program {
    fn convert(self, _clvm: &Arc<RwLock<SpendContext>>) -> Result<Memos<NodePtr>> {
        Ok(Memos::new(self.1))
    }
}

impl Convert<Program> for Memos<NodePtr> {
    fn convert(self, clvm: &Arc<RwLock<SpendContext>>) -> Result<Program> {
        Ok(Program(clvm.clone(), self.value))
    }
}

impl<T, U> Convert<Vec<U>> for Vec<T>
where
    T: Convert<U>,
{
    fn convert(self, clvm: &Arc<RwLock<SpendContext>>) -> Result<Vec<U>> {
        self.into_iter()
            .map(|value| T::convert(value, clvm))
            .collect()
    }
}

impl<T, U> Convert<Option<U>> for Option<T>
where
    T: Convert<U>,
{
    fn convert(self, clvm: &Arc<RwLock<SpendContext>>) -> Result<Option<U>> {
        self.map(|value| T::convert(value, clvm)).transpose()
    }
}

macro_rules! conditions {
    ( $( $condition:ident $( < $( $generic:ty ),* > )? { $function:ident( $( $name:ident: $ty:ty ),* ) }, )* ) => {
        $( #[derive(Clone)]
        pub struct $condition {
            $( pub $name: $ty, )*
        } )*

        $( paste! {
            impl Clvm {
                pub fn $function( &self, $( $name: $ty ),* ) -> Result<Program> {
                    let mut ctx = self.0.write().unwrap();
                    $( let $name = Convert::convert($name, &self.0)?; )*
                    let ptr = types::$condition $( ::< $( $generic ),* > )? ::new( $( $name ),* )
                    .to_clvm(&mut ctx.allocator)?;
                    Ok(Program(self.0.clone(), ptr))
                }
            }

            impl Program {
                #[allow(unused)]
                pub fn [< parse_ $function >]( &self ) -> Result<Option<$condition>> {
                    let ctx = self.0.read().unwrap();

                    let Some(condition) = types::$condition $( ::< $( $generic ),* > )? ::from_clvm(&ctx.allocator, self.1).ok() else {
                        return Ok(None);
                    };

                    Ok(Some($condition {
                        $( $name: Convert::convert(condition.$name, &self.0.clone())?, )*
                    }))
                }
            }
        } )*
    };
}

conditions!(
    Remark<NodePtr> {
        remark(rest: Program)
    },
    AggSigParent {
        agg_sig_parent(public_key: PublicKey, message: Bytes)
    },
    AggSigPuzzle {
        agg_sig_puzzle(public_key: PublicKey, message: Bytes)
    },
    AggSigAmount {
        agg_sig_amount(public_key: PublicKey, message: Bytes)
    },
    AggSigPuzzleAmount {
        agg_sig_puzzle_amount(public_key: PublicKey, message: Bytes)
    },
    AggSigParentAmount {
        agg_sig_parent_amount(public_key: PublicKey, message: Bytes)
    },
    AggSigParentPuzzle {
        agg_sig_parent_puzzle(public_key: PublicKey, message: Bytes)
    },
    AggSigUnsafe {
        agg_sig_unsafe(public_key: PublicKey, message: Bytes)
    },
    AggSigMe {
        agg_sig_me(public_key: PublicKey, message: Bytes)
    },
    CreateCoin {
        create_coin(puzzle_hash: Bytes32, amount: u64, memos: Option<Program>)
    },
    ReserveFee {
        reserve_fee(amount: u64)
    },
    CreateCoinAnnouncement {
        create_coin_announcement(message: Bytes)
    },
    CreatePuzzleAnnouncement {
        create_puzzle_announcement(message: Bytes)
    },
    AssertCoinAnnouncement {
        assert_coin_announcement(announcement_id: Bytes32)
    },
    AssertPuzzleAnnouncement {
        assert_puzzle_announcement(announcement_id: Bytes32)
    },
    AssertConcurrentSpend {
        assert_concurrent_spend(coin_id: Bytes32)
    },
    AssertConcurrentPuzzle {
        assert_concurrent_puzzle(puzzle_hash: Bytes32)
    },
    AssertSecondsRelative {
        assert_seconds_relative(seconds: u64)
    },
    AssertSecondsAbsolute {
        assert_seconds_absolute(seconds: u64)
    },
    AssertHeightRelative {
        assert_height_relative(height: u32)
    },
    AssertHeightAbsolute {
        assert_height_absolute(height: u32)
    },
    AssertBeforeSecondsRelative {
        assert_before_seconds_relative(seconds: u64)
    },
    AssertBeforeSecondsAbsolute {
        assert_before_seconds_absolute(seconds: u64)
    },
    AssertBeforeHeightRelative {
        assert_before_height_relative(height: u32)
    },
    AssertBeforeHeightAbsolute {
        assert_before_height_absolute(height: u32)
    },
    AssertMyCoinId {
        assert_my_coin_id(coin_id: Bytes32)
    },
    AssertMyParentId {
        assert_my_parent_id(parent_id: Bytes32)
    },
    AssertMyPuzzleHash {
        assert_my_puzzle_hash(puzzle_hash: Bytes32)
    },
    AssertMyAmount {
        assert_my_amount(amount: u64)
    },
    AssertMyBirthSeconds {
        assert_my_birth_seconds(seconds: u64)
    },
    AssertMyBirthHeight {
        assert_my_birth_height(height: u32)
    },
    AssertEphemeral {
        assert_ephemeral()
    },
    SendMessage<NodePtr> {
        send_message(mode: u8, message: Bytes, data: Vec<Program>)
    },
    ReceiveMessage<NodePtr> {
        receive_message(mode: u8, message: Bytes, data: Vec<Program>)
    },
    Softfork<NodePtr> {
        softfork(cost: u64, rest: Program)
    },
);
