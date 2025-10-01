use assigment01::external_command::show_sysinfo;
use assigment01::quicksort::quicksort;
use assigment01::{download_file, parse_xml};

fn main() {
    println!("----- Running quicksort examples -----\n");
    run_quicksort_examples();
    println!();

    println!("----- Running external command example -----\n");
    show_sysinfo();
    println!();

    println!("----- Running XML parsing example -----\n");
    let file = download_file("https://www.w3schools.com/xml/books.xml", "books.xml");
    parse_xml(file.unwrap());
}

fn run_quicksort_examples() {
    // integer array
    let mut integer_input_array = [11, 4, 7, 6, 10, 100, 20];
    let mut hi = integer_input_array.len() - 1;

    println!("Sorting integer input array {:?}", integer_input_array);
    quicksort(&mut integer_input_array, 0, hi);

    println!("{:?}\n", integer_input_array);

    // str array
    let mut string_input_array = ["abc", "aaa", "k", "aba", "xyz", "cool"];
    hi = string_input_array.len() - 1;

    println!("Sorting string input array {:?}", string_input_array);
    quicksort(&mut string_input_array, 0, hi);

    println!("{:?}\n", string_input_array);

    // float array
    let mut float_input_array = [0.2, 1.1, 3.3, 10.0];
    hi = float_input_array.len() - 1;

    println!("Sorting float input array {:?}", float_input_array);
    quicksort(&mut float_input_array, 0, hi);

    println!("{:?}\n", float_input_array);
}
