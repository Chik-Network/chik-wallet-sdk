#![allow(clippy::too_many_arguments)]

use binky::{FromRust, Pyo3Context};
use num_bigint::BigInt;
use pyo3::{
    exceptions::PyTypeError,
    prelude::*,
    types::{PyList, PyNone, PyTuple},
};

binky_macro::binky_pyo3!("bindings.json");

#[pymethods]
impl Klvm {
    pub fn alloc(&self, value: Bound<'_, PyAny>) -> PyResult<Program> {
        Ok(Program::from_rust(alloc(&self.0, value)?, &Pyo3Context)?)
    }
}

pub fn alloc(
    klvm: &chik_sdk_bindings::Klvm,
    value: Bound<'_, PyAny>,
) -> PyResult<chik_sdk_bindings::Program> {
    if let Ok(_value) = value.downcast::<PyNone>() {
        Ok(klvm.nil()?)
    } else if let Ok(value) = value.extract::<BigInt>() {
        Ok(klvm.int(value)?)
    } else if let Ok(value) = value.extract::<bool>() {
        Ok(klvm.bool(value)?)
    } else if let Ok(value) = value.extract::<String>() {
        Ok(klvm.string(value)?)
    } else if let Ok(value) = value.extract::<Vec<u8>>() {
        Ok(klvm.atom(value.into())?)
    } else if let Ok(value) = value.extract::<Program>() {
        Ok(value.0)
    } else if let Ok(value) = value.extract::<PublicKey>() {
        Ok(klvm.atom(value.to_bytes()?.to_vec().into())?)
    } else if let Ok(value) = value.extract::<Signature>() {
        Ok(klvm.atom(value.to_bytes()?.to_vec().into())?)
    } else if let Ok(value) = value.extract::<K1PublicKey>() {
        Ok(klvm.atom(value.to_bytes()?.to_vec().into())?)
    } else if let Ok(value) = value.extract::<K1Signature>() {
        Ok(klvm.atom(value.to_bytes()?.to_vec().into())?)
    } else if let Ok(value) = value.extract::<R1PublicKey>() {
        Ok(klvm.atom(value.to_bytes()?.to_vec().into())?)
    } else if let Ok(value) = value.extract::<R1Signature>() {
        Ok(klvm.atom(value.to_bytes()?.to_vec().into())?)
    } else if let Ok(value) = value.extract::<CurriedProgram>() {
        Ok(value.0.program.curry(value.0.args.clone())?)
    } else if let Ok(value) = value.downcast::<PyTuple>() {
        if value.len() != 2 {
            return PyResult::Err(PyErr::new::<PyTypeError, _>(
                "Expected a tuple with 2 items",
            ));
        }

        let first = alloc(klvm, value.get_item(0)?)?;
        let rest = alloc(klvm, value.get_item(1)?)?;

        Ok(klvm.pair(first, rest)?)
    } else if let Ok(value) = value.extract::<Pair>() {
        Ok(klvm.pair(value.0.first, value.0.rest)?)
    } else if let Ok(value) = value.downcast::<PyList>() {
        let mut list = Vec::new();

        for item in value.iter() {
            list.push(alloc(klvm, item)?);
        }

        Ok(klvm.list(list)?)
    } else if let Ok(value) = value.extract::<Remark>() {
        Ok(klvm.remark(value.0.rest)?)
    } else if let Ok(value) = value.extract::<AggSigParent>() {
        Ok(klvm.agg_sig_parent(value.0.public_key, value.0.message)?)
    } else if let Ok(value) = value.extract::<AggSigPuzzle>() {
        Ok(klvm.agg_sig_puzzle(value.0.public_key, value.0.message)?)
    } else if let Ok(value) = value.extract::<AggSigAmount>() {
        Ok(klvm.agg_sig_amount(value.0.public_key, value.0.message)?)
    } else if let Ok(value) = value.extract::<AggSigPuzzleAmount>() {
        Ok(klvm.agg_sig_puzzle_amount(value.0.public_key, value.0.message)?)
    } else if let Ok(value) = value.extract::<AggSigParentAmount>() {
        Ok(klvm.agg_sig_parent_amount(value.0.public_key, value.0.message)?)
    } else if let Ok(value) = value.extract::<AggSigParentPuzzle>() {
        Ok(klvm.agg_sig_parent_puzzle(value.0.public_key, value.0.message)?)
    } else if let Ok(value) = value.extract::<AggSigUnsafe>() {
        Ok(klvm.agg_sig_unsafe(value.0.public_key, value.0.message)?)
    } else if let Ok(value) = value.extract::<AggSigMe>() {
        Ok(klvm.agg_sig_me(value.0.public_key, value.0.message)?)
    } else if let Ok(value) = value.extract::<CreateCoin>() {
        Ok(klvm.create_coin(value.0.puzzle_hash, value.0.amount, value.0.memos)?)
    } else if let Ok(value) = value.extract::<ReserveFee>() {
        Ok(klvm.reserve_fee(value.0.amount)?)
    } else if let Ok(value) = value.extract::<CreateCoinAnnouncement>() {
        Ok(klvm.create_coin_announcement(value.0.message)?)
    } else if let Ok(value) = value.extract::<CreatePuzzleAnnouncement>() {
        Ok(klvm.create_puzzle_announcement(value.0.message)?)
    } else if let Ok(value) = value.extract::<AssertCoinAnnouncement>() {
        Ok(klvm.assert_coin_announcement(value.0.announcement_id)?)
    } else if let Ok(value) = value.extract::<AssertPuzzleAnnouncement>() {
        Ok(klvm.assert_puzzle_announcement(value.0.announcement_id)?)
    } else if let Ok(value) = value.extract::<AssertConcurrentSpend>() {
        Ok(klvm.assert_concurrent_spend(value.0.coin_id)?)
    } else if let Ok(value) = value.extract::<AssertConcurrentPuzzle>() {
        Ok(klvm.assert_concurrent_puzzle(value.0.puzzle_hash)?)
    } else if let Ok(value) = value.extract::<AssertSecondsRelative>() {
        Ok(klvm.assert_seconds_relative(value.0.seconds)?)
    } else if let Ok(value) = value.extract::<AssertSecondsAbsolute>() {
        Ok(klvm.assert_seconds_absolute(value.0.seconds)?)
    } else if let Ok(value) = value.extract::<AssertHeightRelative>() {
        Ok(klvm.assert_height_relative(value.0.height)?)
    } else if let Ok(value) = value.extract::<AssertHeightAbsolute>() {
        Ok(klvm.assert_height_absolute(value.0.height)?)
    } else if let Ok(value) = value.extract::<AssertBeforeSecondsRelative>() {
        Ok(klvm.assert_before_seconds_relative(value.0.seconds)?)
    } else if let Ok(value) = value.extract::<AssertBeforeSecondsAbsolute>() {
        Ok(klvm.assert_before_seconds_absolute(value.0.seconds)?)
    } else if let Ok(value) = value.extract::<AssertBeforeHeightRelative>() {
        Ok(klvm.assert_before_height_relative(value.0.height)?)
    } else if let Ok(value) = value.extract::<AssertBeforeHeightAbsolute>() {
        Ok(klvm.assert_before_height_absolute(value.0.height)?)
    } else if let Ok(value) = value.extract::<AssertMyCoinId>() {
        Ok(klvm.assert_my_coin_id(value.0.coin_id)?)
    } else if let Ok(value) = value.extract::<AssertMyParentId>() {
        Ok(klvm.assert_my_parent_id(value.0.parent_id)?)
    } else if let Ok(value) = value.extract::<AssertMyPuzzleHash>() {
        Ok(klvm.assert_my_puzzle_hash(value.0.puzzle_hash)?)
    } else if let Ok(value) = value.extract::<AssertMyAmount>() {
        Ok(klvm.assert_my_amount(value.0.amount)?)
    } else if let Ok(value) = value.extract::<AssertMyBirthSeconds>() {
        Ok(klvm.assert_my_birth_seconds(value.0.seconds)?)
    } else if let Ok(value) = value.extract::<AssertMyBirthHeight>() {
        Ok(klvm.assert_my_birth_height(value.0.height)?)
    } else if let Ok(_value) = value.extract::<AssertEphemeral>() {
        Ok(klvm.assert_ephemeral()?)
    } else if let Ok(value) = value.extract::<SendMessage>() {
        Ok(klvm.send_message(value.0.mode, value.0.message, value.0.data)?)
    } else if let Ok(value) = value.extract::<ReceiveMessage>() {
        Ok(klvm.receive_message(value.0.mode, value.0.message, value.0.data)?)
    } else if let Ok(value) = value.extract::<Softfork>() {
        Ok(klvm.softfork(value.0.cost, value.0.rest)?)
    } else if let Ok(_value) = value.extract::<MeltSingleton>() {
        Ok(klvm.melt_singleton()?)
    } else if let Ok(value) = value.extract::<TransferNft>() {
        Ok(klvm.transfer_nft(
            value.0.launcher_id,
            value.0.trade_prices.clone(),
            value.0.singleton_inner_puzzle_hash,
        )?)
    } else if let Ok(value) = value.extract::<RunCatTail>() {
        Ok(klvm.run_cat_tail(value.0.program.clone(), value.0.solution.clone())?)
    } else if let Ok(value) = value.extract::<UpdateNftMetadata>() {
        Ok(klvm.update_nft_metadata(
            value.0.updater_puzzle_reveal.clone(),
            value.0.updater_solution.clone(),
        )?)
    } else if let Ok(value) = value.extract::<UpdateDataStoreMerkleRoot>() {
        Ok(klvm.update_data_store_merkle_root(value.0.new_merkle_root, value.0.memos.clone())?)
    } else if let Ok(value) = value.extract::<NftMetadata>() {
        Ok(klvm.nft_metadata(value.0.clone())?)
    } else if let Ok(value) = value.extract::<MipsMemo>() {
        Ok(klvm.mips_memo(value.0.clone())?)
    } else if let Ok(value) = value.extract::<InnerPuzzleMemo>() {
        Ok(klvm.inner_puzzle_memo(value.0.clone())?)
    } else if let Ok(value) = value.extract::<RestrictionMemo>() {
        Ok(klvm.restriction_memo(value.0.clone())?)
    } else if let Ok(value) = value.extract::<WrapperMemo>() {
        Ok(klvm.wrapper_memo(value.0.clone())?)
    } else if let Ok(value) = value.extract::<Force1of2RestrictedVariableMemo>() {
        Ok(klvm.force_1_of_2_restricted_variable_memo(value.0.clone())?)
    } else if let Ok(value) = value.extract::<MemoKind>() {
        Ok(klvm.memo_kind(value.0.clone())?)
    } else if let Ok(value) = value.extract::<MemberMemo>() {
        Ok(klvm.member_memo(value.0.clone())?)
    } else if let Ok(value) = value.extract::<MofNMemo>() {
        Ok(klvm.m_of_n_memo(value.0.clone())?)
    } else {
        PyResult::Err(PyErr::new::<PyTypeError, _>("Unsupported KLVM value type"))
    }
}
