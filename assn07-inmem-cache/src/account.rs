#[derive(Debug)]
pub struct Account {
    pub id: u32,
    pub balance: f64,
}

impl Account {
    pub fn new(id: u32, balance: f64) -> Self {
        Self { id, balance }
    }

    pub fn deposit(&mut self, amount: f64) -> Result<(), String> {
        self.balance += amount;
        Ok(())
    }

    pub fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if self.balance >= amount {
            self.balance -= amount;
            Ok(())
        } else {
            Err("Insufficient funds.".into())
        }
    }
}
