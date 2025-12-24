use std::io;
use chrono::Local;

fn main() {
    // input username
    println!("Please enter your name:");

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");

    // Remove any trailing newline
    let name = name.trim();

    // Get the current local time
    let current_time = Local::now();

    // Print the message
    println!("Hello {}, right now the time is {}", name, current_time.format("%Y-%m-%d %H:%M:%S"));
}

