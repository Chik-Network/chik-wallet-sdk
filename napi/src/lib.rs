#![allow(clippy::wildcard_imports)]
#![allow(clippy::too_many_arguments)]

use bindy::{FromRust, IntoRust, NapiParamContext, NapiReturnContext};
use napi::bindgen_prelude::*;
use napi_derive::napi;

bindy_macro::bindy_napi!("bindings.json");

#[napi]
impl Klvm {
    #[napi]
    pub fn alloc(&self, env: Env, value: Value) -> Result<Program> {
        Ok(Program::from_rust(
            alloc(env, &self.0, value)?,
            &NapiReturnContext(env),
        )?)
    }

    #[napi]
    pub fn bound_checked_number(&self, env: Env, value: f64) -> Result<Program> {
        Ok(Program::from_rust(
            self.0.f64(value)?,
            &NapiReturnContext(env),
        )?)
    }
}

#[napi]
impl Program {
    #[napi]
    pub fn to_bound_checked_number(&self) -> Result<Option<f64>> {
        Ok(self.0.to_small_int()?)
    }
}

pub type Value = Either26<
    f64,
    BigInt,
    bool,
    String,
    Uint8Array,
    Array,
    Null,
    ClassInstance<Program>,
    ClassInstance<PublicKey>,
    ClassInstance<Signature>,
    ClassInstance<K1PublicKey>,
    ClassInstance<K1Signature>,
    ClassInstance<R1PublicKey>,
    ClassInstance<R1Signature>,
    ClassInstance<Remark>,
    ClassInstance<AggSigParent>,
    ClassInstance<AggSigPuzzle>,
    ClassInstance<AggSigAmount>,
    ClassInstance<AggSigPuzzleAmount>,
    ClassInstance<AggSigParentAmount>,
    ClassInstance<AggSigParentPuzzle>,
    ClassInstance<AggSigUnsafe>,
    ClassInstance<AggSigMe>,
    ClassInstance<CreateCoin>,
    ClassInstance<ReserveFee>,
    Value2,
>;

type Value2 = Either26<
    ClassInstance<CreateCoinAnnouncement>,
    ClassInstance<CreatePuzzleAnnouncement>,
    ClassInstance<AssertCoinAnnouncement>,
    ClassInstance<AssertPuzzleAnnouncement>,
    ClassInstance<AssertConcurrentSpend>,
    ClassInstance<AssertConcurrentPuzzle>,
    ClassInstance<AssertSecondsRelative>,
    ClassInstance<AssertSecondsAbsolute>,
    ClassInstance<AssertHeightRelative>,
    ClassInstance<AssertHeightAbsolute>,
    ClassInstance<AssertBeforeSecondsRelative>,
    ClassInstance<AssertBeforeSecondsAbsolute>,
    ClassInstance<AssertBeforeHeightRelative>,
    ClassInstance<AssertBeforeHeightAbsolute>,
    ClassInstance<AssertMyCoinId>,
    ClassInstance<AssertMyParentId>,
    ClassInstance<AssertMyPuzzleHash>,
    ClassInstance<AssertMyAmount>,
    ClassInstance<AssertMyBirthSeconds>,
    ClassInstance<AssertMyBirthHeight>,
    ClassInstance<AssertEphemeral>,
    ClassInstance<SendMessage>,
    ClassInstance<ReceiveMessage>,
    ClassInstance<Softfork>,
    ClassInstance<Pair>,
    Value3,
>;

type Value3 = Either15<
    ClassInstance<NftMetadata>,
    ClassInstance<CurriedProgram>,
    ClassInstance<MipsMemo>,
    ClassInstance<InnerPuzzleMemo>,
    ClassInstance<RestrictionMemo>,
    ClassInstance<WrapperMemo>,
    ClassInstance<Force1of2RestrictedVariableMemo>,
    ClassInstance<MemoKind>,
    ClassInstance<MemberMemo>,
    ClassInstance<MofNMemo>,
    ClassInstance<MeltSingleton>,
    ClassInstance<TransferNft>,
    ClassInstance<RunCatTail>,
    ClassInstance<UpdateNftMetadata>,
    ClassInstance<UpdateDataStoreMerkleRoot>,
>;

