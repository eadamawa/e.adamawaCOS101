use std::io;

fn main() {
    // Just making the output look clean
    println!("==================================================");
    println!(" EY NIGERIA RECRUITMENT EXPERIENCE CHECKER ");
    println!("==================================================");

    // This vector will store each candidate as (name, years_of_experience)
    let mut candidates: Vec<(String, u32)> = Vec::new();

    // Ask how many people were interviewed
    println!("\nEnter the number of candidates interviewed: ");
    let mut count_input = String::new();
    io::stdin().read_line(&mut count_input).expect("Failed to read input");

    let candidate_count: u32 = count_input.trim().parse().expect("Invalid number! Please enter a valid integer.");

    // Loop through each candidate and collect their details
    for i in 0..candidate_count {
        println!("\n--- Candidate {} ---", i + 1);

        // Get candidate name
        println!("Enter candidate name: ");
        let mut name_input = String::new();
        io::stdin().read_line(&mut name_input).expect("Failed to read candidate name");
        let name = name_input.trim().to_string();

        // Get candidate experience
        println!("Enter years of programming experience: ");
        let mut exp_input = String::new();
        io::stdin().read_line(&mut exp_input).expect("Failed to read experience");

        let years: u32 = exp_input.trim().parse().expect("Invalid input! Please enter a valid numeric value.");

        // Store the candidate info inside the vector
        candidates.push((name, years));
    }

    // If nobody was entered, just stop the program
    if candidates.is_empty() {
        println!("\nNo candidate data was provided.");
        return;
    }

    // Start by assuming the first candidate is the top one
    let mut top_candidate_name = &candidates[0].0;
    let mut max_years = candidates[0].1;

    // Check every candidate to find who has the highest experience
    for candidate in candidates.iter() {
        if candidate.1 > max_years {
            max_years = candidate.1;
            top_candidate_name = &candidate.0;
        }
    }

    // Final output
    println!("\n==================================================");
    println!(" SELECTION RESULT ");
    println!("==================================================");
    println!(
        "The candidate with the highest programming experience is: {}",
        top_candidate_name
    );
    println!("Years of Experience: {} years", max_years);
}
