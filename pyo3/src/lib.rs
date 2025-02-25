#![allow(clippy::too_many_arguments)]

use binky::{FromRust, IntoRust, Pyo3, Pyo3Context};
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

    pub fn int(&self, value: BigInt) -> PyResult<Program> {
        Ok(Program::from_rust(
            self.0
                .big_int(IntoRust::<_, _, Pyo3>::into_rust(value, &Pyo3Context)?)?,
            &Pyo3Context,
        )?)
    }
}

#[pymethods]
impl Program {
    pub fn to_int(&self) -> PyResult<Option<BigInt>> {
        Ok(<Option<BigInt> as FromRust<_, _, Pyo3>>::from_rust(
            self.0.to_big_int()?,
            &Pyo3Context,
        )?)
    }
}

pub fn alloc(
    klvm: &chik_sdk_bindings::Klvm,
    value: Bound<'_, PyAny>,
) -> PyResult<chik_sdk_bindings::Program> {
    if let Ok(_value) = value.downcast::<PyNone>() {
        Ok(klvm.nil()?)
    } else if let Ok(value) = value.extract::<BigInt>() {
        Ok(klvm.big_int(value)?)
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
    } else if let Ok(value) = value.extract::<NftMetadata>() {
        Ok(klvm.nft_metadata(value.0.clone())?)
    } else {
        PyResult::Err(PyErr::new::<PyTypeError, _>("Unsupported KLVM value type"))
    }
}
