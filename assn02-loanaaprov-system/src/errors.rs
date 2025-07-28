
#[derive(Debug)]
pub enum LoanError {
    AgeError(u8),
    IncomeError(u32),
    LoanAmountError(u32),
}

impl std::fmt::Display for LoanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoanError::AgeError(age) => write!(f, "AgeError: age {} is too low", age),
            LoanError::IncomeError(income) => {
                write!(f, "IncomeError: income {} is insufficient", income)
            }
            LoanError::LoanAmountError(amount) => {
                write!(f, "LoanError: loan amount {} exceeds limit", amount)
            }
        }
    }
}

impl std::error::Error for LoanError {}
