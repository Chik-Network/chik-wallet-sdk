use binky::Result;
use chik_puzzles::NFT_METADATA_UPDATER_DEFAULT_HASH;
use klvm_utils::TreeHash;

#[derive(Clone)]
pub struct Constants;

impl Constants {
    pub fn default_metadata_updater_hash() -> Result<TreeHash> {
        Ok(NFT_METADATA_UPDATER_DEFAULT_HASH.into())
    }
}
