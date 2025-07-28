
use crate::models::UserAccount;
use crate::errors::WalletError;

pub fn authenticate(user: &UserAccount, input_password: &str) -> Result<(), WalletError> {
    if user.password == input_password {
        Ok(())
    } else {
        Err(WalletError::IncorrectPassword)
    }
}
