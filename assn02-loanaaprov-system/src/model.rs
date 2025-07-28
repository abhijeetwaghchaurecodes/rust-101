
#[derive(Debug)]
pub struct Applicant {
    pub name: String,
    pub income: u32,
    pub age: u8,
    pub loan_amount: u32,
    pub co_applicant: Option<String>,
}

impl Applicant {
    pub fn is_eligible(&self) -> Result<(), String> {
        if self.age < 21 {
            return Err(format!("AgeError: {} is below minimum age 21", self.age));
        }
        if self.income < 25000 {
            return Err(format!(
                "IncomeError: INR{} is below minimum threshold of INR25,000",
                self.income
            ));
        }
        if self.loan_amount > (self.income * 10) {
            return Err(format!(
                "LoanError: Loan INR{} too high for income INR{}",
                self.loan_amount, self.income
            ));
        }
        Ok(())
    }

    pub fn print_summary(&self) {
        println!(
            "Applicant: {}\nIncome: INR{}\nAge: {}\nRequested Loan: INR{}\nCo-applicant: {}\n",
            self.name,
            self.income,
            self.age,
            self.loan_amount,
            self.co_applicant
                .as_ref()
                .map_or("None", |c| c.as_str())
        );
    }
}
