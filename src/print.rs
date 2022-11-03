

pub fn run() {
    // Print to console
    println!("Hello WBA Class! This is on print.rs file! Whattt?!");

    // Basic formatting
    println!("Number: {}", 1);

    println!("{} is from {}", "André", "Portugal");

    // Positional arguments
    println!("{0} is from {1} and {0} likes to {2}!", "André", "Portugal", "play RL" );

    // Named Arguments
    println!("{name} likes to play {activity}!", name = "André", activity = "RL");

    // Placeholders traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for degut trait
    println!("{:?}", (12, true, "Hi"));

    // basic math
    println!("10 + 10 = {}", 10 + 10);
}