
use crate::models::{Transaction, UserAccount};
use crate::errors::WalletError;

pub fn check_balance(user: &UserAccount) -> f64 {
    user.balance
}

pub fn transfer_funds(
    sender: &mut UserAccount,
    receiver: &mut UserAccount,
    amount: f64,
) -> Result<(), WalletError> {
    if sender.balance < amount {
        return Err(WalletError::InsufficientFunds);
    }

    sender.balance -= amount;
    receiver.balance += amount;

    let txn = Transaction {
        to: receiver.username.clone(),
        amount,
    };
    sender.history.push(txn);
    Ok(())
}
