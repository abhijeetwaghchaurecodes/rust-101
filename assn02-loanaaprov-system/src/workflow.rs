
use crate::model::Applicant;

pub fn evaluate(applicant: &Applicant) {
    match applicant.is_eligible() {
        Ok(_) => {
            println!("✅ Loan approved for {}!", applicant.name);
        }
        Err(e) => {
            println!("❌ Loan rejected: {}", e);
        }
    }
}
