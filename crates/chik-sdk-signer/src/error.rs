use klvm_traits::{FromKlvmError, ToKlvmError};
use klvmr::reduction::EvalErr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SignerError {
    #[error("Eval error: {0}")]
    Eval(#[from] EvalErr),

    #[error("To KLVM error: {0}")]
    ToKlvm(#[from] ToKlvmError),

    #[error("From KLVM error: {0}")]
    FromKlvm(#[from] FromKlvmError),

    #[error("Infinity public key")]
    InfinityPublicKey,

    #[error("Invalid secp key")]
    InvalidSecpKey(#[from] k256::ecdsa::Error),
}
