use chia_protocol::Bytes32;
use chia_wallet::{
    did::{DidArgs, DidSolution},
    LineageProof, Proof,
};
use clvm_traits::FromClvm;
use clvm_utils::tree_hash;
use clvmr::{reduction::Reduction, run_program, Allocator, ChiaDialect, NodePtr};

use crate::{
    did_inner_puzzle_hash, singleton_puzzle_hash, CreateCoinWithMemos, DidInfo, ParseContext,
    ParseError, ParseSingleton,
};

pub fn parse_did(
    allocator: &mut Allocator,
    ctx: &ParseContext,
    singleton: &ParseSingleton,
    max_cost: u64,
) -> Result<Option<DidInfo<NodePtr>>, ParseError> {
    let args = DidArgs::<NodePtr, NodePtr>::from_clvm(allocator, singleton.inner_args())?;

    let DidSolution::InnerSpend(p2_solution) =
        DidSolution::<NodePtr>::from_clvm(allocator, singleton.inner_solution())?;

    if args.singleton_struct != singleton.args().singleton_struct {
        return Err(ParseError::DidSingletonStructMismatch);
    }

    let Reduction(_cost, output) = run_program(
        allocator,
        &ChiaDialect::new(0),
        args.inner_puzzle,
        p2_solution,
        max_cost,
    )?;

    let conditions = Vec::<NodePtr>::from_clvm(allocator, output)?;
    let mut p2_puzzle_hash = None;

    for condition in conditions {
        let Ok(create_coin) = CreateCoinWithMemos::from_clvm(allocator, condition) else {
            continue;
        };

        if create_coin.amount % 2 == 0 {
            continue;
        }

        p2_puzzle_hash = create_coin
            .memos
            .first()
            .and_then(|memo| Some(Bytes32::new(memo.as_ref().try_into().ok()?)));
        break;
    }

    let Some(p2_puzzle_hash) = p2_puzzle_hash else {
        return Err(ParseError::MissingCreateCoin);
    };

    let did_inner_puzzle_hash = did_inner_puzzle_hash(
        p2_puzzle_hash,
        args.recovery_did_list_hash,
        args.num_verifications_required,
        args.singleton_struct.launcher_id,
        tree_hash(allocator, args.metadata).into(),
    );

    let singleton_puzzle_hash =
        singleton_puzzle_hash(args.singleton_struct.launcher_id, did_inner_puzzle_hash);

    if singleton_puzzle_hash != ctx.coin().puzzle_hash {
        return Err(ParseError::UnknownDidOutput);
    }

    Ok(Some(DidInfo {
        launcher_id: args.singleton_struct.launcher_id,
        coin: ctx.coin().clone(),
        p2_puzzle_hash,
        did_inner_puzzle_hash,
        recovery_did_list_hash: args.recovery_did_list_hash,
        num_verifications_required: args.num_verifications_required,
        metadata: args.metadata,
        proof: Proof::Lineage(LineageProof {
            parent_coin_info: ctx.parent_coin().parent_coin_info,
            inner_puzzle_hash: tree_hash(allocator, singleton.args().inner_puzzle).into(),
            amount: ctx.parent_coin().amount,
        }),
    }))
}

#[cfg(test)]
mod tests {
    use chia_bls::PublicKey;
    use chia_protocol::{Bytes32, Coin};
    use chia_wallet::standard::standard_puzzle_hash;
    use clvm_traits::ToNodePtr;
    use clvmr::Allocator;

    use crate::{
        parse_did, parse_puzzle, parse_singleton, Chainable, CreateDid, Launcher, SpendContext,
        StandardSpend,
    };

    #[test]
    fn test_parse_did() -> anyhow::Result<()> {
        let mut allocator = Allocator::new();
        let mut ctx = SpendContext::new(&mut allocator);

        let pk = PublicKey::default();
        let puzzle_hash = standard_puzzle_hash(&pk).into();
        let parent = Coin::new(Bytes32::default(), puzzle_hash, 1);

        let (create_did, did_info) = Launcher::new(parent.coin_id(), 1)
            .create(&mut ctx)?
            .create_standard_did(&mut ctx, pk.clone())?;

        StandardSpend::new()
            .chain(create_did)
            .finish(&mut ctx, parent, pk)?;

        let coin_spends = ctx.take_spends();

        let coin_spend = coin_spends
            .into_iter()
            .find(|cs| cs.coin.coin_id() == did_info.coin.parent_coin_info)
            .unwrap();

        let puzzle = coin_spend.puzzle_reveal.to_node_ptr(&mut allocator)?;
        let solution = coin_spend.solution.to_node_ptr(&mut allocator)?;

        let parse_ctx = parse_puzzle(
            &mut allocator,
            puzzle,
            solution,
            coin_spend.coin,
            did_info.coin.clone(),
        )?;

        let parse = parse_singleton(&mut allocator, &parse_ctx)?.unwrap();
        let parse = parse_did(&mut allocator, &parse_ctx, &parse, u64::MAX)?;
        assert_eq!(
            parse.map(|did_info| did_info.with_metadata(())),
            Some(did_info)
        );

        Ok(())
    }
}
