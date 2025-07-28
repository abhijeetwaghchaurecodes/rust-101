
use std::fmt;

pub struct Cash {
    amount: u32,
}

impl Cash {
    pub fn new(amount: u32) -> Self {
        println!("[Init] Cash allocated on HEAP at: {:p}", &amount);
        Self { amount }
    }

    pub fn get_balance(&self) -> u32 {
        self.amount
    }

    pub fn withdraw(&mut self, value: u32) -> Result<u32, String> {
        if value > self.amount {
            Err(format!(
                "Insufficient funds: requested {}, available {}",
                value, self.amount
            ))
        } else {
            self.amount -= value;
            Ok(value)
        }
    }

    // ✅ NEW: Deposit functionality
    pub fn deposit(&mut self, value: u32) -> Result<(), String> {
        self.amount = self.amount.checked_add(value).ok_or("Overflow during deposit")?;
        Ok(())
    }
}

impl Drop for Cash {
    fn drop(&mut self) {
        println!("[RAII] Cash released: ${}, memory auto-freed.", self.amount);
    }
}

impl fmt::Display for Cash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Current Cash in ATM: ${}", self.amount)
    }
}
