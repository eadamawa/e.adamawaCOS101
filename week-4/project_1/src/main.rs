use std::io;

fn main() {
    // Read a
    let mut a_input = String::new();
    println!("Enter value for a:");
    io::stdin().read_line(&mut a_input).expect("Failed to read input");
    let a: f64 = a_input.trim().parse().expect("Please enter a valid number");

    // Read b
    let mut b_input = String::new();
    println!("Enter value for b:");
    io::stdin().read_line(&mut b_input).expect("Failed to read input");
    let b: f64 = b_input.trim().parse().expect("Please enter a valid number");

    // Read c
    let mut c_input = String::new();
    println!("Enter value for c:");
    io::stdin().read_line(&mut c_input).expect("Failed to read input");
    let c: f64 = c_input.trim().parse().expect("Please enter a valid number");

    // Discriminant
    let d = b * b - 4.0 * a * c;

    println!("Discriminant = {}", d);

    if d > 0.0 {
        // Two distinct real roots
        let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b - d.sqrt()) / (2.0 * a);
        println!("Two distinct real roots:");
        println!("Root 1 = {}", root1);
        println!("Root 2 = {}", root2);

    } else if d == 0.0 {
        // One real root
        let root = -b / (2.0 * a);
        println!("Exactly one real root:");
        println!("Root = {}", root);

    } else {
        // No real roots
        println!("No real roots. The discriminant is negative.");
    }
}
