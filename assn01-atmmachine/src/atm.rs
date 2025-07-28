
use crate::cash::Cash;
use crate::errors::ATMError;
use crate::utils;

pub struct ATM<'a> {
    pub id: u32,
    pub cash_store: &'a mut Cash,
}

impl<'a> ATM<'a> {
    pub fn new(id: u32, cash_store: &'a mut Cash) -> Self {
        println!("[Init] ATM ID {} allocated on STACK at: {:p}", id, &id);
        Self { id, cash_store }
    }

    pub fn perform_transaction(&mut self, withdrawal: u32) -> Result<(), ATMError> {
        println!(
            "[Action] ATM #{} attempting withdrawal of ${}",
            self.id, withdrawal
        );
        match self.cash_store.withdraw(withdrawal) {
            Ok(amount) => {
                println!(
                    "[Success] Withdrawal of ${} successful. {}",
                    amount, self.cash_store
                );
                Ok(())
            }
            Err(msg) => Err(ATMError::TransactionFailed(msg)),
        }
    }

    // ✅ NEW: Deposit Cash Method
    pub fn credit_cash(&mut self, deposit: u32) -> Result<(), ATMError> {
        println!(
            "[Action] ATM #{} attempting deposit of ${}",
            self.id, deposit
        );
        match self.cash_store.deposit(deposit) {
            Ok(_) => {
                println!(
                    "[Success] Deposit of ${} successful. {}",
                    deposit, self.cash_store
                );
                Ok(())
            }
            Err(msg) => Err(ATMError::TransactionFailed(msg)),
        }
    }

    pub fn show_balance(&self) {
        println!("{}", self.cash_store);
    }

    pub fn print_memory_segment(&self) {
        utils::print_stack_and_heap_info(self.id, &self.cash_store);
    }
}
