use std::io;

fn main() {
    println!("=== THE RESTAURANT MENU ===");
    println!("P: Poundo Yam / Edinkaiko Soup - ₦3200");
    println!("F: Fried Rice & Chicken        - ₦3000");
    println!("A: Amala & Ewedu Soup          - ₦2500");
    println!("E: Eba & Egusi Soup            - ₦2000");
    println!("W: White Rice & Stew           - ₦2500");
    println!();

    // Read food type
    println!("Enter food code (P, F, A, E, W): ");
    let mut food = String::new();
    io::stdin().read_line(&mut food).expect("Failed to read input");
    let food = food.trim().to_uppercase();

    // Read quantity
    println!("Enter quantity: ");
    let mut qty = String::new();
    io::stdin().read_line(&mut qty).expect("Failed to read input");
    let qty: i32 = qty.trim().parse().expect("Please enter a number");

    // Price decision
    let price = match food.as_str() {
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,
        _ => {
            println!("Invalid food code!");
            return;
        }
    };

    let total = price * qty;
    println!("Total before discount: ₦{}", total);

    // Discount rule
    let final_amount = if total > 10000 {
        let discount = total as f32 * 0.05;
        println!("Discount applied: ₦{}", discount);
        total as f32 - discount
    } else {
        total as f32
    };

    println!("Final amount to pay: ₦{}", final_amount);
}
