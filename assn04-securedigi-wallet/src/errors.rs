
#[derive(Debug)]
pub enum WalletError {
    IncorrectPassword,
    InsufficientFunds,
    UserNotFound,
    InvalidInput,
}

impl std::fmt::Display for WalletError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WalletError::IncorrectPassword => write!(f, "incorrect password."),
            WalletError::InsufficientFunds => write!(f, "insufficient funds."),
            WalletError::UserNotFound => write!(f, "user not found."),
            WalletError::InvalidInput => write!(f, "invalid input."),
        }
    }
}

impl std::error::Error for WalletError {}
