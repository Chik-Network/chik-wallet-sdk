use chik_puzzles::{
    cat::{
        CatArgs, EverythingWithSignatureTailArgs, GenesisByCoinIdTailArgs, CAT_PUZZLE,
        CAT_PUZZLE_HASH, EVERYTHING_WITH_SIGNATURE_TAIL_PUZZLE,
        EVERYTHING_WITH_SIGNATURE_TAIL_PUZZLE_HASH, GENESIS_BY_COIN_ID_TAIL_PUZZLE,
        GENESIS_BY_COIN_ID_TAIL_PUZZLE_HASH,
    },
    did::{DidArgs, DID_INNER_PUZZLE, DID_INNER_PUZZLE_HASH},
    nft::{
        NftIntermediateLauncherArgs, NftOwnershipLayerArgs, NftRoyaltyTransferPuzzleArgs,
        NftStateLayerArgs, NFT_INTERMEDIATE_LAUNCHER_PUZZLE, NFT_INTERMEDIATE_LAUNCHER_PUZZLE_HASH,
        NFT_OWNERSHIP_LAYER_PUZZLE, NFT_OWNERSHIP_LAYER_PUZZLE_HASH, NFT_ROYALTY_TRANSFER_PUZZLE,
        NFT_ROYALTY_TRANSFER_PUZZLE_HASH, NFT_STATE_LAYER_PUZZLE, NFT_STATE_LAYER_PUZZLE_HASH,
    },
    singleton::{SingletonArgs, SINGLETON_TOP_LAYER_PUZZLE, SINGLETON_TOP_LAYER_PUZZLE_HASH},
    standard::{StandardArgs, STANDARD_PUZZLE, STANDARD_PUZZLE_HASH},
};
use klvm_traits::ToKlvm;
use klvm_utils::{CurriedProgram, TreeHash, TreeHasher};

pub trait Mod {
    const MOD_REVEAL: &[u8];
    const MOD_HASH: TreeHash;

    fn curry_tree_hash(&self) -> TreeHash
    where
        Self: Sized + ToKlvm<TreeHasher>,
    {
        CurriedProgram {
            program: Self::MOD_HASH,
            args: self,
        }
        .to_klvm(&mut TreeHasher)
        .unwrap()
    }
}

impl<T> Mod for &T
where
    T: Mod,
{
    const MOD_REVEAL: &'static [u8] = T::MOD_REVEAL;
    const MOD_HASH: TreeHash = T::MOD_HASH;
}

impl Mod for StandardArgs {
    const MOD_REVEAL: &[u8] = &STANDARD_PUZZLE;
    const MOD_HASH: TreeHash = STANDARD_PUZZLE_HASH;
}

impl<I> Mod for CatArgs<I> {
    const MOD_REVEAL: &[u8] = &CAT_PUZZLE;
    const MOD_HASH: TreeHash = CAT_PUZZLE_HASH;
}

impl<I, M> Mod for DidArgs<I, M> {
    const MOD_REVEAL: &[u8] = &DID_INNER_PUZZLE;
    const MOD_HASH: TreeHash = DID_INNER_PUZZLE_HASH;
}

impl Mod for NftIntermediateLauncherArgs {
    const MOD_REVEAL: &[u8] = &NFT_INTERMEDIATE_LAUNCHER_PUZZLE;
    const MOD_HASH: TreeHash = NFT_INTERMEDIATE_LAUNCHER_PUZZLE_HASH;
}

impl Mod for NftRoyaltyTransferPuzzleArgs {
    const MOD_REVEAL: &[u8] = &NFT_ROYALTY_TRANSFER_PUZZLE;
    const MOD_HASH: TreeHash = NFT_ROYALTY_TRANSFER_PUZZLE_HASH;
}

impl<I, P> Mod for NftOwnershipLayerArgs<I, P> {
    const MOD_REVEAL: &[u8] = &NFT_OWNERSHIP_LAYER_PUZZLE;
    const MOD_HASH: TreeHash = NFT_OWNERSHIP_LAYER_PUZZLE_HASH;
}

impl<I, M> Mod for NftStateLayerArgs<I, M> {
    const MOD_REVEAL: &[u8] = &NFT_STATE_LAYER_PUZZLE;
    const MOD_HASH: TreeHash = NFT_STATE_LAYER_PUZZLE_HASH;
}

impl<I> Mod for SingletonArgs<I> {
    const MOD_REVEAL: &[u8] = &SINGLETON_TOP_LAYER_PUZZLE;
    const MOD_HASH: TreeHash = SINGLETON_TOP_LAYER_PUZZLE_HASH;
}

impl Mod for EverythingWithSignatureTailArgs {
    const MOD_REVEAL: &[u8] = &EVERYTHING_WITH_SIGNATURE_TAIL_PUZZLE;
    const MOD_HASH: TreeHash = EVERYTHING_WITH_SIGNATURE_TAIL_PUZZLE_HASH;
}

impl Mod for GenesisByCoinIdTailArgs {
    const MOD_REVEAL: &[u8] = &GENESIS_BY_COIN_ID_TAIL_PUZZLE;
    const MOD_HASH: TreeHash = GENESIS_BY_COIN_ID_TAIL_PUZZLE_HASH;
}
