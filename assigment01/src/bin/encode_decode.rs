use std::io;

fn main() {
    println!(
        "Please enter four positive integer numbers (m n xy) separated by one or more blank spaces or type quit."
    );

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input!");

    println!("{}", { validate_input(input) })
}

// Validate user input
fn validate_input(user_input: String) -> bool {
    let trimmed_input = user_input.trim();

    if trimmed_input == "quit" {
        return true;
    }
    // Split string
    let input_iter = user_input.split_ascii_whitespace();

    for param in input_iter {
        let number: i32 = param.parse().expect("Not a valid input");
        if number < 0 {
            return false;
        };
    }
    return true;
}
