use std::fmt;

#[derive(Debug)]
pub enum TxnError {
    AccountNotFound(u32),
    InsufficientFunds,
    InvalidTransferSameAccount(u32),
    GenericError(String),
}

impl fmt::Display for TxnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TxnError::AccountNotFound(id) => write!(f, "Account not found: {}", id),
            TxnError::InsufficientFunds => write!(f, "Insufficient funds"),
            TxnError::InvalidTransferSameAccount(id) => {
                write!(f, "Transfer to the same account is invalid: {}", id)
            }
            TxnError::GenericError(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for TxnError {}

impl From<String> for TxnError {
    fn from(value: String) -> Self {
        TxnError::GenericError(value)
    }
}
