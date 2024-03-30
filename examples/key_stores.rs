use std::str::FromStr;

use bip39::Mnemonic;
use chia_bls::{
    derive_keys::{
        master_to_wallet_hardened_intermediate, master_to_wallet_unhardened_intermediate,
    },
    DerivableKey, PublicKey, SecretKey,
};
use chia_wallet::{standard::DEFAULT_HIDDEN_PUZZLE_HASH, DeriveSynthetic};
use chia_wallet_sdk::sqlite::SqliteKeyStore;
use sqlx::SqlitePool;

// This is for simulator testing purposes only. Do not use this mnemonic on mainnet.
const MNEMONIC: &str = "
    setup update spoil lazy square course ring tell
    hard eager industry ticket guess amused build reunion
    woman system cause afraid first material machine morning
";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = SqlitePool::connect(":memory:").await?;

    let seed = Mnemonic::from_str(MNEMONIC)?.to_seed("");
    let root_sk = SecretKey::from_seed(&seed);

    let unhardened_key_store = SqliteKeyStore::new_with_migrations(pool.clone(), false).await?;
    let hardened_key_store = SqliteKeyStore::new_with_migrations(pool, true).await?;

    let int_pk = master_to_wallet_unhardened_intermediate(&root_sk.public_key());
    let int_sk = master_to_wallet_hardened_intermediate(&root_sk);

    let unhardened_pks: Vec<PublicKey> = (0..100)
        .map(|index| {
            int_pk
                .derive_unhardened(index)
                .derive_synthetic(&DEFAULT_HIDDEN_PUZZLE_HASH)
        })
        .collect();
    unhardened_key_store
        .extend_keys(0, unhardened_pks.as_slice())
        .await?;

    let hardened_pks: Vec<PublicKey> = (0..100)
        .map(|index| {
            int_sk
                .derive_hardened(index)
                .public_key()
                .derive_synthetic(&DEFAULT_HIDDEN_PUZZLE_HASH)
        })
        .collect();
    hardened_key_store
        .extend_keys(0, hardened_pks.as_slice())
        .await?;

    println!(
        "First unhardened puzzle hash: {}",
        unhardened_key_store
            .puzzle_hash(0)
            .await?
            .expect("missing puzzle hash")
    );

    Ok(())
}
