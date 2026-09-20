use std::io;

fn main() {
    // Read experience
    let mut exp_input = String::new();
    println!("Is the employee experienced? (yes/no):");
    io::stdin().read_line(&mut exp_input).expect("Failed to read input");
    let experienced = exp_input.trim();

    // Read age
    let mut age_input = String::new();
    println!("Enter employee age:");
    io::stdin().read_line(&mut age_input).expect("Failed to read input");
    let age: i32 = age_input.trim().parse().expect("Please enter a valid number");

    // Decision making
    if experienced == "yes" {
        if age >= 40 {
            println!("Annual incentive: ₦1,560,000");
        } else if age >= 30 && age <= 39 {
            println!("Annual incentive: ₦1,480,000");
        } else if age < 28 {
            println!("Annual incentive: ₦1,300,000");
        } else {
            // Covers age 28–29 (not listed but safe)
            println!("Annual incentive: ₦1,300,000");
        }
    } else {
        println!("Annual incentive: ₦100,000");
    }
}
