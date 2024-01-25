// Use the command `rustc main.rs` to compile me.
// Normally, Rust programs use `cargo` to manage themselves.

// Import standard library modules
use std::io;

// Define the main function
fn main() {
    // Print a welcome message
    println!("Welcome to My Rust Program!");

    // Call a function or write your code here

    // Example: Reading input from the user
    let mut input = String::new();
    println!("Please enter something:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("You entered: {}", input.trim());
}
