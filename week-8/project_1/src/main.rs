use std::io;

fn main() {
    // APS levels and job titles for each career path (basically the whole table)
    let aps_levels = vec![
        "APS 1-2",
        "APS 3-5",
        "APS 5-8",
        "EL1 8-10",
        "EL2 10-13",
        "SES",
    ];

    // Office Admin job titles
    let office_admin = vec![
        "Intern",
        "Administrator",
        "Senior Administrator",
        "Office Manager",
        "Director",
        "CEO",
    ];

    // Academic job titles
    let academic = vec![
        "-",
        "Research Assistant",
        "PhD Candidate",
        "Post-Doc Researcher",
        "Senior Lecturer",
        "Dean",
    ];

    // Lawyer job titles
    let lawyer = vec![
        "Paralegal",
        "Junior Associate",
        "Associate",
        "Senior Associate 1-2",
        "Senior Associate 3-4",
        "Partner",
    ];

    // Teacher job titles
    let teacher = vec![
        "Placement",
        "Classroom Teacher",
        "Senior Teacher",
        "Leading Teacher",
        "Deputy Principal",
        "Principal",
    ];

    println!("==================================================");
    println!(" PUBLIC SERVICE APS LEVEL CHECKER (NIGERIA) ");
    println!("==================================================");

    // Ask the user which sector they belong to
    println!("\nSelect Public Service Sector:");
    println!("1. Office Administrator");
    println!("2. Academic");
    println!("3. Lawyer");
    println!("4. Teacher");
    println!("Enter choice (1-4): ");

    let mut category_input = String::new();
    io::stdin()
        .read_line(&mut category_input)
        .expect("Failed to read input");

    let category: i32 = category_input
        .trim()
        .parse()
        .expect("Invalid input! Please enter a number.");

    // Ask for years of experience
    println!("\nEnter staff member's years of experience: ");
    let mut exp_input = String::new();
    io::stdin()
        .read_line(&mut exp_input)
        .expect("Failed to read input");

    let years: f32 = exp_input
        .trim().parse().expect("Invalid input! Please enter a numeric value.");

    // Converting years of experience into the correct APS row index
    let index: usize = if years >= 1.0 && years <= 2.0 {
        0
    } else if years > 2.0 && years <= 5.0 {
        1
    } else if years > 5.0 && years <= 8.0 {
        2
    } else if years > 8.0 && years <= 10.0 {
        3
    } else if years > 10.0 && years <= 13.0 {
        4
    } else if years > 13.0 {
        5
    } else {
        println!("\nInvalid experience entry (must be at least 1 year).");
        return;
    };

    // Grab the APS level using the index
    let level = aps_levels[index];

    // matching the sector and print the correct job title + APS level
    match category {
        1 => {
            println!("\n--- Staff Level Validation Output ---");
            println!("Category: Office Administrator");
            println!("Position Title: {}", office_admin[index]);
            println!("APS Position Level: {}", level);
        }
        2 => {
            if academic[index] == "-" {
                println!("\nNo Academic position allocated for APS 1-2 level.");
            } else {
                println!("\n--- Staff Level Validation Output ---");
                println!("Category: Academic");
                println!("Position Title: {}", academic[index]);
                println!("APS Position Level: {}", level);
            }
        }
        3 => {
            println!("\n--- Staff Level Validation Output ---");
            println!("Category: Lawyer");
            println!("Position Title: {}", lawyer[index]);
            println!("APS Position Level: {}", level);
        }
        4 => {
            println!("\n--- Staff Level Validation Output ---");
            println!("Category: Teacher");
            println!("Position Title: {}", teacher[index]);
            println!("APS Position Level: {}", level);
        }
        _ => {
            println!("\nInvalid sector selected! Choice must be between 1 and 4.");
        }
    }
}
