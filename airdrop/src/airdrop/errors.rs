use thiserror::Error;

#[derive(Error, Debug, serde::Serialize, serde::Deserialize, Clone)]
pub enum AirdropError {
    #[error("Invalid Address format")]
    InvalidAddress,
    #[error("Invalid Amount")]
    InvalidAmount,
    #[error("Insufficient Funds Needs: {requires}, have {available}")]
    InsufficientFunds { requires: u64, available: u64 },
    #[error("Transaction Failed")]
    TransactionFailed,
    #[error("Batch Size Limit Exceeded")]
    BatchSizeLimitExceeded,
    #[error("Duplicate Address")]
    DuplicateAddress,
    #[error("Network Timeout")]
    NetworkTimeout,
    #[error("Invalid CSV format")]
    InvalidCSVFormat,
    #[error("IO Error")]
    IOError(String),
    #[error("Invalid CSV Header")]
    InvalidCSVHeader,
    #[error("Zero Amount")]
    ZeroAmount,
    #[error("Empty Batch")]
    EmptyBatch,
}
