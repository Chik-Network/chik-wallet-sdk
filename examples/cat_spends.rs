use chik_protocol::{Bytes32, Coin};
use chik_puzzles::standard::StandardArgs;
use chik_sdk_driver::{Cat, CatSpend, SpendContext, SpendWithConditions, StandardLayer};
use chik_sdk_test::test_secret_key;
use chik_sdk_types::Conditions;

fn main() -> anyhow::Result<()> {
    let ctx = &mut SpendContext::new();

    let sk = test_secret_key()?;
    let pk = sk.public_key();
    let p2 = StandardLayer::new(pk);

    let p2_puzzle_hash = StandardArgs::curry_tree_hash(pk).into();
    let coin = Coin::new(Bytes32::default(), p2_puzzle_hash, 1_000);

    let memos = ctx.hint(p2_puzzle_hash)?;

    // Issue the CAT using the single issuance (genesis by coin id) TAIL.
    let conditions = Conditions::new().create_coin(p2_puzzle_hash, coin.amount, Some(memos));
    let (issue_cat, cat) = Cat::single_issuance_eve(ctx, coin.coin_id(), coin.amount, conditions)?;
    p2.spend(ctx, coin, issue_cat)?;
    println!("Issued test CAT with asset id {}", cat.asset_id);

    // Spend the CAT coin.
    let new_cat = cat.wrapped_child(p2_puzzle_hash, 1000);
    let cat_spends = [CatSpend::new(
        new_cat,
        p2.spend_with_conditions(
            ctx,
            Conditions::new().create_coin(p2_puzzle_hash, coin.amount, Some(memos)),
        )?,
    )];

    Cat::spend_all(ctx, &cat_spends)?;

    let new_coin = new_cat.wrapped_child(p2_puzzle_hash, 1000).coin;

    println!(
        "Spent the CAT coin to create new coin with id {}",
        new_coin.coin_id()
    );

    Ok(())
}
