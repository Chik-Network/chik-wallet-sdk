#![allow(unsafe_code)]
#![allow(clippy::wildcard_imports)]

use bindy::{FromRust, IntoRust, NapiParamContext, NapiReturnContext};
use napi::bindgen_prelude::*;
use napi_derive::napi;

bindy_macro::bindy_napi!("bindings.json");

#[napi]
impl Clvm {
    #[napi]
    pub fn alloc(&self, env: Env, value: Value<'_>) -> Result<Program> {
        Ok(Program::from_rust(
            alloc(env, &self.0, value)?,
            &NapiReturnContext(env),
        )?)
    }

    #[napi]
    pub fn int(&self, env: Env, value: f64) -> Result<Program> {
        Ok(Program::from_rust(
            self.0.f64(value)?,
            &NapiReturnContext(env),
        )?)
    }

    #[napi]
    pub fn big_int(&self, env: Env, value: BigInt) -> Result<Program> {
        Ok(Program::from_rust(
            self.0.big_int(value.into_rust(&NapiParamContext)?)?,
            &NapiReturnContext(env),
        )?)
    }
}

#[napi]
impl Program {
    #[napi]
    pub fn to_int(&self) -> Result<Option<f64>> {
        Ok(self.0.to_small_int()?)
    }

    #[napi]
    pub fn to_big_int(&self, env: Env) -> Result<Option<BigInt>> {
        Ok(Option::<BigInt>::from_rust(
            self.0.to_big_int()?,
            &NapiReturnContext(env),
        )?)
    }
}

pub type Value<'a> = Either26<
    f64,
    BigInt,
    bool,
    String,
    Uint8Array,
    Array,
    Null,
    ClassInstance<'a, Program>,
    ClassInstance<'a, PublicKey>,
    ClassInstance<'a, Signature>,
    ClassInstance<'a, K1PublicKey>,
    ClassInstance<'a, K1Signature>,
    ClassInstance<'a, R1PublicKey>,
    ClassInstance<'a, R1Signature>,
    ClassInstance<'a, Remark>,
    ClassInstance<'a, AggSigParent>,
    ClassInstance<'a, AggSigPuzzle>,
    ClassInstance<'a, AggSigAmount>,
    ClassInstance<'a, AggSigPuzzleAmount>,
    ClassInstance<'a, AggSigParentAmount>,
    ClassInstance<'a, AggSigParentPuzzle>,
    ClassInstance<'a, AggSigUnsafe>,
    ClassInstance<'a, AggSigMe>,
    ClassInstance<'a, CreateCoin>,
    ClassInstance<'a, ReserveFee>,
    Value2<'a>,
>;

type Value2<'a> = Either26<
    ClassInstance<'a, CreateCoinAnnouncement>,
    ClassInstance<'a, CreatePuzzleAnnouncement>,
    ClassInstance<'a, AssertCoinAnnouncement>,
    ClassInstance<'a, AssertPuzzleAnnouncement>,
    ClassInstance<'a, AssertConcurrentSpend>,
    ClassInstance<'a, AssertConcurrentPuzzle>,
    ClassInstance<'a, AssertSecondsRelative>,
    ClassInstance<'a, AssertSecondsAbsolute>,
    ClassInstance<'a, AssertHeightRelative>,
    ClassInstance<'a, AssertHeightAbsolute>,
    ClassInstance<'a, AssertBeforeSecondsRelative>,
    ClassInstance<'a, AssertBeforeSecondsAbsolute>,
    ClassInstance<'a, AssertBeforeHeightRelative>,
    ClassInstance<'a, AssertBeforeHeightAbsolute>,
    ClassInstance<'a, AssertMyCoinId>,
    ClassInstance<'a, AssertMyParentId>,
    ClassInstance<'a, AssertMyPuzzleHash>,
    ClassInstance<'a, AssertMyAmount>,
    ClassInstance<'a, AssertMyBirthSeconds>,
    ClassInstance<'a, AssertMyBirthHeight>,
    ClassInstance<'a, AssertEphemeral>,
    ClassInstance<'a, SendMessage>,
    ClassInstance<'a, ReceiveMessage>,
    ClassInstance<'a, Softfork>,
    ClassInstance<'a, Pair>,
    ClassInstance<'a, NftMetadata>,
>;

