mod model;
mod errors;
mod validation;
mod workflow;

use model::Applicant;
use validation::{read_input, read_optional};
use workflow::evaluate;

fn main() {
    println!("Loan Approval System");

    loop {
        println!("\n--- New Applicant ---");

        let name: String = read_input("Enter name: ");
        let age: u8 = read_input("Enter age (must be 21+): ");
        let income: u32 = read_input("Enter monthly income (in INR): ");
        let loan_amount: u32 = read_input("Enter loan amount requested: ");
        let co_applicant = read_optional("Enter co-applicant name (optional): ");

        let applicant = Applicant {
            name,
            age,
            income,
            loan_amount,
            co_applicant,
        };

        applicant.print_summary();
        evaluate(&applicant);

        let again: String = read_input("\nDo you want to apply again? (y/n): ");
        if again.to_lowercase() != "y" {
            println!("Thank you for using the system!");
            break;
        }
    }
}
