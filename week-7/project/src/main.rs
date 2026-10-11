use std::io;

// 1. Trapezium Area Function: height / 2 * (base1 + base2)
fn calculate_trapezium() {
    let mut input1 = String::new();
    println!("Enter height of the trapezium:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let height: f64 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter base1 of the trapezium:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let base1: f64 = input2.trim().parse().expect("Invalid input");

    let mut input3 = String::new();
    println!("Enter base2 of the trapezium:");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let base2: f64 = input3.trim().parse().expect("Invalid input");

    let area = (height / 2.0) * (base1 + base2);
    println!("The area of the trapezium is: {}", area);
}

// 2. Rhombus Area Function: 1/2 * diagonal1 * diagonal2
fn calculate_rhombus() {
    let mut input1 = String::new();
    println!("Enter diagonal 1 of the rhombus:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let diagonal1: f64 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter diagonal 2 of the rhombus:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let diagonal2: f64 = input2.trim().parse().expect("Invalid input");

    let area = 0.5 * diagonal1 * diagonal2;
    println!("The area of the rhombus is: {}", area);
}

// 3. Parallelogram Area Function: base * altitude[cite: 1]
fn calculate_parallelogram() {
    let mut input1 = String::new();
    println!("Enter base of the parallelogram:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let base: f64 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter altitude of the parallelogram:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let altitude: f64 = input2.trim().parse().expect("Invalid input");

    let area = base * altitude;
    println!("The area of the parallelogram is: {}", area);
}

// 4. Cube Surface Area Function: 6 * side * side[cite: 1]
fn calculate_cube() {
    let mut input = String::new();
    println!("Enter side length of the cube:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let side: f64 = input.trim().parse().expect("Invalid input");

    let surface_area = 6.0 * side * side;
    println!("The surface area of the cube is: {}", surface_area);
}

// 5. Cylinder Volume Function: pi * radius * radius * height[cite: 1]
fn calculate_cylinder() {
    let mut input1 = String::new();
    println!("Enter radius of the cylinder:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let radius: f64 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter height of the cylinder:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let height: f64 = input2.trim().parse().expect("Invalid input");

    // Using schoolbook pi from Practice 3 (22.0 / 7.0)[cite: 1]
    let pi: f64 = 22.0 / 7.0;
    let volume = pi * radius * radius * height;
    println!("The volume of the cylinder is: {}", volume);
}

fn main() {
    // Menu display
    println!("   WELCOME TO THE SHAPE CALCULATOR      ");
    println!("1. Trapezium (Area)");
    println!("2. Rhombus (Area)");
    println!("3. Parallelogram (Area)");
    println!("4. Cube (Surface Area)");
    println!("5. Cylinder (Volume)");
    println!("Please enter your choice (1-5):");

    // Read user choice
    let mut choice_input = String::new();
    io::stdin().read_line(&mut choice_input).expect("Failed to read input");
    let choice: i32 = choice_input.trim().parse().expect("Invalid input");

    // Decision block using basic if/else statements
    if choice == 1 {
        calculate_trapezium();
    } else if choice == 2 {
        calculate_rhombus();
    } else if choice == 3 {
        calculate_parallelogram();
    } else if choice == 4 {
        calculate_cube();
    } else if choice == 5 {
        calculate_cylinder();
    } else {
        println!("Invalid selection! Please run the program again and choose a number from 1 to 5.");
    }
}