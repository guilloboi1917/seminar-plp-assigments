use std::string;

use assigment01::quicksort::quicksort;

fn main() {
    // Some quicksort tests

    // integer array
    let mut integer_input_array = [11, 4, 7, 6, 10, 100, 20];
    let mut hi = integer_input_array.len() - 1;
    quicksort(&mut integer_input_array, 0, hi);

    println!("{:?}", integer_input_array);

    // str array
    let mut string_input_array = ["abc", "aaa", "k", "aba", "xyz", "cool"];
    hi = string_input_array.len() - 1;
    quicksort(&mut string_input_array, 0, hi);

    println!("{:?}", string_input_array);

    // str array
    let mut float_input_array = [0.2, 1.1, 3.3, 10.0];
    hi = float_input_array.len() - 1;
    quicksort(&mut float_input_array, 0, hi);

    println!("{:?}", float_input_array);
}
