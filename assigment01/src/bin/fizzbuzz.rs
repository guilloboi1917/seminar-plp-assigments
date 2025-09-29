use core::num;
use std::io;

fn main() {
    println!(
        "Please enter four positive integer numbers (m n x y) separated by one or more blank spaces or type quit."
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

    let input_list: Vec<&str> = input_iter.collect();

    if input_list.len() != 4 {
        return false;
    }

    let mut input_array: [i32; 4] = [0, 0, 0, 0];

    for (i, param) in input_list.iter().enumerate() {
        let number: i32 = param.parse().expect("Not a valid input");
        if number < 0 {
            return false;
        };
        input_array[i] = number;
    }

    // final check on numbers
    if input_array[1] <= input_array[0] {
        return false;
    }

    if input_array[2] == input_array[3] {
        return false;
    }

    return true;
}
