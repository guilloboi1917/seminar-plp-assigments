use std::io;

// Input params for fizzbuzz
struct FizzbuzzParams {
    m: u32,
    n: u32,
    x: u32,
    y: u32,
}

fn main() {
    loop {
        println!(
            "Please enter four positive integer numbers (m n x y) separated by one or more blank spaces or type quit."
        );

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input!");

        // Match parsed input to some actions
        match parse_input(input) {
            // QUIT
            Ok(None) => break,

            // VALID INPUT
            Ok(Some(params)) => {
                println!("{}", run_fizzbuzz(params.m, params.n, params.x, params.y));
                continue;
            }

            // ERROR
            Err(err_msg) => {
                println!("Err {}", err_msg);
                continue;
            }
        }
    }
}

fn run_fizzbuzz(m: u32, n: u32, x: u32, y: u32) -> String {
    let mut result = String::new();

    // Iterate over all values
    for i in m..=n {
        // Check fizzbuzz rules
        if i % (x * y) == 0 {
            result.push_str("FizzBuzz");
        } else if i % x == 0 {
            result.push_str("Fizz");
        } else if i % y == 0 {
            result.push_str("Buzz");
        } else {
            result.push_str(&i.to_string());
        }

        // Add space between elements (except last one)
        if i < n {
            result.push(' ');
        }
    }

    result
}

// Validate user input by returning a result
// Quit -> Ok(None)
// Run fizzbuzz -> Ok(params)
// Error -> Err
fn parse_input(user_input: String) -> Result<Option<FizzbuzzParams>, String> {
    // Trim input and check for quit statement
    let trimmed_input = user_input.trim();

    if trimmed_input.eq_ignore_ascii_case("quit") {
        return Ok(None);
    }

    // Collect into str-vector
    let input_list: Vec<&str> = trimmed_input.split_ascii_whitespace().collect();

    if input_list.len() != 4 {
        return Err("Please enter exactly 4 numbers".to_string());
    }

    let mut params = [0u32; 4];

    for (i, param) in input_list.iter().enumerate() {
        // Parse to a unsigned integer and will get an error if a number < 1 will be provided
        // Using map_err for more concise code, could use match result and return Ok and Err
        params[i] = param
            .parse()
            .map_err(|_| format!("'{}' is not a positive integer", param))?;

        if params[i] == 0 {
            return Err("Number must be positive (> 0)".to_string());
        }
    }

    // Get parameters
    let [m, n, x, y] = params;

    // final check on numbers
    if n <= m {
        return Err("n must be larger than m".to_string());
    }

    if x == y {
        return Err("x must be different than y".to_string());
    }

    // We return Ok with the params attached
    return Ok(Some(FizzbuzzParams { m, n, x, y }));
}