fn alloc(
    env: Env,
    klvm: &chik_sdk_bindings::Klvm,
    value: Value,
) -> bindy::Result<chik_sdk_bindings::Program> {
    match value {
        Value::A(value) => klvm.f64(value),
        Value::B(value) => klvm.int(value.into_rust(&NapiParamContext)?),
        Value::C(value) => klvm.bool(value),
        Value::D(value) => klvm.string(value),
        Value::E(value) => klvm.atom(value.to_vec().into()),
        Value::F(value) => {
            let mut list = Vec::new();

            for index in 0..value.len() {
                let item = value.get::<Value>(index)?.unwrap();
                list.push(alloc(env, klvm, item)?);
            }

            Ok(klvm.list(list)?)
        }
        Value::G(_) => klvm.nil(),
        Value::H(value) => Ok(value.0.clone()),
        Value::I(value) => klvm.atom(value.to_bytes(env)?.to_vec().into()),
        Value::J(value) => klvm.atom(value.to_bytes(env)?.to_vec().into()),
        Value::K(value) => klvm.atom(value.to_bytes(env)?.to_vec().into()),
        Value::L(value) => klvm.atom(value.to_bytes(env)?.to_vec().into()),
        Value::M(value) => klvm.atom(value.to_bytes(env)?.to_vec().into()),
        Value::N(value) => klvm.atom(value.to_bytes(env)?.to_vec().into()),
        Value::O(value) => klvm.remark(value.0.rest.clone()),
        Value::P(value) => klvm.agg_sig_parent(value.0.public_key, value.0.message.clone()),
        Value::Q(value) => klvm.agg_sig_puzzle(value.0.public_key, value.0.message.clone()),
        Value::R(value) => klvm.agg_sig_amount(value.0.public_key, value.0.message.clone()),
        Value::S(value) => klvm.agg_sig_puzzle_amount(value.0.public_key, value.0.message.clone()),
        Value::T(value) => klvm.agg_sig_parent_amount(value.0.public_key, value.0.message.clone()),
        Value::U(value) => klvm.agg_sig_parent_puzzle(value.0.public_key, value.0.message.clone()),
        Value::V(value) => klvm.agg_sig_unsafe(value.0.public_key, value.0.message.clone()),
        Value::W(value) => klvm.agg_sig_me(value.0.public_key, value.0.message.clone()),
        Value::X(value) => {
            klvm.create_coin(value.0.puzzle_hash, value.0.amount, value.0.memos.clone())
        }
        Value::Y(value) => klvm.reserve_fee(value.0.amount),
        Value::Z(value) => match value {
            Value2::A(value) => klvm.create_coin_announcement(value.0.message.clone()),
            Value2::B(value) => klvm.create_puzzle_announcement(value.0.message.clone()),
            Value2::C(value) => klvm.assert_coin_announcement(value.0.announcement_id),
            Value2::D(value) => klvm.assert_puzzle_announcement(value.0.announcement_id),
            Value2::E(value) => klvm.assert_concurrent_spend(value.0.coin_id),
            Value2::F(value) => klvm.assert_concurrent_puzzle(value.0.puzzle_hash),
            Value2::G(value) => klvm.assert_seconds_relative(value.0.seconds),
            Value2::H(value) => klvm.assert_seconds_absolute(value.0.seconds),
            Value2::I(value) => klvm.assert_height_relative(value.0.height),
            Value2::J(value) => klvm.assert_height_absolute(value.0.height),
            Value2::K(value) => klvm.assert_before_seconds_relative(value.0.seconds),
            Value2::L(value) => klvm.assert_before_seconds_absolute(value.0.seconds),
            Value2::M(value) => klvm.assert_before_height_relative(value.0.height),
            Value2::N(value) => klvm.assert_before_height_absolute(value.0.height),
            Value2::O(value) => klvm.assert_my_coin_id(value.0.coin_id),
            Value2::P(value) => klvm.assert_my_parent_id(value.0.parent_id),
            Value2::Q(value) => klvm.assert_my_puzzle_hash(value.0.puzzle_hash),
            Value2::R(value) => klvm.assert_my_amount(value.0.amount),
            Value2::S(value) => klvm.assert_my_birth_seconds(value.0.seconds),
            Value2::T(value) => klvm.assert_my_birth_height(value.0.height),
            Value2::U(_value) => klvm.assert_ephemeral(),
            Value2::V(value) => {
                klvm.send_message(value.0.mode, value.0.message.clone(), value.0.data.clone())
            }
            Value2::W(value) => {
                klvm.receive_message(value.0.mode, value.0.message.clone(), value.0.data.clone())
            }
            Value2::X(value) => klvm.softfork(value.0.cost, value.0.rest.clone()),
            Value2::Y(value) => klvm.pair(value.0.first.clone(), value.0.rest.clone()),
            Value2::Z(value) => match value {
                Value3::A(value) => klvm.nft_metadata(value.0.clone()),
                Value3::B(value) => value.0.program.curry(value.0.args.clone()),
                Value3::C(value) => klvm.mips_memo(value.0.clone()),
                Value3::D(value) => klvm.inner_puzzle_memo(value.0.clone()),
                Value3::E(value) => klvm.restriction_memo(value.0.clone()),
                Value3::F(value) => klvm.wrapper_memo(value.0.clone()),
                Value3::G(value) => klvm.force_1_of_2_restricted_variable_memo(value.0.clone()),
                Value3::H(value) => klvm.memo_kind(value.0.clone()),
                Value3::I(value) => klvm.member_memo(value.0.clone()),
                Value3::J(value) => klvm.m_of_n_memo(value.0.clone()),
                Value3::K(_value) => klvm.melt_singleton(),
                Value3::L(value) => klvm.transfer_nft(
                    value.0.launcher_id,
                    value.0.trade_prices.clone(),
                    value.0.singleton_inner_puzzle_hash,
                ),
                Value3::M(value) => {
                    klvm.run_cat_tail(value.0.program.clone(), value.0.solution.clone())
                }
                Value3::N(value) => klvm.update_nft_metadata(
                    value.0.updater_puzzle_reveal.clone(),
                    value.0.updater_solution.clone(),
                ),
                Value3::O(value) => klvm
                    .update_data_store_merkle_root(value.0.new_merkle_root, value.0.memos.clone()),
            },
        },
    }
}