fn alloc<'a>(
    env: Env,
    clvm: &chia_sdk_bindings::Clvm,
    value: Value<'a>,
) -> bindy::Result<chia_sdk_bindings::Program> {
    match value {
        Value::A(value) => clvm.f64(value),
        Value::B(value) => clvm.big_int(value.into_rust(&NapiParamContext)?),
        Value::C(value) => clvm.bool(value),
        Value::D(value) => clvm.string(value),
        Value::E(value) => clvm.atom(value.to_vec().into()),
        Value::F(value) => {
            let mut list = Vec::new();

            for index in 0..value.len() {
                let item = value.get::<Value<'a>>(index)?.unwrap();
                list.push(alloc(env, clvm, item)?);
            }

            Ok(clvm.list(list)?)
        }
        Value::G(_) => clvm.nil(),
        Value::H(value) => Ok(value.0.clone()),
        Value::I(value) => clvm.atom(value.to_bytes(env)?.to_vec().into()),
        Value::J(value) => clvm.atom(value.to_bytes(env)?.to_vec().into()),
        Value::K(value) => clvm.atom(value.to_bytes(env)?.to_vec().into()),
        Value::L(value) => clvm.atom(value.to_bytes(env)?.to_vec().into()),
        Value::M(value) => clvm.atom(value.to_bytes(env)?.to_vec().into()),
        Value::N(value) => clvm.atom(value.to_bytes(env)?.to_vec().into()),
        Value::O(value) => clvm.remark(value.0.rest.clone()),
        Value::P(value) => clvm.agg_sig_parent(value.0.public_key.clone(), value.0.message.clone()),
        Value::Q(value) => clvm.agg_sig_puzzle(value.0.public_key.clone(), value.0.message.clone()),
        Value::R(value) => clvm.agg_sig_amount(value.0.public_key.clone(), value.0.message.clone()),
        Value::S(value) => {
            clvm.agg_sig_puzzle_amount(value.0.public_key.clone(), value.0.message.clone())
        }
        Value::T(value) => {
            clvm.agg_sig_parent_amount(value.0.public_key.clone(), value.0.message.clone())
        }
        Value::U(value) => {
            clvm.agg_sig_parent_puzzle(value.0.public_key.clone(), value.0.message.clone())
        }
        Value::V(value) => clvm.agg_sig_unsafe(value.0.public_key.clone(), value.0.message.clone()),
        Value::W(value) => clvm.agg_sig_me(value.0.public_key.clone(), value.0.message.clone()),
        Value::X(value) => clvm.create_coin(
            value.0.puzzle_hash.clone(),
            value.0.amount,
            value.0.memos.clone(),
        ),
        Value::Y(value) => clvm.reserve_fee(value.0.amount),
        Value::Z(value) => match value {
            Value2::A(value) => clvm.create_coin_announcement(value.0.message.clone()),
            Value2::B(value) => clvm.create_puzzle_announcement(value.0.message.clone()),
            Value2::C(value) => clvm.assert_coin_announcement(value.0.announcement_id.clone()),
            Value2::D(value) => clvm.assert_puzzle_announcement(value.0.announcement_id.clone()),
            Value2::E(value) => clvm.assert_concurrent_spend(value.0.coin_id.clone()),
            Value2::F(value) => clvm.assert_concurrent_puzzle(value.0.puzzle_hash.clone()),
            Value2::G(value) => clvm.assert_seconds_relative(value.0.seconds),
            Value2::H(value) => clvm.assert_seconds_absolute(value.0.seconds),
            Value2::I(value) => clvm.assert_height_relative(value.0.height),
            Value2::J(value) => clvm.assert_height_absolute(value.0.height),
            Value2::K(value) => clvm.assert_before_seconds_relative(value.0.seconds),
            Value2::L(value) => clvm.assert_before_seconds_absolute(value.0.seconds),
            Value2::M(value) => clvm.assert_before_height_relative(value.0.height),
            Value2::N(value) => clvm.assert_before_height_absolute(value.0.height),
            Value2::O(value) => clvm.assert_my_coin_id(value.0.coin_id.clone()),
            Value2::P(value) => clvm.assert_my_parent_id(value.0.parent_id.clone()),
            Value2::Q(value) => clvm.assert_my_puzzle_hash(value.0.puzzle_hash.clone()),
            Value2::R(value) => clvm.assert_my_amount(value.0.amount),
            Value2::S(value) => clvm.assert_my_birth_seconds(value.0.seconds),
            Value2::T(value) => clvm.assert_my_birth_height(value.0.height),
            Value2::U(_value) => clvm.assert_ephemeral(),
            Value2::V(value) => {
                clvm.send_message(value.0.mode, value.0.message.clone(), value.0.data.clone())
            }
            Value2::W(value) => {
                clvm.receive_message(value.0.mode, value.0.message.clone(), value.0.data.clone())
            }
            Value2::X(value) => clvm.softfork(value.0.cost, value.0.rest.clone()),
            Value2::Y(value) => clvm.pair(value.0.first.clone(), value.0.rest.clone()),
            Value2::Z(value) => clvm.nft_metadata(value.0.clone()),
        },
    }
}
