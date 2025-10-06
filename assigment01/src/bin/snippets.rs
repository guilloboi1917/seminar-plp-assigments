use assigment01::external_command::show_sysinfo;
use assigment01::quicksort::run_quicksort_examples;
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
    match file {
        Ok(file) => {
            parse_xml(file);
        }
        Err(err_msg) => {
            println!("Err: {err_msg}\n",)
        }
    }
}
