// Define upper and lower limits for chars
// Use i32 to make calculations conistent
const LOWERLIMIT: i32 = 'a' as i32;
const UPPERLIMIT: i32 = 'z' as i32;
const MAX_ROT: i32 = 26; // exclusive

fn main() {
    let input_string = "abcde".to_string();
    let offset = -1;
    let salt = "jjh".to_string();

    let mut encode_result: String = String::new();
    let mut decode_result: String = String::new();

    println!("Input: {}", input_string);
    println!("Salt: {}", salt);

    match encode(input_string, offset, salt) {
        Ok(result) => {
            encode_result = result;
            println!("Encode Result: {}", encode_result);
        }
        Err(err_msg) => println!("{}", err_msg),
    }

    match decode(encode_result, offset) {
        Ok(result) => {
            decode_result = result;
            println!("Decode Result: {}", decode_result);
        }
        Err(err_msg) => println!("{}", err_msg),
    }
}

fn encode(input: String, offset: i32, salt: String) -> Result<String, String> {
    if input.is_empty() {
        return Err("Please provide a non-empty input".to_string());
    }

    // Salt needs to be provided (assumption for decoder)
    if salt.is_empty() {
        return Err("Please provide a salt value".to_string());
    }
    // For easier handling let's create a vec<char>
    let encode_chars: Vec<char> = input.chars().collect();
    let mut encode_salt = salt.chars().into_iter().cycle();

    // check offset (between -26 and 26)
    if offset.abs() >= MAX_ROT {
        return Err("Err: Offset must be between -26 and 26".to_string());
    }

    let mut complete_as_i32: Vec<i32> = Vec::new();

    for (i, character) in encode_chars.iter().enumerate() {
        // check if in range
        if !character.is_ascii_lowercase() {
            return Err("Err: Only lowercase ascii characters are allowed".to_string());
        }

        complete_as_i32.push(*character as i32);

        // No salt added at last position
        if i < encode_chars.len() - 1 {
            if let Some(salt_value) = encode_salt.next() {
                complete_as_i32.push(salt_value as i32);
            } else {
                return Err("Err: Salt exhausted".to_string()); // Shouldnt happen but just for sanity
            }
        }
    }

    // Apply ROT, convert to string and return
    rot_veci32_to_string(complete_as_i32, offset, UPPERLIMIT, LOWERLIMIT)
}

fn decode(input: String, offset: i32) -> Result<String, String> {
    if input.is_empty() {
        return Err("Please provide a non-empty input".to_string());
    }

    let decode_chars: Vec<char> = input.chars().collect();

    // Convert to numbers
    let mut complete_as_i32: Vec<i32> = Vec::new();

    for (i, character) in decode_chars.iter().enumerate() {
        // check if in range
        if !character.is_ascii_lowercase() {
            return Err("Err: Only lowercase ascii characters are allowed".to_string());
        }

        // Add if first character or a multiple of 2 (due to indexing)
        if i == 0 || i % 2 == 0 {
            complete_as_i32.push(*character as i32);
        }
    }

    // Apply ROT, convert to string and return
    rot_veci32_to_string(complete_as_i32, -offset, UPPERLIMIT, LOWERLIMIT)
}

fn rot_veci32_to_string(
    input_vec: Vec<i32>,
    rot_offset: i32,
    upper_limit: i32,
    lower_limit: i32,
) -> Result<String, String> {
    // Apply ROT
    let rot_shifted: Vec<i32> = input_vec
        .iter()
        .map(|x| shift_in_range(*x, rot_offset, upper_limit, lower_limit))
        .collect();

    // Convert back to string
    let result = rot_shifted
        .iter()
        .map(|&c| c as u8 as char)
        .collect::<String>();

    Ok(result)
}

// Apply number shift in specified range
fn shift_in_range(x: i32, shift: i32, upper_limit: i32, lower_limit: i32) -> i32 {
    let mut val = x;
    let number_range = upper_limit - lower_limit + 1; // Should be 26 in our case 122 - 97 + 1
    let raw_shifted = val + shift;

    if raw_shifted < lower_limit {
        // If the raw shifted value is lower than lower limit than we can simply add the number range
        val = raw_shifted + number_range;
    } else if raw_shifted > upper_limit {
        // If the raw shifted value is higher than higher limit than we can simply subtract the number range
        val = raw_shifted - number_range;
    } else {
        val = raw_shifted;
    }

    val
}
