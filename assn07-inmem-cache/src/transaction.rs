use std::collections::HashMap;

use crate::account::Account;
use crate::error::TxnError;

pub enum Transaction {
    Deposit { account_id: u32, amount: f64 },
    Withdraw { account_id: u32, amount: f64 },
    Transfer { from_id: u32, to_id: u32, amount: f64 },
}

pub fn process_transaction(
    txn: Transaction,
    accounts: &mut HashMap<u32, Account>,
) -> Result<String, TxnError> {
    match txn {
        Transaction::Deposit { account_id, amount } => {
            let account = accounts
                .get_mut(&account_id)
                .ok_or(TxnError::AccountNotFound(account_id))?;
            account.deposit(amount)?;
            Ok(format!("Deposited INR{} to account {}", amount, account_id))
        }

        Transaction::Withdraw { account_id, amount } => {
            let account = accounts
                .get_mut(&account_id)
                .ok_or(TxnError::AccountNotFound(account_id))?;
            account.withdraw(amount)?;
            Ok(format!("Withdrew INR{} from account {}", amount, account_id))
        }

        Transaction::Transfer { from_id, to_id, amount } => {
            if from_id == to_id {
                return Err(TxnError::InvalidTransferSameAccount(from_id));
            }

            {
                let sender = accounts
                    .get_mut(&from_id)
                    .ok_or(TxnError::AccountNotFound(from_id))?;
                sender.withdraw(amount)?;
            }

            let receiver = accounts
                .get_mut(&to_id)
                .ok_or(TxnError::AccountNotFound(to_id))?;
            receiver.deposit(amount)?;

            Ok(format!(
                "Transferred INR{} from account {} to {}",
                amount, from_id, to_id
            ))
        }
    }
}
