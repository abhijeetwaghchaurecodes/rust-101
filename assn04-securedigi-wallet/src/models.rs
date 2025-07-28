
#[derive(Debug, Clone)]
pub struct Transaction {
    pub to: String,
    pub amount: f64,
}

#[derive(Debug)]
pub struct UserAccount {
    pub username: String,
    pub password: String, 
    pub balance: f64,
    pub history: Vec<Transaction>,
}

impl UserAccount {
    pub fn new(username: String, password: String, balance: f64) -> Self {
        Self {
            username,
            password,
            balance,
            history: Vec::new(),
        }
    }

    pub fn get_history(&self) -> &[Transaction] {
        &self.history
    }
}
