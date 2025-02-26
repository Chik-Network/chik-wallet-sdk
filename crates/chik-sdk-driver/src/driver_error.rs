use std::num::TryFromIntError;

use klvm_traits::{FromKlvmError, ToKlvmError};
use klvmr::reduction::EvalErr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DriverError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("try from int error")]
    TryFromInt(#[from] TryFromIntError),

    #[error("failed to serialize klvm value: {0}")]
    ToKlvm(#[from] ToKlvmError),

    #[error("failed to deserialize klvm value: {0}")]
    FromKlvm(#[from] FromKlvmError),

    #[error("klvm eval error: {0}")]
    Eval(#[from] EvalErr),

    #[error("invalid mod hash")]
    InvalidModHash,

    #[error("non-standard inner puzzle layer")]
    NonStandardLayer,

    #[error("missing child")]
    MissingChild,

    #[error("missing hint")]
    MissingHint,

    #[error("missing memo")]
    MissingMemo,

    #[error("invalid memo")]
    InvalidMemo,

    #[error("invalid singleton struct")]
    InvalidSingletonStruct,

    #[error("expected even oracle fee, but it was odd")]
    OddOracleFee,

    #[error("custom driver error: {0}")]
    Custom(String),

    #[error("invalid merkle proof")]
    InvalidMerkleProof,

    #[error("unknown puzzle")]
    UnknownPuzzle,

    #[error("wrong number of spends")]
    WrongSpendCount,

    #[error("missing member spend")]
    MissingMemberSpend,
}
